# Native Capture Backend Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** A `native` capture backend in `apps/server/src/capture` — Desktop Duplication → own HLSL conversion → own NVENC layer → hub — chosen by `CAPTURE_BACKEND` (`auto` by default), with ffmpeg kept as the automatic fallback.

**Architecture:** `media` learns to turn NVENC's Annex B access units into the hub's 4-byte length-prefixed units plus the avcC/hvcC record built byte for byte the way ffmpeg builds it. The server gets a `NativeBackend` whose `run` spawns one capture thread (acquire → convert → encode → publish, synchronous, at a steady frame rate, IDR only at start and when the hub asks), and a `SwitchingBackend` that tries native first and hands the same screen to ffmpeg when native fails before its first frame.

**Tech Stack:** Rust 2024; crates `desktop` and `nvenc` (merged on master from their own plans); `media`; tokio `oneshot` for the thread result; `windows` crate `Win32_Media` for `timeBeginPeriod`.

**Spec:** `docs/superpowers/specs/2026-10-26-native-pipeline-design.md` (sections "Server components", "NVENC settings", "Testing policy", phase 2).

## Global Constraints

- Code rules (spec): one type, function or constant per file; `mod.rs` lists `mod x;` lines then `pub use x::*;` lines, alphabetical; modules split into `constants/`, `services/`, `types/`, `test/`; one test per file named `<sentence>_test.rs` whose function is the sentence; no comments; imports at the top; errors are enums with hand-written `Display` and `Error`.
- Windows-only code: `#[cfg(windows)] mod x;` with `#[cfg(not(windows))] mod x_unsupported;` beside it, as in `display`. `desktop` and `nvenc` are `cfg(windows)` dependencies of `app`.
- NVENC session (spec): ultra-low-latency tuning, preset P4, CBR with a one-frame VBV, infinite GOP, no B-frames, IDR only at start and on request, parameter sets repeated with every IDR — all already inside `nvenc`; the server passes `preset: 4`, `bit_depth: 8` (10-bit is plan H) and `VIDEO_BITRATE_KBPS`.
- The bitrate stays at `VIDEO_BITRATE_KBPS` for the whole run in this plan; `Encoder::set_bitrate` gets its caller with the phase 4 congestion controller.
- Hardware tests carry `#[ignore = "needs an NVIDIA GPU driving output 0 of the first adapter"]` and only read the current monitor. In `app` run them only as `cargo nextest run -p app --run-ignored only -E 'test(/^capture::/)'` — never `--run-ignored` on `app` without that filter: the `display` test turns the monitors off and must never be run.
- Never start the `fastcast` binary. Tests only.
- Plan E (`2026-10-26-server-udp-endpoint.md`) also edits `app/services/run.rs`, `Settings`, `settings_from`, the two settings default tests, `.env.example` and `README.md`. Whichever lands second rebases and keeps both changes.
- Use the `desktop` and `nvenc` names as merged on master; where a name below differs from master, master wins.
- Commits: one-line imperative subject, no body.
- Verification after every task: `cargo nextest run --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all --check`.

## Review Focus

1. A cancelled native run (viewer left, new request, shutdown) stops its thread, never publishes after `run` returned, and the next run never overlaps it on the same output — Task 7 `a_cancelled_native_run_lets_the_next_one_capture`, `a_cancelled_native_run_publishes_nothing_once_it_has_returned`.
2. A PC whose captured screen is on the AMD iGPU (or whose NVENC is missing) must not retry native every 5 s: `auto` hands the same screen to ffmpeg in the same run and remembers that picture — Task 5 `a_native_run_that_fails_before_its_first_frame_hands_over_to_ffmpeg`, `the_same_picture_skips_the_native_backend_after_it_failed`.
3. A still desktop keeps frames coming at about the frame rate, and the 200 Hz monitor of this PC does not push more than the frame rate — Task 6 clock tests, Task 7 `frames_come_at_about_the_frame_rate`.
4. Losing duplication access (UAC prompt, Ctrl+Alt+Del, mode change) does not end the stream: the last picture is repeated while the duplication is reopened, and a new size ends the run cleanly so the supervisor restarts it — Task 6 `reopen_step_of` tests.
5. The infinite GOP must not grow the hub's GOP cache without bound — Task 6 lowers `MAX_GOP_FRAMES` to 30; the existing cap tests pin it.

---

### Task 1: Annex B units and the avcC record

**Files:**
- Create: `packages/media/src/annexb/{mod.rs, services/mod.rs, services/annex_b_units.rs, services/length_prefixed_units.rs, test/mod.rs}`
- Create: `packages/media/src/avc/types/sample_format.rs`, `packages/media/src/avc/services/{avc_sample_format.rs, avcc_from_parameter_sets.rs}`
- Modify: `packages/media/src/lib.rs` (`mod annexb;`), the `mod.rs` files of the touched folders
- Test: `packages/media/src/annexb/test/*_test.rs`, `packages/media/src/avc/test/*_test.rs`, helper `packages/media/src/avc/test/avcc_parameter_sets.rs`

