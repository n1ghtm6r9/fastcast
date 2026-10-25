# TV Native Library Build Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build the Rust crate `apps/client/native` (package `fastcast-native`, library `fastcast_native`) for the TV's ABIs `arm64-v8a` and `armeabi-v7a`, package `libfastcast_native.so` into the APK from Gradle, and do the same in CI — with one JNI function that proves the library links and uses `wire`.

**Architecture:** Gradle calls `cargo build` once per ABI with the NDK's `clang` as linker (no cargo-ndk), copies the libraries into a generated `jniLibs` directory and registers it through the AGP variant API, so only packaging tasks depend on the Rust build. The crate is a workspace member, so `cargo nextest run --workspace` on Windows also tests it on the host.

**Tech Stack:** Rust 2024 with targets `aarch64-linux-android` and `armv7-linux-androideabi`, Android NDK 27.3.13750724, AGP 8.7.3, Kotlin DSL, `jni-sys` 0.4.1.

**Spec:** `docs/superpowers/specs/2026-10-26-native-pipeline-design.md` — sections "TV components" and "Constraints".

## Global Constraints

- Rust code rules: one item per file, file name = item name in snake_case, module layout `constants/ services/ types/ test/` as in `packages/media/src/avc/`; one test per file `<sentence>_test.rs`; no comments anywhere (Rust, Kotlin DSL, YAML); imports at the top.
- No new crates; `jni-sys` and `wire` are already declared in `apps/client/native/Cargo.toml`. The `[profile.android]` (release with `panic = "unwind"`) already exists in the root `Cargo.toml`.
- The library is always built with `--profile android --locked`.
- Do not change Kotlin sources in this plan; nothing loads the library yet.
- Never print, log or commit the signing keystore or its password (`~/.fastcast/keystore.properties`, `FASTCAST_SIGNING`). Never install anything on the TV (`adb install` is forbidden).
- Installing dev tools is allowed: rustup targets and the NDK through `sdkmanager`.
- Gradle from the worktree, never `cd`, in Git Bash:
  ```bash
  export JAVA_HOME="/c/Users/n1ghtm6r9/AppData/Local/Programs/Java/jdk-17.0.20.1+1"
  export ANDROID_HOME="/c/Users/n1ghtm6r9/AppData/Local/Android/Sdk"
  C=<worktree>/apps/client
  "$C/gradlew" -p "$C" testDebugUnitTest lintDebug assembleRelease
  ```
  Test count: the JUnit XML under `app/build/test-results/testDebugUnitTest` (590 before this plan). Lint verdict: last line of `app/build/reports/lint-results-debug.txt` ("No issues found.").
