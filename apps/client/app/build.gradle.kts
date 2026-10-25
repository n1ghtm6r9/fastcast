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

android {
    namespace = "dev.fastcast.client"
    compileSdk = 35

    defaultConfig {
        applicationId = "dev.fastcast.client"
        minSdk = 26
        targetSdk = 35
        versionCode = buildNumber
        versionName = "0.1.$buildNumber"
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

dependencies {
    implementation(libs.okhttp)

    testImplementation(libs.junit)
    testImplementation(libs.json)
}
