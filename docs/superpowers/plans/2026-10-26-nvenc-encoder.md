# NVENC Encoder Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build `packages/nvenc`: drive NVIDIA's hardware encoder directly — our own C layer compiled by `cc` against the vendored NVENC SDK 13.0 header, and a safe Rust `Encoder` that turns D3D11 NV12/P010 textures into H.264 or HEVC Annex B with ultra-low-latency settings, keyframes on demand and live bitrate changes.

**Architecture:** All NVENC structs are filled in C against the real `nvEncodeAPI.h`, so no struct layout is copied by hand. Rust sees four C functions with plain arguments. Settings are validated in C before the driver is touched, so validation is testable on machines without NVIDIA. Hardware tests carry `#[ignore = "..."]`.

**Tech Stack:** Rust 2024, C (MSVC via `cc` 1.10), `windows` 0.62.2 for the D3D11 types, NVENC API 13.0 (driver ≥ 570; this PC: RTX 5070, driver 621).

**Spec:** `docs/superpowers/specs/2026-10-26-native-pipeline-design.md` — sections "Server components" and "NVENC settings".

## Global Constraints

- Rust: one item per file (type, function, constant, or the `impl` block of one method), file name = item name in snake_case, module layout `constants/ services/ types/ test/` as in `packages/media/src/avc/`; one test per file `<sentence>_test.rs`, the `#[test] fn` is the sentence.
- C: one function per `.c` file named after it; shared declarations in `native/fc_internal.h`; the Rust-facing API in `native/fastcast_nvenc.h`. Compiled with warnings as errors (`/W4 /WX`). If only the vendored header triggers a warning, wrap its `#include` in `#pragma warning(push, 0)` / `#pragma warning(pop)`.
- No comments in our code (Rust, C, build script). The vendored header stays byte-for-byte as downloaded (its comments and MIT notice included).
- No new crates; `cc` and `windows` are already declared in `packages/nvenc/Cargo.toml`. You may add `windows` features.
- Every module that touches Windows APIs or the C layer is `#[cfg(windows)]`; `build.rs` compiles C only when `CARGO_CFG_TARGET_OS == "windows"`.
- Errors: `NvencError` with hand-written `Display` and `Error` (pattern: `packages/media/src/flv/types/flv_error.rs`).
- Hardware tests: `#[ignore = "needs an NVIDIA GPU"]` (or `"benchmark"`), run with `cargo nextest run -p nvenc --run-ignored all`. They only encode synthetic textures; they never touch displays.
- Commands from the worktree root, never `cd`: `cargo nextest run -p nvenc`, `cargo clippy -p nvenc --all-targets -- -D warnings`, `cargo fmt --all --check`.
- Commit after each task: one-line imperative subject, no body. Never push.

## Review Focus

1. Driver missing, too old, or GPU without the codec: a clear `NvencError`, never a crash. Tests: Task 1 validation tests and `c_status_codes_map_to_nvenc_errors` (this GPU cannot produce "codec unsupported", so the code mapping is what gets tested).
2. A texture whose size or format differs from the settings would make the GPU read out of bounds: rejected in Rust before reaching C. Test: Task 3 `a_texture_of_another_size_is_rejected`.
3. Leaks across reconnects: every open must be fully released (consumer GPUs allow few sessions). Test: Task 2 `twenty_encoders_opened_one_after_another_all_open`.
4. A bitrate change must not produce a keyframe (a 4K IDR is a latency spike). Test: Task 4 `the_bitrate_changes_without_a_keyframe`.
5. Alternating input textures (double buffering in the capture loop): each must be registered once and reused. Test: Task 3 `two_textures_can_take_turns`.

---

## File Structure

```
packages/nvenc/
  build.rs                    compiles native/*.c into the static library fastcast_nvenc
  vendor/nvEncodeAPI.h        nv-codec-headers tag n13.0.19.0, include/ffnvcodec/nvEncodeAPI.h, unchanged
  native/fastcast_nvenc.h     C API used by Rust (below)
  native/fc_internal.h        struct fc_encoder, internal prototypes
  native/*.c                  one function per file
  src/lib.rs                  re-exports (Task 5)
  src/ffi/                    #[repr(C)] mirrors and extern declarations
  src/encoder/                EncoderCodec, EncoderSettings, EncodedFrame, NvencError, Encoder
```