- Rust checks: `cargo nextest run -p fastcast-native`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all --check`.
- Commit after each task: one-line imperative subject, no body. Never push.

## Review Focus

1. The TV may run 32-bit userspace on a 64-bit SoC: both ABIs must be in the APK. Check: Task 3 APK listing.
2. Developers without the NDK must still run unit tests and lint: `testDebugUnitTest` and `lintDebug` must not depend on the cargo task. Check: Task 3 `--dry-run` step.
3. Windows paths with backslashes and spaces (`C:\Users\…\Android\Sdk`): arguments go to processes as a list, never through a shell string. Check: Task 3 runs on this Windows PC.
4. Android 15+ devices with 16 KB pages: link with `max-page-size=16384`. Check: Task 3 `llvm-readelf -l` alignment step.
5. CI runners lack Android Rust targets: the client job must add them. Check: Task 4 workflow diff.

---

## File Structure

```
apps/client/native/src/lib.rs                     mod jni; pub use jni::*;
apps/client/native/src/jni/mod.rs                 module root
apps/client/native/src/jni/services/…             JNI exports, one per file
apps/client/native/src/jni/test/…                 tests
apps/client/app/build.gradle.kts                  ndkVersion, cargo task, jniLibs registration
.github/workflows/release.yml                     client job: Rust targets and cache
README.md                                         build prerequisites line
```

### Task 1: First JNI export

**Files:** Create `apps/client/native/src/jni/{mod.rs,services/mod.rs,services/java_dev_fastcast_client_stream_native_link_protocol_version.rs,test/mod.rs,test/the_jni_protocol_version_is_the_wire_version_test.rs}`; modify `apps/client/native/src/lib.rs`.

**Interfaces — Consumes:** `wire::PROTOCOL_VERSION: u8`. **Produces:**
```rust
#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_fastcast_client_stream_NativeLink_protocolVersion(_env: *mut jni_sys::JNIEnv, _class: jni_sys::jclass) -> jni_sys::jint;
```
It returns `wire::PROTOCOL_VERSION` as `jint`. The Kotlin side (wave 2) will be `object NativeLink { @JvmStatic external fun protocolVersion(): Int }` in package `dev.fastcast.client.stream`.

- [ ] **Step 1: Write the failing test**:

```rust
#[test]
fn the_jni_protocol_version_is_the_wire_version() {
    let version = Java_dev_fastcast_client_stream_NativeLink_protocolVersion(std::ptr::null_mut(), std::ptr::null_mut());
    assert_eq!(version, i32::from(wire::PROTOCOL_VERSION));
}
```
- [ ] **Step 2: Run** `cargo nextest run -p fastcast-native` — expected: compile error (missing function).
- [ ] **Step 3: Implement.**
- [ ] **Step 4: Run** the test and `cargo clippy --workspace --all-targets -- -D warnings` — expected: pass.
- [ ] **Step 5: Commit** `Export the protocol version to the TV app over JNI`.

### Task 2: Android toolchain on this PC

No repository change; it makes Task 3 runnable here.

- [ ] **Step 1:** `rustup target add aarch64-linux-android armv7-linux-androideabi`; `rustup target list --installed` shows both.
- [ ] **Step 2:** Install the NDK: `"$ANDROID_HOME/cmdline-tools/latest/bin/sdkmanager.bat" --install "ndk;27.3.13750724"` (if `cmdline-tools/latest` is missing, use the single directory under `cmdline-tools/`; accept licenses with `yes | sdkmanager.bat --licenses` only if the install asks). Check `"$ANDROID_HOME/ndk/27.3.13750724/toolchains/llvm/prebuilt/windows-x86_64/bin/clang.exe" --version`.
- [ ] **Step 3: Smoke build by hand** (proves the linker settings Task 3 will automate):
  ```bash
  NDK_BIN="$ANDROID_HOME/ndk/27.3.13750724/toolchains/llvm/prebuilt/windows-x86_64/bin"
  CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER="$NDK_BIN/clang.exe" \
  CARGO_TARGET_AARCH64_LINUX_ANDROID_RUSTFLAGS="-C link-arg=--target=aarch64-linux-android26 -C link-arg=-Wl,-z,max-page-size=16384" \
  cargo build --locked -p fastcast-native --profile android --target aarch64-linux-android --manifest-path <worktree>/Cargo.toml
  ```
  Expected: `target/aarch64-linux-android/android/libfastcast_native.so` exists.

### Task 3: Gradle builds and packages the library

**Files:** Modify `apps/client/app/build.gradle.kts`.

**Interfaces — Produces:** the APK contains `lib/arm64-v8a/libfastcast_native.so` and `lib/armeabi-v7a/libfastcast_native.so`.

Decisions:
- `android { ndkVersion = "27.3.13750724" }` (the version preinstalled on GitHub's ubuntu-latest image).
- A task class in the build script, e.g. `abstract class CargoAndroidLibraries : DefaultTask()`, with `@get:Inject abstract val execOperations: ExecOperations`; inputs: `ndkDirectory` (from `androidComponents.sdkComponents.ndkDirectory`), the repository root, the source trees `apps/client/native` and `packages/wire` (`@InputFiles @PathSensitive(RELATIVE)`, excluding `target`), the root `Cargo.toml` and `Cargo.lock`; output: `@get:OutputDirectory abstract val outputDirectory: DirectoryProperty` set to `layout.buildDirectory.dir("generated/rustJniLibs")`.
- For each ABI run `cargo build --locked -p fastcast-native --profile android --target <triple> --manifest-path <root>/Cargo.toml` with environment:
  - `CARGO_TARGET_<TRIPLE_IN_UPPER_SNAKE>_LINKER` = `<ndk>/toolchains/llvm/prebuilt/<host>/bin/clang` (`clang.exe` on Windows), host `windows-x86_64`, `linux-x86_64` or `darwin-x86_64`;
  - `CARGO_TARGET_<TRIPLE_IN_UPPER_SNAKE>_RUSTFLAGS` = `-C link-arg=--target=<clang triple>26 -C link-arg=-Wl,-z,max-page-size=16384`.
  - ABI table: `arm64-v8a` → `aarch64-linux-android` / `aarch64-linux-android26`; `armeabi-v7a` → `armv7-linux-androideabi` / `armv7a-linux-androideabi26`.
  Then copy `<root>/target/<triple>/android/libfastcast_native.so` to `<output>/<abi>/libfastcast_native.so`. The output directory is emptied first.
- Registration: `androidComponents { onVariants { variant -> variant.sources.jniLibs?.addGeneratedSourceDirectory(task, CargoAndroidLibraries::outputDirectory) } }`.

- [ ] **Step 1: Check the current APK has no native library** (the failing check): `unzip -l "$C/app/build/outputs/apk/release/app-release.apk" | grep -c libfastcast_native.so` after `assembleRelease` → expected `0`.
- [ ] **Step 2: Implement** the build script changes.
- [ ] **Step 3: Run** `"$C/gradlew" -p "$C" testDebugUnitTest lintDebug assembleRelease` — expected: BUILD SUCCESSFUL, 590 tests, lint "No issues found."
- [ ] **Step 4: Verify the APK and the libraries:**
  - `unzip -l .../app-release.apk | grep libfastcast_native.so` → exactly `lib/arm64-v8a/libfastcast_native.so` and `lib/armeabi-v7a/libfastcast_native.so`.
  - `"$NDK_BIN/llvm-nm.exe" -D --defined-only <each .so>` lists `Java_dev_fastcast_client_stream_NativeLink_protocolVersion`.
  - `"$NDK_BIN/llvm-readelf.exe" -d <each .so>` → `NEEDED` only among `libc.so`, `libm.so`, `libdl.so`.
  - `"$NDK_BIN/llvm-readelf.exe" -l <arm64 .so>` → every `LOAD` segment has `Align` `0x4000`.
  - Report each library's size.
- [ ] **Step 5: Verify unit tests and lint do not build Rust:** `"$C/gradlew" -p "$C" testDebugUnitTest lintDebug --dry-run` — expected: no `CargoAndroidLibraries` task in the list.
- [ ] **Step 6: Run** a second `assembleRelease` without changes — expected: the cargo task is `UP-TO-DATE`.
- [ ] **Step 7: Commit** `Build the TV native library from Gradle for both ABIs`.

### Task 4: CI and README

**Files:** Modify `.github/workflows/release.yml` (client job), `README.md`.

- [ ] **Step 1:** In the `client` job, before "Test, lint and build", add:
```yaml
      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: aarch64-linux-android,armv7-linux-androideabi
      - uses: Swatinem/rust-cache@v2
        with:
          shared-key: android
```
Nothing else changes in the workflow (the NDK comes with the runner image; Gradle runs cargo).
- [ ] **Step 2:** Validate the YAML parses (`python -I -c "import sys, yaml; yaml.safe_load(open(sys.argv[1]))" .github/workflows/release.yml` if PyYAML is available, otherwise review the indentation by eye) and show the diff in the report.
- [ ] **Step 3:** README, in the section that lists what the build needs (find it with `grep -n "Нужны" README.md`): add one short Russian sentence that building the TV app also needs the Rust targets `aarch64-linux-android` and `armv7-linux-androideabi` and Android NDK 27.3.13750724, and that Gradle builds the native library itself.
- [ ] **Step 4: Run** `cargo nextest run --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all --check` — expected: green.
- [ ] **Step 5: Commit** `Build the TV native library in CI`.