**Interfaces — Produces:**
```rust
pub fn annex_b_units(data: &[u8]) -> Vec<&[u8]>;
pub fn length_prefixed_units(units: &[&[u8]]) -> Vec<u8>;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SampleFormat { pub chroma_format_idc: u8, pub bit_depth_luma_minus8: u8, pub bit_depth_chroma_minus8: u8 }
pub fn avc_sample_format(rbsp: &[u8]) -> Option<SampleFormat>;
pub fn avcc_from_parameter_sets(sps: &[u8], pps: &[u8]) -> Option<Vec<u8>>;
```

- [ ] **Step 1: Write the failing tests**
  - `units_are_split_on_three_and_four_byte_start_codes`: `[0,0,0,1, 0x67,1,2, 0,0,1, 0x68,3, 0,0,0,1, 0x65,4,5]` → `[[0x67,1,2], [0x68,3], [0x65,4,5]]`.
  - `zero_bytes_before_a_start_code_are_not_part_of_the_unit`: `[0,0,1, 0x67,1,0,0, 0,0,1, 0x68]` → `[[0x67,1], [0x68]]`.
  - `bytes_before_the_first_start_code_are_ignored`: `[7,7, 0,0,1, 0x65,1]` → `[[0x65,1]]`.
  - `data_without_a_start_code_has_no_units`: `[0x65,1,2]` → `[]`.
  - `an_empty_unit_between_start_codes_is_skipped`: `[0,0,1, 0,0,1, 0x65]` → `[[0x65]]`.
  - `units_are_written_after_four_byte_big_endian_lengths`: `[[0x65,1], [0x41]]` → `[0,0,0,2, 0x65,1, 0,0,0,1, 0x41]`.
  - `every_real_avcc_record_is_rebuilt_byte_for_byte_from_its_parameter_sets`: for each of `real_avcc_records()`, `avcc_parameter_sets(&record.avcc)` gives `(sps, pps)` and `assert_eq!(avcc_from_parameter_sets(&sps, &pps), Some(record.avcc.clone()), "{}", record.name)`. The helper reads the SPS length at bytes 6..8, the SPS, the PPS count byte, the PPS length and the PPS.
  - `an_sps_too_short_to_hold_a_level_gives_no_record`: `avcc_from_parameter_sets(&[0x67, 0x64, 0x00], &[0x68, 0xCE])` → `None`.
  - `a_parameter_set_longer_than_a_u16_length_gives_no_record`: a real SPS with a PPS of 65 536 bytes → `None`.

- [ ] **Step 2: Run** `cargo nextest run -p media` — expected: compile errors (functions missing).

- [ ] **Step 3: Implement** in the files above.
  - `annex_b_units`: find every `00 00 01`; a unit runs from after one start code to the next start code (or the end), with its trailing `0x00` bytes cut (a NAL unit never ends in `0x00`); empty units are dropped.
  - `avc_sample_format(rbsp)` (SPS without its header byte, emulation prevention removed): `profile_idc` u8, 16 bits skipped, `seq_parameter_set_id` ue; when `HIGH_PROFILE_IDCS` holds the profile, `chroma_format_idc` ue (> 3 → `None`), one more bit when it is 3, then `bit_depth_luma_minus8` ue and `bit_depth_chroma_minus8` ue; other profiles give `(1, 0, 0)`.
  - `avcc_from_parameter_sets(sps, pps)` (NAL units with their header byte) writes exactly what ffmpeg writes: `[1, sps[1], sps[2], sps[3], 0xFF, 0xE1, sps_len u16 BE, sps…, 1, pps_len u16 BE, pps…]`, and when `HIGH_PROFILE_IDCS` holds `sps[1]` also `[0xFC | chroma_format_idc, 0xF8 | bit_depth_luma_minus8, 0xF8 | bit_depth_chroma_minus8, 0x00]` from `avc_sample_format(&remove_emulation_prevention(&sps[1..]))`. `None` when the SPS is shorter than 4 bytes, a set is longer than 65 535 bytes, or the sample format cannot be read.

- [ ] **Step 4: Run** `cargo nextest run -p media` — expected: all pass.

- [ ] **Step 5: Commit** `Split Annex B units and build avcC records from parameter sets`

### Task 2: The hvcC record

**Files:**
- Create: `packages/media/src/hevc/types/hevc_record_fields.rs`, `packages/media/src/hevc/services/{hevc_record_fields_of.rs, hvcc_from_parameter_sets.rs}`
- Modify: `packages/media/src/hevc/mod.rs` (`pub(crate) mod test;`), `types/mod.rs`, `services/mod.rs`
- Test: `packages/media/src/hevc/test/*_test.rs`, helper `packages/media/src/hevc/test/hvcc_parameter_sets.rs`