## C API (exact)

`native/fastcast_nvenc.h` declares, with `#include <stdint.h>` and no comments:

```c
typedef struct fc_encoder fc_encoder;
typedef struct fc_settings { uint32_t codec; uint32_t width; uint32_t height; uint32_t fps; uint32_t bitrate_kbps; uint32_t bit_depth; uint32_t preset; } fc_settings;
typedef struct fc_result { int32_t status; uint32_t call; } fc_result;
typedef struct fc_packet { const uint8_t* data; uint32_t size; uint32_t keyframe; } fc_packet;
enum { FC_CODEC_H264 = 1, FC_CODEC_HEVC = 2 };
enum { FC_LIBRARY_MISSING = -1, FC_ENTRY_POINT_MISSING = -2, FC_DRIVER_TOO_OLD = -3, FC_CODEC_UNSUPPORTED = -4, FC_INVALID_SETTINGS = -5, FC_OUT_OF_MEMORY = -6, FC_TEXTURE_LIMIT = -7 };
enum { FC_CALL_NONE = 0, FC_CALL_LOAD = 1, FC_CALL_OPEN_SESSION = 2, FC_CALL_GET_GUIDS = 3, FC_CALL_GET_CAPS = 4, FC_CALL_PRESET_CONFIG = 5, FC_CALL_INITIALIZE = 6, FC_CALL_CREATE_BITSTREAM = 7, FC_CALL_REGISTER = 8, FC_CALL_MAP = 9, FC_CALL_ENCODE = 10, FC_CALL_LOCK = 11, FC_CALL_UNLOCK = 12, FC_CALL_UNMAP = 13, FC_CALL_RECONFIGURE = 14 };
fc_result fc_encoder_open(void* d3d11_device, const fc_settings* settings, fc_encoder** encoder);
fc_result fc_encoder_encode(fc_encoder* encoder, void* d3d11_texture, uint32_t force_keyframe, fc_packet* packet);
fc_result fc_encoder_set_bitrate(fc_encoder* encoder, uint32_t bitrate_kbps);
void fc_encoder_close(fc_encoder* encoder);
```

- `status`: 0 success, > 0 the `NVENCSTATUS` returned by the driver, < 0 one of the `FC_*` codes; `call` names the step that failed (`FC_CALL_NONE` on success and for validation errors).
- `fc_packet.data` points into a buffer owned by the encoder, valid until the next encode or close.

### Task 1: Vendored header, C layer skeleton, validation

**Files:**
- Create: `packages/nvenc/vendor/nvEncodeAPI.h`, `packages/nvenc/build.rs`, `packages/nvenc/native/fastcast_nvenc.h`, `packages/nvenc/native/fc_internal.h`, `packages/nvenc/native/fc_validate_settings.c`, `fc_encoder_open.c`, `fc_encoder_set_bitrate.c`, `fc_encoder_close.c`, `packages/nvenc/src/ffi/…`, `packages/nvenc/src/encoder/…` (types only), tests.
- Modify: `packages/nvenc/src/lib.rs`.

