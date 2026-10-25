import javax.inject.Inject
import org.gradle.process.ExecOperations
import org.jetbrains.kotlin.gradle.dsl.JvmTarget
import org.jetbrains.kotlin.gradle.tasks.KotlinCompile

plugins {
    alias(libs.plugins.android.application)
    alias(libs.plugins.kotlin.android)
}

val buildNumber = System.getenv("FASTCAST_VERSION_CODE")?.takeIf(String::isNotBlank)?.toInt() ?: 1
val githubRepository = System.getenv("GITHUB_REPOSITORY")?.takeIf(String::isNotBlank) ?: "n1ghtm6r9/fastcast"
val signingFile = File(
    System.getenv("FASTCAST_SIGNING")?.takeIf(String::isNotBlank)
        ?: "${System.getProperty("user.home")}/.fastcast/keystore.properties",
)
val signingValues: Map<String, String> =
    if (signingFile.isFile) {
        signingFile.readLines()
            .map { it.removePrefix("\uFEFF") }
            .filter { '=' in it && !it.trimStart().startsWith("#") }
            .associate { it.substringBefore('=').trim() to it.substringAfter('=') }
    } else {
        emptyMap()
    }

fun signingValue(key: String): String =
    requireNotNull(signingValues[key]) { "$key is missing in ${signingFile.name}" }

abstract class CargoAndroidLibraries : DefaultTask() {
    @get:Inject
    abstract val execOperations: ExecOperations

    @get:Input
    abstract val ndkPath: Property<String>

    @get:Internal
    abstract val repositoryRoot: DirectoryProperty

    @get:InputFiles
    @get:PathSensitive(PathSensitivity.RELATIVE)
    abstract val sources: ConfigurableFileCollection

    @get:OutputDirectory
    abstract val outputDirectory: DirectoryProperty

    private class Target(val abi: String, val triple: String, val clangTriple: String)

    private val targets = listOf(
        Target("arm64-v8a", "aarch64-linux-android", "aarch64-linux-android26"),
        Target("armeabi-v7a", "armv7-linux-androideabi", "armv7a-linux-androideabi26"),
    )

    @TaskAction
    fun build() {
        val root = repositoryRoot.get().asFile
        val output = outputDirectory.get().asFile
        val linker = clangExecutable()
        output.deleteRecursively()
        targets.forEach { target ->
            val variablePrefix = "CARGO_TARGET_${target.triple.uppercase().replace('-', '_')}"
            execOperations.exec {
                commandLine(
                    "cargo", "build", "--locked",
                    "-p", "fastcast-native",
                    "--profile", "android",
                    "--target", target.triple,
                    "--manifest-path", root.resolve("Cargo.toml").path,
                )
                environment("${variablePrefix}_LINKER", linker.path)
                environment(
                    "${variablePrefix}_RUSTFLAGS",
                    "-C link-arg=--target=${target.clangTriple} -C link-arg=-Wl,-z,max-page-size=16384",
                )
            }
            root.resolve("target/${target.triple}/android/libfastcast_native.so")
                .copyTo(output.resolve("${target.abi}/libfastcast_native.so"), overwrite = true)
        }
    }

    private fun clangExecutable(): File {
        val osName = System.getProperty("os.name")
        val (host, extension) = when {
            osName.startsWith("Windows") -> "windows-x86_64" to ".exe"
            osName.startsWith("Mac") -> "darwin-x86_64" to ""
            else -> "linux-x86_64" to ""
        }
        return File(ndkPath.get()).resolve("toolchains/llvm/prebuilt/$host/bin/clang$extension")
    }
}

android {
    namespace = "dev.fastcast.client"
    compileSdk = 35
    ndkVersion = "27.3.13750724"

    defaultConfig {
        applicationId = "dev.fastcast.client"
        minSdk = 26
        targetSdk = 35
        versionCode = buildNumber
        versionName = "1.0.$buildNumber"
        buildConfigField(
            "String",
            "UPDATE_MANIFEST_URL",
            "\"https://github.com/$githubRepository/releases/latest/download/fastcast-tv.json\"",
        )
    }

    buildFeatures {
        buildConfig = true
    }

    signingConfigs {
        if (signingFile.isFile) {
            create("release") {
                storeFile = signingFile.absoluteFile.parentFile.resolve(signingValue("storeFile"))
                storePassword = signingValue("storePassword")
                keyAlias = signingValue("keyAlias")
                keyPassword = signingValue("keyPassword")
            }
        }
    }

    buildTypes {
        debug {
            signingConfigs.findByName("release")?.let { signingConfig = it }
        }
        release {
            isMinifyEnabled = false
            signingConfigs.findByName("release")?.let { signingConfig = it }
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }

    lint {
        abortOnError = true
        disable += "AndroidGradlePluginVersion"
    }
}

kotlin {
    compilerOptions {
        jvmTarget.set(JvmTarget.JVM_17)
    }
}

tasks.withType<KotlinCompile>().configureEach {
    if (name.endsWith("UnitTestKotlin")) compilerOptions.noJdk.set(false)
}

val cargoAndroidLibraries = tasks.register<CargoAndroidLibraries>("cargoAndroidLibraries") {
    val root = rootDir.resolve("../..").canonicalFile
    repositoryRoot.set(root)
    ndkPath.set(
        androidComponents.sdkComponents.sdkDirectory.map { it.dir("ndk/${android.ndkVersion}").asFile.absolutePath },
    )
    sources.from(
        fileTree(root.resolve("apps/client/native")) { exclude("**/target/**") },
        fileTree(root.resolve("packages/wire")) { exclude("**/target/**") },
        root.resolve("Cargo.toml"),
        root.resolve("Cargo.lock"),
    )
}

androidComponents {
    onVariants { variant ->
        variant.sources.jniLibs?.addGeneratedSourceDirectory(
            cargoAndroidLibraries,
            CargoAndroidLibraries::outputDirectory,
        )
    }
}

dependencies {
    implementation(libs.okhttp)

    testImplementation(libs.junit)
    testImplementation(libs.json)
}