**Interfaces — Consumes:** `SampleFormat` (Task 1). **Produces:**
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HevcRecordFields { pub general_profile_tier_level: [u8; 12], pub max_sub_layers_minus1: u8, pub temporal_id_nesting: bool, pub sample_format: SampleFormat }
pub fn hevc_record_fields_of(rbsp: &[u8]) -> Option<HevcRecordFields>;
pub fn hvcc_from_parameter_sets(vps: &[u8], sps: &[u8], pps: &[u8]) -> Option<Vec<u8>>;
```

- [ ] **Step 1: Write the failing tests**
  - `every_real_hvcc_record_is_rebuilt_byte_for_byte_from_its_parameter_sets`: for each of `real_hvcc_records()`, `hvcc_parameter_sets(&record.hvcc)` gives the units of its arrays in order (types 32, 33, 34, one unit each) and `assert_eq!(hvcc_from_parameter_sets(&vps, &sps, &pps), Some(record.hvcc.clone()), "{}", record.name)`.
  - `an_sps_cut_inside_its_profile_tier_level_gives_no_record`: the first 8 bytes of a real SPS → `None`.
  - `a_parameter_set_longer_than_a_u16_length_gives_no_record`: a PPS of 65 536 bytes → `None`.

- [ ] **Step 2: Run** `cargo nextest run -p media` — expected: compile errors.

- [ ] **Step 3: Implement.**
  - `hevc_record_fields_of(rbsp)` (SPS after its 2-byte header, emulation prevention removed): `general_profile_tier_level = rbsp[1..13]`; a `BitReader` reads `sps_video_parameter_set_id` (4), `max_sub_layers_minus1` (3), `temporal_id_nesting` (1), `skip_profile_tier_level`, `sps_seq_parameter_set_id` ue, `chroma_format_idc` ue (> 3 → `None`), one bit when it is 3, width ue, height ue, `read_frame_crop`, `bit_depth_luma_minus8` ue, `bit_depth_chroma_minus8` ue.
  - `hvcc_from_parameter_sets` writes, as ffmpeg does for these streams: `0x01`; the 12 profile-tier-level bytes; `0xF0 0x00` (min spatial segmentation 0); `0xFC` (parallelism 0); `0xFC | chroma_format_idc`; `0xF8 | bit_depth_luma_minus8`; `0xF8 | bit_depth_chroma_minus8`; `0x00 0x00` (average frame rate); `((max_sub_layers_minus1 + 1) << 3) | (temporal_id_nesting << 2) | 0x03`; `0x03` arrays; then for `(32, vps)`, `(33, sps)`, `(34, pps)`: `[type, 0x00, 0x01, len u16 BE, unit…]`. `None` when the SPS cannot be read or a set is longer than 65 535 bytes.

- [ ] **Step 4: Run** `cargo nextest run -p media` — expected: all pass. A record that does not match byte for byte is a bug in the builder, not in the fixture.

- [ ] **Step 5: Commit** `Build hvcC records from parameter sets`

### Task 3: Splitting an encoded access unit

**Files:**
- Create: `packages/media/src/annexb/types/{mod.rs, access_unit.rs}`, `packages/media/src/annexb/services/split_access_unit.rs`
- Create constants: `packages/media/src/avc/constants/{pps_nal_unit_type.rs (8), access_unit_delimiter_nal_unit_type.rs (9)}`, `packages/media/src/hevc/constants/{hevc_vps_nal_unit_type.rs (32), hevc_pps_nal_unit_type.rs (34), hevc_access_unit_delimiter_nal_unit_type.rs (35)}`
- Modify: `packages/media/src/lib.rs` (`pub use annexb::{AccessUnit, split_access_unit};`)
- Test: `packages/media/src/annexb/test/*_test.rs`

**Interfaces — Consumes:** Tasks 1–2, existing `SPS_NAL_UNIT_TYPE`, `NAL_UNIT_TYPE_MASK`, `HEVC_SPS_NAL_UNIT_TYPE`, `hevc_nal_unit_type`, `VideoCodec`. **Produces:**
```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessUnit { pub record: Option<Vec<u8>>, pub payload: Vec<u8> }
pub fn split_access_unit(codec: VideoCodec, annex_b: &[u8]) -> AccessUnit;
```

- [ ] **Step 1: Write the failing tests** (Annex B built with 4-byte start codes from the real records `nvenc_high_1920x1080` and `nvenc_main_3840x2160`):
  - `an_avc_key_frame_gives_its_record_and_keeps_only_the_other_units`: AUD `[0x09, 0xF0]`, SPS, PPS, SEI `[0x06, 0x05, 0x01, 0x80]`, IDR `[0x65, 0x88, 0x84]` → `record == Some(real avcC)`, `payload == length_prefixed_units(&[SEI, IDR])`.
  - `an_hevc_key_frame_gives_its_record_and_keeps_only_the_other_units`: AUD `[0x46, 0x01, 0x50]`, VPS, SPS, PPS, IDR `[0x26, 0x01, 0xAF]` → `record == Some(real hvcC)`, `payload == length_prefixed_units(&[IDR])`.
  - `a_frame_without_parameter_sets_has_no_record`: AVC `[0x41, 0x9A]` → `record == None`, `payload == length_prefixed_units(&[[0x41, 0x9A]])`.
  - `an_sps_without_a_pps_gives_no_record`: AVC SPS + IDR → `record == None`, `payload == length_prefixed_units(&[IDR])`.

- [ ] **Step 2: Run** `cargo nextest run -p media` — expected: compile errors.

- [ ] **Step 3: Implement** `split_access_unit`: parameter sets (AVC 7, 8; HEVC 32, 33, 34) and access unit delimiters (AVC 9; HEVC 35) leave the payload; the first unit of each parameter-set type feeds `avcc_from_parameter_sets` / `hvcc_from_parameter_sets`; every other unit goes to the payload in order through `length_prefixed_units`.

- [ ] **Step 4: Run** `cargo nextest run -p media` — expected: all pass.

- [ ] **Step 5: Commit** `Split encoded access units into a record and length-prefixed units`

### Task 4: The `CAPTURE_BACKEND` setting

**Files:**
- Create: `apps/server/src/settings/types/backend_choice.rs`, `apps/server/src/settings/constants/capture_backend_setting.rs`, `apps/server/src/settings/services/{backend_choice_named.rs, read_backend_choice.rs}`
- Modify: `apps/server/src/settings/types/settings.rs` (`pub capture_backend: BackendChoice` after `video_encoder`), `services/settings_from.rs`, the `mod.rs` files, `test/nothing_set_gives_the_documented_defaults_test.rs` (`Auto`), `test/every_variable_overrides_its_default_test.rs` (`CAPTURE_BACKEND=ffmpeg` → `Ffmpeg`), `.env.example` (`CAPTURE_BACKEND=auto` after `VIDEO_ENCODER=auto`), `README.md` (settings table)
- Test: `apps/server/src/settings/test/{an_unknown_capture_backend_is_refused_naming_capture_backend_test.rs, capture_backend_names_are_read_in_any_case_test.rs}`

**Interfaces — Produces:**
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackendChoice { Auto, Native, Ffmpeg }
pub const CAPTURE_BACKEND_SETTING: TextSetting = TextSetting { variable: "CAPTURE_BACKEND", default: "auto" };
pub fn backend_choice_named(name: &str) -> Option<BackendChoice>;
pub fn read_backend_choice(setting: &TextSetting, lookup: &impl Fn(&str) -> Option<String>) -> Result<BackendChoice, SettingsError>;
```

- [ ] **Step 1: Write the failing tests**: `an_unknown_capture_backend_is_refused_naming_capture_backend` (`CAPTURE_BACKEND=gpu` → `Err(SettingsError { variable: "CAPTURE_BACKEND", expected: "one of auto, native, ffmpeg".to_string(), got: "gpu".to_string() })`), `capture_backend_names_are_read_in_any_case` (`" Native "` → `Native`, `"FFMPEG"` → `Ffmpeg`, `"Auto"` → `Auto`); update the two default tests.
- [ ] **Step 2: Run** `cargo nextest run -p app` — expected: compile errors.
- [ ] **Step 3: Implement** like `encoder_choice_named` / `read_encoder_choice`. README row after `VIDEO_ENCODER`: `` | `CAPTURE_BACKEND` | `auto` | `auto`: свой захват и NVENC, а если экран выводит не видеокарта NVIDIA или они не запустились — ffmpeg; `native`: только свой захват; `ffmpeg`: только ffmpeg | ``.
- [ ] **Step 4: Run** `cargo nextest run -p app` — expected: all pass (the example environment file test included).
- [ ] **Step 5: Commit** `Read the capture backend choice from CAPTURE_BACKEND`

### Task 5: Switching between the native and ffmpeg backends

**Files:**
- Modify: `apps/server/src/capture/types/capture_backend.rs` (`hub: &Arc<Hub>` in `run`), `services/run_while_watched.rs` (`hub: &Arc<Hub>`), `types/ffmpeg_backend_run.rs`, `test/fake_backend_run.rs`, `test/fake_run.rs` (`FailsNatively { frames: u64 }`), the tests that call `run_while_watched`
- Create: `apps/server/src/capture/types/{native_error.rs, switched_pipeline.rs, switching_backend.rs, switching_backend_run.rs}`
- Modify: `apps/server/src/capture/types/capture_error.rs` (`Native { frames: u64, cause: NativeError }`)
- Test: `apps/server/src/capture/test/*_test.rs` listed in Step 1

**Interfaces — Consumes:** `BackendChoice` (Task 4), `FakeBackend::{scripted, preparing, prepares, starts}`. **Produces:**
```rust
#[derive(Debug)]
pub enum NativeError {
    #[cfg(windows)] Desktop(desktop::DesktopError),
    #[cfg(windows)] Encoder(nvenc::NvencError),
    OutputMissing { index: u32 },
    NotNvidia { adapter: String },
    UndescribableStream,
    Spawn(std::io::Error),
    WorkerGone,
    #[cfg(not(windows))] Unsupported,
}
pub enum SwitchedPipeline<N, F> { Native { pipeline: N, request: ViewerRequest }, Ffmpeg(F) }
pub struct SwitchingBackend<N, F> { /* choice, native: Arc<N>, ffmpeg: Arc<F>, native_failed_for: Mutex<Option<ViewerRequest>> */ }
impl<N: CaptureBackend, F: CaptureBackend> SwitchingBackend<N, F> { pub fn new(choice: BackendChoice, native: Arc<N>, ffmpeg: Arc<F>) -> Self; }
impl<N: CaptureBackend, F: CaptureBackend> CaptureBackend for SwitchingBackend<N, F> { type Pipeline = SwitchedPipeline<N::Pipeline, F::Pipeline>; … }
```
`Display`: `Desktop`/`Encoder` → the inner error; `OutputMissing` → `output {index} of the first graphics adapter is not there`; `NotNvidia` → `the output is driven by {adapter}, not by an NVIDIA GPU`; `UndescribableStream` → `the encoder wrote a key frame without parameter sets the server can describe`; `Spawn` → `could not start the capture thread: {source}`; `WorkerGone` → `the capture thread stopped without a result`; `Unsupported` → `native capture needs Windows`. `Error::source` returns the inner error of `Desktop`, `Encoder`, `Spawn`. `CaptureError::Native` displays `native capture failed after {frames} frames: {cause}` and its source is `cause`. `FakeRun::FailsNatively { frames }` returns `Err(CaptureError::Native { frames, cause: NativeError::NotNvidia { adapter: "a fake adapter".to_owned() } })` at once.

Behaviour:
- `prepare`: `Ffmpeg` → `Ffmpeg(ffmpeg.prepare)`; `Native` → `Native { native.prepare?, request }`; `Auto` → straight to ffmpeg when `native_failed_for` equals the request, otherwise native, and on a native preparation error `warn!("native capture is not available ({failure}), using ffmpeg")` and ffmpeg.
- `open_screen` → `ffmpeg.open_screen`.
- `run`: `Ffmpeg` → `ffmpeg.run`. `Native` → `native.run`; in `Auto` an `Err(CaptureError::Native { frames: 0, .. })` stores the request in `native_failed_for`, logs `warn!("native capture failed before its first frame ({failure}), using ffmpeg for this picture")`, then runs `ffmpeg.prepare(request)` and `ffmpeg.run` on the same target and returns that result. Any other result is returned as it is. The lock is never held across an `.await`.

- [ ] **Step 1: Write the failing tests** (hub `Arc::new(Hub::new())`, target `capture_target()`, request `ViewerRequest::default()` unless named):
  - `the_ffmpeg_choice_never_prepares_the_native_backend`: `Ffmpeg` → pipeline `Ffmpeg(_)`, `native.prepares() == 0`.
  - `the_auto_choice_runs_the_native_backend_when_it_prepares`: native script `[EndsAfter { delay: ZERO, frames: 2 }]` → `Ok(CaptureReport { frames: 2 })`, `native.starts() == 1`, `ffmpeg.starts() == 0`.
  - `a_native_preparation_failure_falls_back_to_ffmpeg_in_auto`: native `preparing(vec![FakePreparation::Fails], …)` → pipeline `Ffmpeg(_)`, `ffmpeg.prepares() == 1`.
  - `a_native_preparation_failure_is_reported_when_native_is_chosen`: `Native` + failing preparation → `Err(_)`, `ffmpeg.prepares() == 0`.
  - `a_native_run_that_fails_before_its_first_frame_hands_over_to_ffmpeg`: native `[FailsNatively { frames: 0 }]`, ffmpeg `[EndsAfter { delay: ZERO, frames: 3 }]` → `Ok(CaptureReport { frames: 3 })`, `ffmpeg.prepares() == 1`, `ffmpeg.starts() == 1`.
  - `the_same_picture_skips_the_native_backend_after_it_failed`: after the previous scenario, `prepare(&request)` → `Ffmpeg(_)`, `native.prepares() == 1`.
  - `another_picture_tries_the_native_backend_again`: after the failure, `prepare(&ViewerRequest { mode: None, codecs: vec![VideoCodec::Hevc] })` → `Native { .. }`, `native.prepares() == 2`.
  - `a_native_run_that_fails_after_frames_is_reported_as_it_is`: native `[FailsNatively { frames: 5 }]` → `Err(CaptureError::Native { frames: 5, .. })`, `ffmpeg.starts() == 0`, and the next `prepare(&request)` → `Native { .. }`.
  - `a_native_choice_reports_a_failure_before_the_first_frame_as_it_is`: `Native`, native `[FailsNatively { frames: 0 }]` → `Err(CaptureError::Native { frames: 0, .. })`, `ffmpeg.prepares() == 0`.
- [ ] **Step 2: Run** `cargo nextest run -p app` — expected: compile errors.
- [ ] **Step 3: Implement** the trait change, the error types and `SwitchingBackend`.
- [ ] **Step 4: Run** `cargo nextest run -p app` — expected: all pass, the supervisor tests included.
- [ ] **Step 5: Commit** `Switch to ffmpeg when native capture fails before its first frame`

### Task 6: Publishing encoder output, pacing and reopening

**Files:**
- Create: `apps/server/src/capture/services/{publish_encoded_frame.rs, reopen_step_of.rs}`, `apps/server/src/capture/types/{frame_clock.rs, frame_clock_may_encode.rs, frame_clock_until_repeat.rs, frame_clock_encoded.rs, reopen_step.rs}`
- Modify: `apps/server/src/hub/constants/max_gop_frames.rs` (`30`)
- Test: `apps/server/src/capture/test/*_test.rs` listed below, helpers `annex_b_of.rs`, `hvcc_parameter_sets_of.rs`

**Interfaces — Consumes:** `media::{split_access_unit, VideoCodec}`, `stream_config_from_record`, `Hub::{set_config, publish}`, `NativeError` (Task 5). **Produces:**
```rust
pub fn publish_encoded_frame(hub: &Hub, codec: VideoCodec, fps: u32, keyframe: bool, annex_b: &[u8], timestamp_ms: u32) -> Result<(), NativeError>;
pub struct FrameClock { /* interval, next: Option<Instant> */ }
impl FrameClock {
    pub fn new(fps: u32) -> FrameClock;
    pub fn may_encode(&self, now: Instant) -> bool;
    pub fn until_repeat(&self, now: Instant) -> Duration;
    pub fn encoded(&mut self, at: Instant);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReopenStep { Retry, Resume, Restart }
pub fn reopen_step_of(expected: (u32, u32), reopened: Option<(u32, u32)>) -> ReopenStep;
```
- `publish_encoded_frame`: `split_access_unit`; a record becomes `stream_config_from_record(codec, record.into(), fps)` and `hub.set_config`; a key frame without a record, or with one that gives no config, is `Err(NativeError::UndescribableStream)` and nothing is published; then `hub.publish(keyframe, timestamp_ms, &payload)`.
- `FrameClock` is a one-frame token bucket, so a faster monitor averages exactly `fps` and a monitor at `fps` is encoded the moment each picture arrives: interval `1 s / fps`; `encoded(at)` sets `next = max(at, next) + interval` (`at + interval` the first time); `may_encode(now)` is true before the first encode and whenever `now + interval / 4 >= next`; `until_repeat(now)` is one interval before the first encode, then `next − now` (zero once passed).
- `reopen_step_of`: `None` → `Retry`, the expected size → `Resume`, another size → `Restart`.

- [ ] **Step 1: Write the failing tests** (`publish_*` use `nvenc_3840x2160_hvcc()` split by `hvcc_parameter_sets_of` and joined by `annex_b_of` with 4-byte start codes; the subscription is `hub.join()` taken first; clock tests use fps 100 and `let start = Instant::now()`):
  - `an_encoded_key_frame_describes_the_stream_before_it_is_published`: VPS, SPS, PPS, IDR `[0x26, 0x01, 0xAF, 0x08]`, fps 120, timestamp 7 → the received frame is key, `config.codec == "hvc1.1.6.L153.90"`, `(config.width, config.height, config.fps) == (3840, 2160, 120)`, `wire[1..5] == 7u32.to_le_bytes()`, `wire[5..] == [0, 0, 0, 4, 0x26, 0x01, 0xAF, 0x08]`.
  - `a_delta_frame_after_a_key_frame_shares_its_config`: then `[0x02, 0x01, 0xD0]` not key → not key, `Arc::ptr_eq` configs, `wire[5..] == [0, 0, 0, 3, 0x02, 0x01, 0xD0]`.
  - `a_delta_frame_before_any_key_frame_is_not_published`: `try_recv` is empty.
  - `a_key_frame_without_parameter_sets_is_refused`: IDR only → `Err(NativeError::UndescribableStream)`, nothing received.
  - `a_key_frame_whose_parameter_sets_cannot_be_described_is_refused`: SPS replaced by `[0x42, 0x01, 0x01]` → same.
  - `the_first_picture_may_be_encoded_at_once`; `a_picture_slightly_before_its_turn_may_be_encoded` (encoded at `start`, `start + 7.5 ms` → true); `a_picture_arriving_at_twice_the_frame_rate_waits_for_its_turn` (`start + 5 ms` → false); `an_early_picture_does_not_move_the_next_turn_earlier` (encoded at `start` and `start + 8 ms`, `until_repeat(start + 8 ms) == 12 ms`); `a_late_picture_starts_a_new_turn_from_itself` (encoded at `start` and `start + 25 ms`, `until_repeat(start + 25 ms) == 10 ms`); `the_last_picture_is_repeated_one_interval_after_it_was_encoded` (`until_repeat(start + 4 ms) == 6 ms`, `until_repeat(start + 12 ms) == ZERO`); `before_any_picture_the_wait_is_one_interval` (10 ms).
  - `a_reopened_duplication_of_the_same_size_resumes_the_capture`, `a_reopened_duplication_of_another_size_restarts_the_capture`, `a_duplication_that_cannot_be_reopened_yet_is_retried`.
- [ ] **Step 2: Run** `cargo nextest run -p app` — expected: compile errors.
- [ ] **Step 3: Implement** the files above and set `MAX_GOP_FRAMES` to 30.
- [ ] **Step 4: Run** `cargo nextest run -p app` — expected: all pass, `a_gop_at_the_cap_is_still_kept` and `a_gop_past_the_cap_is_dropped_until_the_next_key_frame` included.
- [ ] **Step 5: Commit** `Publish encoder output with its record and pace native frames`

### Task 7: The native backend

**Files:**
- Modify: `packages/desktop` `OutputInfo` (`pub adapter_index: u32, pub output_index: u32`, the `EnumAdapters1` / `EnumOutputs` indices, set in `list_outputs`)
- Create (all platforms): `apps/server/src/capture/types/{native_backend.rs, native_backend_run.rs, native_pipeline.rs, stop_on_drop.rs, stop_on_drop_drop.rs}`, `apps/server/src/capture/constants/native_encoder_preset.rs` (`4`)
- Create (Windows): `apps/server/src/capture/services/{run_native_capture.rs, native_output_for.rs, encoder_codec_of.rs}`, `apps/server/src/capture/types/{timer_resolution.rs, timer_resolution_drop.rs}`; (other platforms) `services/run_native_capture_unsupported.rs`
- Modify: `apps/server/Cargo.toml` (windows feature `Win32_Media`)
- Test: `packages/desktop` ignored `the_outputs_of_each_adapter_are_numbered_from_zero`; `apps/server/src/capture/test/*_test.rs` listed below

**Interfaces — Consumes:** `desktop::{list_outputs, OutputInfo, GpuDevice, Duplication, DesktopError, Converter, BitDepth, NVIDIA_VENDOR_ID}`, `nvenc::{Encoder, EncoderSettings, EncoderCodec}`, Tasks 5–6. **Produces:**
```rust
pub struct NativeBackend { /* settings: Settings, worker: Mutex<Option<JoinHandle<()>>> */ }
impl NativeBackend { pub fn new(settings: Settings) -> Self; }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NativePipeline { pub codec: VideoCodec }
pub struct StopOnDrop(pub Arc<AtomicBool>);
pub fn run_native_capture(hub: &Hub, codec: VideoCodec, target: &CaptureTarget, bitrate_kbps: u32, stop: &AtomicBool) -> Result<CaptureReport, CaptureError>;
pub fn native_output_for<'a>(target: &CaptureTarget, outputs: &'a [OutputInfo]) -> Option<&'a OutputInfo>;
pub fn encoder_codec_of(codec: VideoCodec) -> EncoderCodec;
pub struct TimerResolution;
impl TimerResolution { pub fn one_millisecond() -> TimerResolution; }
```
- `prepare` → `NativePipeline { codec: Hevc }` when `request.codecs` holds HEVC, else `Avc`. `open_screen` → `open_screen(&self.settings, request)`.
- `run`: `info!("starting native capture of output {} with NVENC {:?} at {} fps, {} kbps", …)`; a `StopOnDrop` guard on a fresh stop flag; take the previous worker handle out of the mutex; spawn a thread named `fastcast-capture` (spawn error → `Native { frames: 0, cause: Spawn }`) that first joins the previous worker, then sends `run_native_capture(…)` through a `tokio::sync::oneshot`; store the new handle; await the result (a dropped sender → `Native { frames: 0, cause: WorkerGone }`).
- `native_output_for`: the output with `adapter_index == 0` and `output_index == target.output` (the numbering `CaptureTarget` shares with ddagrab and `find_output_index`).
- `run_native_capture`, every error carrying the frames published so far:
  1. `TimerResolution::one_millisecond()`; `list_outputs`; `native_output_for` or `OutputMissing { index }`; a vendor other than `NVIDIA_VENDOR_ID` → `NotNvidia { adapter: adapter_name }`.
  2. `GpuDevice::new(adapter_luid)`, `Duplication::new`, its size, `Converter::new(…, BitDepth::Eight)`, `Encoder::open(device.device(), &EncoderSettings { codec: encoder_codec_of(codec), width, height, fps: target.fps, bitrate_kbps, bit_depth: 8, preset: NATIVE_ENCODER_PRESET })`.
  3. `FrameClock::new(target.fps)`, start instant, no picture yet. Until `stop` is set:
     - without a duplication: `Duplication::new` and `reopen_step_of`; `Restart` ends the run with `Ok`; `Retry` sleeps `until_repeat` and repeats the last picture when there is one; `Resume` keeps the new duplication.
     - `acquire(&device, until_repeat(now))`: an update converts `frame()` with `pointer()` and marks a picture; it is encoded only when `may_encode`. A timeout repeats the last picture when there is one. `DesktopError::AccessLost` drops the duplication. Other errors end the run.
     - encoding: `force = hub.take_key_frame_request()`, `encoder.encode(converter.output(), force)`, then, unless `stop` is set, `publish_encoded_frame(hub, codec, target.fps, keyframe, &data, elapsed ms as u32)`, `clock.encoded(now)`, one more frame. The converter and the encoder share `device` on this one thread, so D3D11 orders the conversion before NVENC reads the texture and no extra `Flush` is needed.
  4. `Ok(CaptureReport { frames })`.
- The unsupported variant returns `Err(Native { frames: 0, cause: Unsupported })`.

- [ ] **Step 1: Write the failing tests**
  - `the_native_backend_prefers_hevc_when_the_tv_decodes_it` (codecs `[Avc, Hevc]` → `Hevc`), `the_native_backend_uses_avc_when_the_tv_lists_no_hevc` (`[]` → `Avc`).
  - Windows: `the_native_output_is_the_numbered_output_of_the_first_adapter` (outputs adapter 0 / output 0 `\\.\DISPLAY1`, adapter 0 / output 1 `\\.\DISPLAY2`, adapter 1 / output 0 `\\.\DISPLAY3`; target output 1 → `\\.\DISPLAY2`), `an_output_number_past_the_first_adapter_is_missing` (target output 2 → `None`).
  - Ignored hardware tests (`NativeBackend::new(default_settings())`, `NativePipeline { codec: Hevc }`, `CaptureTarget { output: 0, fps: 60 }`, the run raced in `tokio::select!` against the subscription, each wait bounded by a 5 s `tokio::time::timeout`): `the_native_backend_starts_with_a_described_key_frame` (first frame key, `config.codec` starts with `"hvc1."`, width and height above 0); `a_key_frame_request_is_answered_with_a_key_frame` (after the first frame `hub.request_key_frame()`, a key frame within the next 5); `frames_come_at_about_the_frame_rate` (50 to 62 frames in the second after the first, whatever the monitor's refresh rate); `a_cancelled_native_run_lets_the_next_one_capture` (drop the first run after its first frame, a second run delivers a frame); `a_cancelled_native_run_publishes_nothing_once_it_has_returned` (after the first frame `hub.request_key_frame()`, drop the run, `hub.reset()`, sleep 200 ms, `hub.join().config.is_none()`).
- [ ] **Step 2: Run** `cargo nextest run -p app` — expected: compile errors.
- [ ] **Step 3: Implement** the files above.
- [ ] **Step 4: Run** `cargo nextest run --workspace` (all pass), then on this PC `cargo nextest run -p desktop --run-ignored all` and `cargo nextest run -p app --run-ignored only -E 'test(/^capture::/)'` — expected: the five hardware tests pass.
- [ ] **Step 5: Commit** `Capture natively with Desktop Duplication and NVENC`

### Task 8: Wiring and docs

**Files:**
- Modify: `apps/server/src/app/services/run.rs`, `apps/server/src/capture/services/supervise_capture.rs`, `README.md`
- Modify: `docs/superpowers/specs/2026-10-26-native-pipeline-design.md` only if the code had to differ from it

**Interfaces — Consumes:** `SwitchingBackend`, `NativeBackend`, `FfmpegBackend`, `Settings::capture_backend`.

- [ ] **Step 1: Wire** `run.rs`: `Arc::new(SwitchingBackend::new(settings.capture_backend, Arc::new(NativeBackend::new(settings.clone())), Arc::new(FfmpegBackend::new(settings))))` replaces the `FfmpegBackend`. The supervisor's restart line becomes `warn!("the capture stopped after {} frames, restarting", report.frames)`.
- [ ] **Step 2: Docs** in `README.md`:
  - Item 2 of the pipeline list becomes: «По умолчанию (`CAPTURE_BACKEND=auto`) сервер захватывает экран сам: Desktop Duplication → свой шейдер (курсор и перевод в YUV по BT.709 в ограниченном диапазоне) → NVENC, без ffmpeg и без копий через процессор. Ключевой кадр кодируется в начале и тогда, когда его просит ТВ. Если экран выводит не видеокарта NVIDIA или NVENC не запустился, сервер пишет причину и для этой картинки берёт ffmpeg с фильтром `ddagrab`.»
  - Item 1: «только после того, как найден ffmpeg и проверен кодировщик (п. 3)» becomes «только после того, как выбран способ захвата (п. 2)».
  - The `KEYFRAME_INTERVAL_SECONDS` row ends with «; только для ffmpeg».
- [ ] **Step 3: Verify**: `cargo nextest run --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all --check` — expected: all clean; then the Task 7 hardware command once more.
- [ ] **Step 4: Commit** `Capture natively by default and fall back to ffmpeg`