**Interfaces — Produces:**
```rust
#[repr(C)] pub struct FcSettings { pub codec: u32, pub width: u32, pub height: u32, pub fps: u32, pub bitrate_kbps: u32, pub bit_depth: u32, pub preset: u32 }
#[repr(C)] pub struct FcResult { pub status: i32, pub call: u32 }
#[repr(C)] pub struct FcPacket { pub data: *const u8, pub size: u32, pub keyframe: u32 }
pub enum FcEncoder {}
unsafe extern "C" { pub fn fc_encoder_open(...) -> FcResult; pub fn fc_encoder_encode(...) -> FcResult; pub fn fc_encoder_set_bitrate(...) -> FcResult; pub fn fc_encoder_close(encoder: *mut FcEncoder); }
pub enum EncoderCodec { Avc, Hevc }
pub struct EncoderSettings { pub codec: EncoderCodec, pub width: u32, pub height: u32, pub fps: u32, pub bitrate_kbps: u32, pub bit_depth: u8, pub preset: u8 }
pub enum NvencError { LibraryMissing, EntryPointMissing, DriverTooOld, CodecUnsupported, InvalidSettings, OutOfMemory, TextureLimit, TextureMismatch, Status { call: &'static str, status: i32 } }
pub fn fc_settings_of(settings: &EncoderSettings) -> FcSettings;
pub fn nvenc_result(result: FcResult) -> Result<(), NvencError>;
```
- Get the header: `curl -fsSL https://raw.githubusercontent.com/FFmpeg/nv-codec-headers/n13.0.19.0/include/ffnvcodec/nvEncodeAPI.h -o packages/nvenc/vendor/nvEncodeAPI.h`; check it has `#define NVENCAPI_MAJOR_VERSION 13`. If that tag is gone, take the newest `n13.0.*` tag (`gh api repos/FFmpeg/nv-codec-headers/tags`) and say which in the report.
- `build.rs`: on Windows targets, `cc::Build` with every `native/*.c`, include dirs `native` and `vendor`, `/W4`, `warnings_into_errors(true)`, `compile("fastcast_nvenc")`; `cargo:rerun-if-changed=native` and `=vendor`.
- `fc_validate_settings` runs first in `fc_encoder_open` and `fc_encoder_set_bitrate`, before any driver call: non-null device/encoder and out pointers; codec 1 or 2; width and height even, 2..=8192; fps 1..=240; bitrate 1..=800000 kbps; bit depth 8 or 10; preset 1..=7 — else `FC_INVALID_SETTINGS`. In this task `fc_encoder_open` runs the validation and otherwise returns `FC_LIBRARY_MISSING`; Task 2 replaces that branch with the open sequence.
- `nvenc_result` maps `FC_*` codes to the matching variants and positive statuses to `Status { call: <API name>, status }` with names `"LoadLibrary"`, `"nvEncOpenEncodeSessionEx"`, `"nvEncGetEncodeGUIDs"`, `"nvEncGetEncodeCaps"`, `"nvEncGetEncodePresetConfigEx"`, `"nvEncInitializeEncoder"`, `"nvEncCreateBitstreamBuffer"`, `"nvEncRegisterResource"`, `"nvEncMapInputResource"`, `"nvEncEncodePicture"`, `"nvEncLockBitstream"`, `"nvEncUnlockBitstream"`, `"nvEncUnmapInputResource"`, `"nvEncReconfigureEncoder"`.
- `Display`: `LibraryMissing` → "nvEncodeAPI64.dll was not found; install the NVIDIA driver"; `DriverTooOld` → "the NVIDIA driver is older than NVENC API 13.0; update the driver"; `Status` → "{call} failed with NVENCSTATUS {status}"; the others a plain sentence each.

- [ ] **Step 1: Write the failing tests** (they run everywhere on Windows, no GPU needed):

```rust
#[test]
fn the_c_layer_rejects_odd_sizes_before_touching_the_driver() {
    let settings = fc_settings_of(&EncoderSettings { codec: EncoderCodec::Hevc, width: 1001, height: 1000, fps: 60, bitrate_kbps: 50_000, bit_depth: 8, preset: 4 });
    let mut encoder = std::ptr::null_mut();
    let result = unsafe { fc_encoder_open(1usize as *mut c_void, &settings, &mut encoder) };
    assert_eq!((result.status, result.call), (-5, 0));
    assert!(encoder.is_null());
}
```
Also: `the_c_layer_rejects_a_null_device` (`-5`), `the_c_layer_rejects_a_preset_outside_1_to_7` (preset 8 → `-5`), `setting_a_bitrate_on_no_encoder_is_rejected` (`fc_encoder_set_bitrate(null, 1000)` → `-5`), `closing_no_encoder_does_nothing` (`fc_encoder_close(null)` returns), `the_settings_reach_c_unchanged` (`fc_settings_of` gives codec 2 for Hevc and 1 for Avc, other fields equal), `c_status_codes_map_to_nvenc_errors` (`-1 → LibraryMissing`, `-3 → DriverTooOld`, `(8, 10) → Status { call: "nvEncEncodePicture", status: 8 }`, `(0, 0) → Ok`), `nvenc_errors_name_the_failed_call` (`Status { call: "nvEncEncodePicture", status: 8 }` formats as "nvEncEncodePicture failed with NVENCSTATUS 8").
- [ ] **Step 2: Run** `cargo nextest run -p nvenc` — expected: compile or link errors.
- [ ] **Step 3: Implement.**
- [ ] **Step 4: Run** tests and clippy — expected: pass.
- [ ] **Step 5: Commit** `Vendor the NVENC header and validate encoder settings in C`.

### Task 2: Opening and closing an encoder

**Files:** Create `native/fc_load_api.c`, `native/fc_configure.c` (and further one-function files as needed), `src/encoder/types/encoder.rs`, `encoder_open.rs`, `encoder_drop.rs`; tests.

**Interfaces — Consumes:** Task 1 items. **Produces:**
```rust
pub struct Encoder { raw: *mut FcEncoder, settings: EncoderSettings }
impl Encoder { pub fn open(device: &ID3D11Device, settings: &EncoderSettings) -> Result<Encoder, NvencError>; }
impl Drop for Encoder
unsafe impl Send for Encoder {}
```
`fc_encoder_open` after validation (every NVENC struct zeroed, its `version` set with the SDK's `*_VER` macro):
1. `LoadLibraryW(L"nvEncodeAPI64.dll")` → `FC_LIBRARY_MISSING`; `NvEncodeAPIGetMaxSupportedVersion` and `NvEncodeAPICreateInstance` through `GetProcAddress` → `FC_ENTRY_POINT_MISSING`; supported version below `(NVENCAPI_MAJOR_VERSION << 4) | NVENCAPI_MINOR_VERSION` → `FC_DRIVER_TOO_OLD`. The library handle and function list belong to the encoder (no globals).
2. `nvEncOpenEncodeSessionEx` with `NV_ENC_DEVICE_TYPE_DIRECTX` and the device.
3. The codec GUID must be among `nvEncGetEncodeGUIDs`, and for 10-bit `NV_ENC_CAPS_SUPPORT_10BIT_ENCODE` must be non-zero → else `FC_CODEC_UNSUPPORTED`.
4. `nvEncGetEncodePresetConfigEx(codec, P<preset>, NV_ENC_TUNING_INFO_ULTRA_LOW_LATENCY)`, then on the copy: `gopLength = NVENC_INFINITE_GOPLENGTH`, `frameIntervalP = 1`; rate control `NV_ENC_PARAMS_RC_CBR`, `averageBitRate = maxBitRate = kbps · 1000`, `vbvBufferSize = vbvInitialDelay = kbps · 1000 / fps`; codec config: `idrPeriod = NVENC_INFINITE_GOPLENGTH`, `repeatSPSPPS = 1`, VUI `videoSignalTypePresentFlag = 1`, `videoFormat = 5`, `videoFullRangeFlag = 0`, `colourDescriptionPresentFlag = 1`, `colourPrimaries = transferCharacteristics = colourMatrix = 1`, `chromaSampleLocationFlag = 1` with both locations 0; 10-bit sets input and output bit depth to 10 (HEVC profile Main10, H.264 profile High 10), 8-bit uses Main / High.
5. `nvEncInitializeEncoder`: encode GUID, preset GUID, ULL tuning, encode and DAR size = max encode size = settings size, `frameRateNum = fps`, `frameRateDen = 1`, `enablePTD = 1`, `enableEncodeAsync = 0`. Keep the initialize params and config inside `fc_encoder` for reconfiguration.
6. `nvEncCreateBitstreamBuffer`.
Any failure releases what was created and returns the status with its `FC_CALL_*`. `fc_encoder_close` unregisters resources, destroys the bitstream buffer and the session, frees the library and the struct; null-safe.

- [ ] **Step 1: Write the failing tests** (ignored `"needs an NVIDIA GPU"`; test helper `nvidia_device() -> ID3D11Device` picks the first DXGI adapter with vendor 0x10DE and creates a device with `D3D11_CREATE_DEVICE_BGRA_SUPPORT`; helper `settings(codec, width, height, bit_depth) -> EncoderSettings` with 60 fps, 50 000 kbps, preset 4): `an_hevc_encoder_opens_on_the_nvidia_adapter` (1920×1080), `an_h264_encoder_opens_on_the_nvidia_adapter`, `a_4k_ten_bit_hevc_encoder_opens` (3840×2160, 10-bit), `twenty_encoders_opened_one_after_another_all_open` (open and drop 20 times, each `Ok`).
- [ ] **Step 2: Run** `cargo nextest run -p nvenc --run-ignored all` — expected: compile errors.
- [ ] **Step 3: Implement.**
- [ ] **Step 4: Run** `cargo nextest run -p nvenc --run-ignored all` — expected: pass.
- [ ] **Step 5: Commit** `Open NVENC sessions with ultra-low-latency CBR settings`.

### Task 3: Encoding textures

**Files:** Create `native/fc_encoder_encode.c`, `native/fc_register_texture.c`, `src/encoder/types/encoded_frame.rs`, `encoder_encode.rs`; tests.

**Interfaces — Produces:**
```rust
pub struct EncodedFrame { pub data: Vec<u8>, pub keyframe: bool }
impl Encoder { pub fn encode(&mut self, texture: &ID3D11Texture2D, force_keyframe: bool) -> Result<EncodedFrame, NvencError>; }
```
- Rust checks `texture.GetDesc()` first: width and height equal the settings, format `DXGI_FORMAT_NV12` for 8-bit or `DXGI_FORMAT_P010` for 10-bit → else `TextureMismatch` without calling C.
- `fc_encoder_encode`: register the texture on first sight (up to 4 cached by pointer, a fifth → `FC_TEXTURE_LIMIT`; `NV_ENC_INPUT_RESOURCE_TYPE_DIRECTX`, settings size, buffer format `NV_ENC_BUFFER_FORMAT_NV12` or `NV_ENC_BUFFER_FORMAT_YUV420_10BIT`, usage `NV_ENC_INPUT_IMAGE`); `nvEncMapInputResource`; `nvEncEncodePicture` with picture struct frame, a growing input timestamp, and `NV_ENC_PIC_FLAG_FORCEIDR | NV_ENC_PIC_FLAG_OUTPUT_SPSPPS` when forced; `nvEncLockBitstream` (waiting), copy into the encoder's own growable buffer, `keyframe = pictureType == NV_ENC_PIC_TYPE_IDR`, `nvEncUnlockBitstream`, `nvEncUnmapInputResource` (also on failure paths).
- Test helpers: `nal_unit_types(annex_b: &[u8], codec: EncoderCodec) -> Vec<u8>` (split on `00 00 01` / `00 00 00 01`; H.264 type `byte & 0x1F`, HEVC `(byte >> 1) & 0x3F`), `gradient_texture(device: &ID3D11Device, width: u32, height: u32, bit_depth: u8, shift: u32) -> ID3D11Texture2D` (NV12/P010, `USAGE_DEFAULT`, `BIND_RENDER_TARGET`, initial data: luma `(x + y + shift)` pattern, chroma mid-grey).

- [ ] **Step 1: Write the failing tests** (ignored `"needs an NVIDIA GPU"`):

```rust
#[test]
#[ignore = "needs an NVIDIA GPU"]
fn the_first_hevc_frame_is_an_idr_with_parameter_sets() {
    let device = nvidia_device();
    let mut encoder = Encoder::open(&device, &settings(EncoderCodec::Hevc, 1920, 1080, 8)).unwrap();
    let frame = encoder.encode(&gradient_texture(&device, 1920, 1080, 8, 0), false).unwrap();
    let types = nal_unit_types(&frame.data, EncoderCodec::Hevc);
    assert!(frame.keyframe);
    assert!([32, 33, 34].iter().all(|kind| types.contains(kind)));
    assert!(types.contains(&19) || types.contains(&20));
}
```
Also: `the_first_h264_frame_is_an_idr_with_parameter_sets` (types contain 7, 8, 5), `the_second_frame_is_not_a_keyframe` (`keyframe == false`, no type 19/20), `a_forced_keyframe_is_an_idr_with_parameter_sets` (fourth frame forced → keyframe, types contain 32, 33, 34 and 19 or 20), `a_texture_of_another_size_is_rejected` (1280×720 texture into a 1920×1080 encoder → `TextureMismatch`), `two_textures_can_take_turns` (alternate two textures for 10 frames, all `Ok`).
- [ ] **Step 2: Run** — expected: compile errors.
- [ ] **Step 3: Implement.**
- [ ] **Step 4: Run** `cargo nextest run -p nvenc --run-ignored all` — expected: pass.
- [ ] **Step 5: Commit** `Encode D3D11 textures with NVENC`.

### Task 4: 10-bit and live bitrate

**Files:** Create `src/encoder/types/encoder_set_bitrate.rs` (methods live next to `encoder.rs`, as `flv_demuxer_next_event.rs` does in `packages/media`); complete `native/fc_encoder_set_bitrate.c`; tests.

**Interfaces — Produces:**
```rust
impl Encoder { pub fn set_bitrate(&mut self, bitrate_kbps: u32) -> Result<(), NvencError>; }
```
- `fc_encoder_set_bitrate`: validation, then `nvEncReconfigureEncoder` with the stored initialize params carrying the new `averageBitRate = maxBitRate` and `vbvBufferSize = vbvInitialDelay = kbps · 1000 / fps`, `resetEncoder = 0`, `forceIDR = 0`; on success the stored params keep the new values. Rust stores the new bitrate in its settings copy.

- [ ] **Step 1: Write the failing tests** (ignored): `ten_bit_hevc_is_main_10` (10-bit encoder, first frame; the SPS NAL (type 33) has `sps[3] & 0x1F == 2`, where `sps[0..2]` is the NAL header), `the_bitrate_changes_without_a_keyframe` (two frames, `set_bitrate(25_000)` is `Ok`, the next frame has `keyframe == false`).
- [ ] **Step 2: Run** — expected: compile errors.
- [ ] **Step 3: Implement.**
- [ ] **Step 4: Run** `cargo nextest run -p nvenc --run-ignored all` — expected: pass.
- [ ] **Step 5: Commit** `Change the NVENC bitrate live and support 10-bit HEVC`.

### Task 5: 4K benchmark, crate root, final check

**Files:** Create the benchmark test; modify `src/lib.rs`.

- [ ] **Step 1:** Write two tests, `#[ignore = "benchmark"] encoding_4k_8_bit_takes_this_long_per_preset` and `encoding_4k_10_bit_takes_this_long_per_preset`: 3840×2160 HEVC, 120 fps, 150 000 kbps, presets 1..=7; per preset 10 warm-up frames, then 60 timed frames alternating two gradient textures; print mean and worst milliseconds per preset. Each test must finish within nextest's 20-second limit (`.config/nextest.toml`).
- [ ] **Step 2:** `lib.rs` (modules `#[cfg(windows)]`) re-exports exactly:
```rust
pub use encoder::{EncodedFrame, Encoder, EncoderCodec, EncoderSettings, NvencError};
```
- [ ] **Step 3: Run** `cargo nextest run --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all --check`, `cargo nextest run -p nvenc --run-ignored all` — expected: all green; put the benchmark table in the final report.
- [ ] **Step 4: Commit** `Benchmark 4K NVENC presets and export the encoder API`.
