# Desktop Capture Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build `packages/desktop`: list the PC's outputs, open a D3D11 device on an output's adapter, capture it with DXGI Desktop Duplication into a texture we own (with pointer position and shape), and convert it on the GPU with our own HLSL into NV12 (8-bit) or P010 (10-bit) BT.709 limited range with left-sited chroma and the cursor drawn in.

**Architecture:** The colour and cursor maths are pure Rust with exact unit tests; they are the reference the GPU output is compared against. GPU code uses the `windows` crate (D3D11, DXGI). HLSL is compiled at run time with `D3DCompile`. Hardware tests carry `#[ignore = "..."]`; they only read the screen. Nothing in this crate changes display settings.

**Tech Stack:** Rust 2024, `windows` 0.62.2 (already in `packages/desktop/Cargo.toml`), HLSL shader model 5.0, `cargo nextest`.

**Spec:** `docs/superpowers/specs/2026-10-26-native-pipeline-design.md` — sections "Server components" and "Colour conversion".

## Global Constraints

- One item per file (type, function, constant, or the `impl` block of one method), file name = item name in snake_case; feature module layout `constants/ services/ types/ test/` as in `packages/media/src/avc/`. `mod.rs`: `mod x;` lines then `pub use x::*;` lines, alphabetical; `#[cfg(test)] pub(crate) mod test;`.
- One test per file `<sentence>_test.rs`; the `#[test] fn` is the sentence. Helpers are their own files in `test/`.
- No comments anywhere — Rust, tests, HLSL. Imports at the top.
- No new crates. You may add `windows` features in `packages/desktop/Cargo.toml` (for example `Win32_UI_HiDpi`, `Win32_System_Threading`); that does not change `Cargo.lock`.
- The `colour` and `cursor` modules compile on every platform; every module that touches Windows APIs is declared with `#[cfg(windows)]` in `lib.rs`.
- Errors: `DesktopError` with hand-written `Display` and `Error` (pattern: `packages/media/src/flv/types/flv_error.rs`).
- Hardware tests: `#[ignore = "needs a GPU and a desktop"]` (or `"benchmark"`). Run them with `cargo nextest run -p desktop --run-ignored only`. They may read the screen; they must never change display modes, turn monitors off, or touch the Parsec virtual display. Never run the server's `display` tests.
- Commands from the worktree root, never `cd`: `cargo nextest run -p desktop`, `cargo clippy -p desktop --all-targets -- -D warnings`, `cargo fmt --all --check`.
- Commit after each task: one-line imperative subject, no body. Never push.

## Review Focus

1. Pointer partly off the output (negative position, or past the right/bottom edge): must be clipped, never read or written out of bounds. Tests: Task 2 `a_cursor_off_the_left_edge_is_clipped`, Task 5 `an_inverting_cursor_is_composited_like_the_reference` (cursor at x = −3).
2. Lock screen, UAC prompt, resolution change: `AcquireNextFrame` fails with `DXGI_ERROR_ACCESS_LOST`; we must return `DesktopError::AccessLost` (not panic) so the caller can recreate the duplication. Test: Task 4 `access_lost_is_reported_as_its_own_error` (maps the HRESULT through the same function `acquire` uses).
3. 10-bit output: P010 keeps the 10-bit code in the top 10 bits of each 16-bit value. Test: Task 5 `the_gpu_matches_the_reference_for_10_bit`.
4. A static desktop: `AcquireNextFrame` times out; `acquire` returns `Ok(None)` and the previous frame stays usable. Test: Task 4 `a_timeout_keeps_the_previous_frame`.
5. Odd sizes and rotated outputs cannot be encoded as 4:2:0 here: `OddSize` / `Rotated` errors instead of garbage. Test: Task 5 `odd_sizes_are_rejected`.

---

## File Structure

```
packages/desktop/src/
  lib.rs           re-exports (Task 6)
  colour/          ycbcr_of, reference_planes, BitDepth, YCbCr, Planes          (pure)  Task 1
  cursor/          CursorImage, CursorKind, decode_cursor_shape, composite_cursor (pure) Task 2
  gpu/             DesktopError, AdapterLuid, OutputInfo, list_outputs, GpuDevice        Task 3
  duplication/     Duplication, PointerState, FrameUpdate                                Task 4
  converter/       Converter, read_planes, shaders/*.hlsl                                Task 5
```

### Task 1: Colour reference

**Files:** Create `packages/desktop/src/colour/…`; modify `lib.rs`.

**Interfaces — Produces:**
```rust
pub enum BitDepth { Eight, Ten }
pub struct YCbCr { pub y: u16, pub cb: u16, pub cr: u16 }
pub struct Planes { pub width: usize, pub height: usize, pub luma: Vec<u16>, pub chroma: Vec<u16> }
pub fn ycbcr_of(r: f32, g: f32, b: f32, depth: BitDepth) -> YCbCr;
pub fn reference_planes(bgra: &[u8], width: usize, height: usize, depth: BitDepth) -> Planes;
```
- Inputs of `ycbcr_of` are R'G'B' in 0..=1. `Y' = 0.2126 R' + 0.7152 G' + 0.0722 B'`, `Pb = (B' − Y') / 1.8556`, `Pr = (R' − Y') / 1.5748`; 8-bit `16 + 219 Y'`, `128 + 224 Pb`, `128 + 224 Pr`; 10-bit `64 + 876 Y'`, `512 + 896 Pb`, `512 + 896 Pr`; round half up.
- `Planes.luma`: one value per pixel, row-major. `Planes.chroma`: `(width / 2) · (height / 2)` interleaved `Cb, Cr` pairs. Chroma `(x, y)`: horizontal taps `0.25 · p(2x − 1) + 0.5 · p(2x) + 0.25 · p(2x + 1)` with columns clamped to the image, applied to R'G'B' (`byte / 255`) on rows `2y` and `2y + 1`, the two rows averaged, then `ycbcr_of`.
- All types derive `Clone, Copy (where possible), Debug, PartialEq, Eq`.

- [ ] **Step 1: Write the failing tests**:

```rust
#[test]
fn bt709_primaries_map_to_their_8_bit_codes() {
    assert_eq!(ycbcr_of(1.0, 0.0, 0.0, BitDepth::Eight), YCbCr { y: 63, cb: 102, cr: 240 });
    assert_eq!(ycbcr_of(0.0, 1.0, 0.0, BitDepth::Eight), YCbCr { y: 173, cb: 42, cr: 26 });
    assert_eq!(ycbcr_of(0.0, 0.0, 1.0, BitDepth::Eight), YCbCr { y: 32, cb: 240, cr: 118 });
}

#[test]
fn the_chroma_filter_weights_the_left_column_pair_1_2_1() {
    let red = [0, 0, 255, 255];
    let blue = [255, 0, 0, 255];
    let row = [red, blue, blue, blue].concat();
    let planes = reference_planes(&[row.clone(), row].concat(), 4, 2, BitDepth::Eight);
    let left = ycbcr_of(0.75, 0.0, 0.25, BitDepth::Eight);
    let right = ycbcr_of(0.0, 0.0, 1.0, BitDepth::Eight);
    assert_eq!(planes.chroma, [left.cb, left.cr, right.cb, right.cr]);
}
```
Also: `white_is_235_128_128_in_8_bit`, `black_is_16_128_128_in_8_bit`, `ten_bit_white_and_red_use_the_ten_bit_ranges` (white `(940, 512, 512)`, red `(250, 409, 960)`), `rows_are_averaged_for_chroma` (2×2, top row white, bottom row black → chroma `[128, 128]`), `luma_is_computed_per_pixel` (same image → luma `[235, 235, 16, 16]`).
- [ ] **Step 2: Run** `cargo nextest run -p desktop` — expected: compile errors.
- [ ] **Step 3: Implement.**
- [ ] **Step 4: Run** tests and clippy — expected: pass.
- [ ] **Step 5: Commit** `Add the BT.709 colour reference`.

### Task 2: Cursor shapes

**Files:** Create `packages/desktop/src/cursor/…`; modify `lib.rs`.

**Interfaces — Produces:**
```rust
pub const POINTER_SHAPE_MONOCHROME: u32 = 1;
pub const POINTER_SHAPE_COLOR: u32 = 2;
pub const POINTER_SHAPE_MASKED_COLOR: u32 = 4;
pub enum CursorKind { Alpha, Masked }
pub struct CursorImage { pub width: u32, pub height: u32, pub kind: CursorKind, pub pixels: Vec<u8> }
pub fn decode_cursor_shape(shape_type: u32, width: u32, height: u32, pitch: u32, buffer: &[u8]) -> Option<CursorImage>;
pub fn composite_cursor(bgra: &mut [u8], width: usize, height: usize, cursor: &CursorImage, left: i32, top: i32);
```
- `pixels`: BGRA, `4 · width` bytes per row, no padding. Arguments are the fields of `DXGI_OUTDUPL_POINTER_SHAPE_INFO` and the shape buffer.
- `Masked` pixel: B, G, R hold the XOR value; A is 0xFF when the screen is kept under the mask, 0 when replaced. Result: `(screen AND (A == 0xFF ? 0xFFFFFF : 0)) XOR BGR`; the screen's own alpha byte is kept.
- Monochrome: the shape height is twice the cursor height; the first half is the AND mask, the second the XOR mask, 1 bit per pixel, most significant bit first, `pitch` bytes per row. (AND, XOR) → (A, BGR): (0, 0) → (0, black); (0, 1) → (0, white); (1, 0) → (0xFF, black); (1, 1) → (0xFF, white). Kind `Masked`.
- Masked colour: alpha byte 0 → (0, BGR), 0xFF → (0xFF, BGR). Kind `Masked`.
- Colour: straight-alpha BGRA, kind `Alpha`; `composite_cursor` blends `round((c · a + s · (255 − a)) / 255)` per channel.
- Unknown type, zero size, or a buffer shorter than `pitch · height` → `None`. `composite_cursor` clips to the image.

- [ ] **Step 1: Write the failing tests**:

```rust
#[test]
fn a_monochrome_cursor_reads_and_then_xor_rows() {
    let cursor = decode_cursor_shape(POINTER_SHAPE_MONOCHROME, 8, 2, 1, &[0b0011_1111, 0b0101_0000]).unwrap();
    assert_eq!((cursor.width, cursor.height, cursor.kind), (8, 1, CursorKind::Masked));
    let kept = [0, 0, 0, 255];
    assert_eq!(cursor.pixels, [[0, 0, 0, 0], [255, 255, 255, 0], kept, [255, 255, 255, 255], kept, kept, kept, kept].concat());
}

#[test]
fn an_inverting_cursor_flips_the_screen_under_it() {
    let mut screen = vec![10, 20, 30, 255, 200, 100, 50, 255];
    let cursor = CursorImage { width: 1, height: 1, kind: CursorKind::Masked, pixels: vec![255, 255, 255, 255] };
    composite_cursor(&mut screen, 2, 1, &cursor, 1, 0);
    assert_eq!(screen, [10, 20, 30, 255, 55, 155, 205, 255]);
}
```
Also: `a_masked_colour_cursor_keeps_its_mask_in_alpha` (2×1, pitch 8, `[10, 20, 30, 0, 40, 50, 60, 255]` → kind `Masked`, same pixels), `a_colour_cursor_keeps_its_alpha` (1×1 `[1, 2, 3, 128]` → kind `Alpha`), `rows_are_read_with_the_shape_pitch` (colour 1×2, pitch 8, `[1, 2, 3, 4, 9, 9, 9, 9, 5, 6, 7, 8, 9, 9, 9, 9]` → pixels `[1, 2, 3, 4, 5, 6, 7, 8]`), `an_unknown_shape_type_gives_no_cursor` (type 3), `a_short_buffer_gives_no_cursor`, `a_half_transparent_cursor_blends_with_the_screen` (screen `[0, 0, 0, 255]`, Alpha `[255, 255, 255, 128]` → `[128, 128, 128, 255]`), `a_cursor_off_the_left_edge_is_clipped` (opaque 2×1 Alpha cursor `[1, 1, 1, 255, 2, 2, 2, 255]` at left −1 on a 2×1 black screen → `[2, 2, 2, 255, 0, 0, 0, 255]`).
- [ ] **Step 2: Run** — expected: compile errors.
- [ ] **Step 3: Implement.**
- [ ] **Step 4: Run** — expected: pass.
- [ ] **Step 5: Commit** `Decode pointer shapes and composite them on the CPU`.

### Task 3: Outputs and GPU device

**Files:** Create `packages/desktop/src/gpu/…`; modify `lib.rs`.

**Interfaces — Produces:**
```rust
pub const NVIDIA_VENDOR_ID: u32 = 0x10DE;
pub struct AdapterLuid { pub low: u32, pub high: i32 }
pub struct OutputInfo { pub device_name: String, pub adapter_name: String, pub adapter_luid: AdapterLuid, pub vendor_id: u32, pub left: i32, pub top: i32, pub width: u32, pub height: u32, pub attached: bool }
pub enum DesktopError { Api { call: &'static str, source: windows::core::Error }, AccessLost, AdapterNotFound, OddSize { width: u32, height: u32 }, Rotated, ShaderCompile(String) }
pub fn list_outputs() -> Result<Vec<OutputInfo>, DesktopError>;
pub struct GpuDevice { /* device, context, vendor_id */ }
impl GpuDevice {
    pub fn new(adapter_luid: AdapterLuid) -> Result<GpuDevice, DesktopError>;
    pub fn device(&self) -> &ID3D11Device;
    pub fn context(&self) -> &ID3D11DeviceContext;
    pub fn vendor_id(&self) -> u32;
}
```
- `list_outputs`: `CreateDXGIFactory1` → `EnumAdapters1` → `EnumOutputs` → `GetDesc` (device name such as `\\.\DISPLAY1`, desktop coordinates, `AttachedToDesktop`); adapter name and vendor from `GetDesc1`.
- `GpuDevice::new`: find the adapter by LUID; `D3D11CreateDevice(adapter, D3D_DRIVER_TYPE_UNKNOWN, D3D11_CREATE_DEVICE_BGRA_SUPPORT, [11_1, 11_0])`; `ID3D11Multithread::SetMultithreadProtected(true)`. No adapter with that LUID → `AdapterNotFound`.
- `DesktopError` `Display`: `Api` → `"{call} failed: {source}"`; the others a plain sentence each. `Error::source` returns the windows error for `Api`. Every failing Windows call is wrapped with its own name (`"IDXGIFactory1::EnumAdapters1"`, `"D3D11CreateDevice"`, …).
- `AdapterLuid` and `OutputInfo` derive `Clone, Debug, PartialEq, Eq` (`AdapterLuid` also `Copy`).

- [ ] **Step 1: Write the failing tests**: `desktop_errors_name_the_failed_call` (`DesktopError::Api { call: "D3D11CreateDevice", source: windows::core::Error::from(E_FAIL) }` formats with `"D3D11CreateDevice failed"` in it); ignored: `the_desktop_has_an_attached_output` (some output with `attached`, `width > 0`, `height > 0`), `a_device_opens_on_the_adapter_of_an_output` (`GpuDevice::new(output.adapter_luid)` is `Ok` and `vendor_id()` equals the output's).
- [ ] **Step 2: Run** `cargo nextest run -p desktop --run-ignored all` — expected: compile errors.
- [ ] **Step 3: Implement.**
- [ ] **Step 4: Run** `cargo nextest run -p desktop --run-ignored all` — expected: pass on this PC.
- [ ] **Step 5: Commit** `List outputs and open a D3D11 device on their adapter`.

### Task 4: Desktop Duplication

**Files:** Create `packages/desktop/src/duplication/…`; modify `lib.rs`.

**Interfaces — Consumes:** `GpuDevice`, `OutputInfo`, `decode_cursor_shape`, `CursorImage`. **Produces:**
```rust
pub struct PointerState { pub visible: bool, pub left: i32, pub top: i32, pub shape: Option<CursorImage>, pub shape_generation: u64 }
pub struct FrameUpdate { pub desktop: bool, pub pointer: bool }
pub struct Duplication { /* IDXGIOutputDuplication, owned frame texture, pointer, size */ }
impl Duplication {
    pub fn new(device: &GpuDevice, output: &OutputInfo) -> Result<Duplication, DesktopError>;
    pub fn width(&self) -> u32;
    pub fn height(&self) -> u32;
    pub fn acquire(&mut self, device: &GpuDevice, timeout: Duration) -> Result<Option<FrameUpdate>, DesktopError>;
    pub fn frame(&self) -> &ID3D11Texture2D;
    pub fn pointer(&self) -> &PointerState;
}
pub fn duplication_error(call: &'static str, error: windows::core::Error) -> DesktopError;
```
- `new`: find the `IDXGIOutput` whose device name matches `output.device_name` on the device's adapter; try `IDXGIOutput5::DuplicateOutput1` with `[DXGI_FORMAT_B8G8R8A8_UNORM]`, and on failure `IDXGIOutput1::DuplicateOutput`. Size from `DXGI_OUTDUPL_DESC.ModeDesc`; odd → `OddSize`; rotation other than identity/unspecified → `Rotated`. Create the owned frame texture: same size, `B8G8R8A8_UNORM`, `USAGE_DEFAULT`, `BIND_SHADER_RESOURCE`.
- `acquire`: `AcquireNextFrame(timeout)`; `DXGI_ERROR_WAIT_TIMEOUT` → `Ok(None)`. When `LastPresentTime != 0`, `CopyResource` into the owned texture (`desktop = true`). When `LastMouseUpdateTime != 0`, update `visible`, `left`, `top` (`pointer = true`). When `PointerShapeBufferSize > 0`, `GetFramePointerShape` → `decode_cursor_shape` → `shape`, `shape_generation += 1`. Always `ReleaseFrame` before returning, errors included.
- `duplication_error` turns `DXGI_ERROR_ACCESS_LOST` into `AccessLost` and anything else into `Api { call, source }`; `acquire` uses it for every failure.
- `PointerState` derives `Clone, Debug, Default, PartialEq, Eq`; `FrameUpdate` derives `Clone, Copy, Debug, PartialEq, Eq`.

- [ ] **Step 1: Write the failing tests**: `access_lost_is_reported_as_its_own_error` (`duplication_error("x", DXGI_ERROR_ACCESS_LOST.into())` matches `AccessLost`); ignored: `the_first_acquire_returns_the_desktop_image` (first attached output; `acquire(1 s)` → `Some(update)` with `update.desktop`; the frame texture's `GetDesc` size equals the output's), `a_captured_frame_has_visible_content` (copy to a staging texture, map, not every byte is 0), `a_timeout_keeps_the_previous_frame` (after the first acquire, keep calling `acquire(10 ms)` until one returns `None` or 50 calls pass; the frame texture is still the same object and still has content).
- [ ] **Step 2: Run** — expected: compile errors.
- [ ] **Step 3: Implement.**
- [ ] **Step 4: Run** `cargo nextest run -p desktop --run-ignored all` — expected: pass.
- [ ] **Step 5: Commit** `Capture outputs with Desktop Duplication`.

### Task 5: GPU colour conversion with the cursor

**Files:** Create `packages/desktop/src/converter/…` including `converter/shaders/fullscreen_vs.hlsl`, `luma_ps.hlsl`, `chroma_ps.hlsl`, `composite.hlsli`; modify `lib.rs`.

**Interfaces — Consumes:** `GpuDevice`, `PointerState`, `CursorImage`, `BitDepth`, `Planes`. **Produces:**
```rust
pub struct Converter { /* shaders, constant buffer, output texture and views, cursor texture */ }
impl Converter {
    pub fn new(device: &GpuDevice, width: u32, height: u32, depth: BitDepth) -> Result<Converter, DesktopError>;
    pub fn convert(&mut self, device: &GpuDevice, source: &ID3D11Texture2D, pointer: &PointerState) -> Result<(), DesktopError>;
    pub fn output(&self) -> &ID3D11Texture2D;
    pub fn depth(&self) -> BitDepth;
}
pub fn read_planes(device: &GpuDevice, texture: &ID3D11Texture2D, depth: BitDepth) -> Result<Planes, DesktopError>;
```
- Output texture: `DXGI_FORMAT_NV12` (Eight) or `DXGI_FORMAT_P010` (Ten), `USAGE_DEFAULT`, `BIND_RENDER_TARGET | BIND_SHADER_RESOURCE`. Odd sizes → `OddSize`.
- Two passes with a full-screen triangle vertex shader (`SV_VertexID`, no vertex buffer): luma into an RTV of format `R8_UNORM` / `R16_UNORM`, chroma into an RTV of format `R8G8_UNORM` / `R16G16_UNORM` at half size (the view format selects the plane). Pixel shaders read the source with `Texture2D.Load` at integer coordinates (no sampler), implement exactly the reference maths of Task 1, and write `code / 255` for 8-bit and `code · 64 / 65535` for 10-bit.
- Cursor: a constant buffer with left, top, width, height, visible, kind, and an SRV of a BGRA cursor texture re-created whenever `pointer.shape_generation` changes. A function in `composite.hlsli` applies the cursor to each source sample before conversion, mirroring `composite_cursor` (Masked: integer AND/XOR on 8-bit values; Alpha: blend in float).
- HLSL sources are joined with `concat!(include_str!(…))`, compiled with `D3DCompile`, entry `main`, targets `vs_5_0` / `ps_5_0`, `D3DCOMPILE_OPTIMIZATION_LEVEL3`; a compiler error becomes `ShaderCompile(<compiler message>)`.
- `read_planes`: copy to a staging texture, `Map`, read the Y plane then the interleaved UV plane (`RowPitch · height` bytes in); P010 values are shifted right by 6.

- [ ] **Step 1: Write the failing tests** (ignored, `"needs a GPU and a desktop"`; helper `bgra_texture(device, width, height, bytes)` creates a `USAGE_DEFAULT`, `BIND_SHADER_RESOURCE` texture with initial data; helper `assert_close(actual: &Planes, expected: &Planes, tolerance: u16)`):

```rust
#[test]
#[ignore = "needs a GPU and a desktop"]
fn the_gpu_matches_the_reference_for_8_bit() {
    let device = first_output_device();
    let mut state = 0x9E37_79B9_7F4A_7C15;
    let mut bgra = random_bgra(&mut state, 64, 32);
    bgra[..16].copy_from_slice(&[0, 0, 255, 255, 0, 255, 0, 255, 255, 0, 0, 255, 255, 255, 255, 255]);
    let source = bgra_texture(&device, 64, 32, &bgra);
    let mut converter = Converter::new(&device, 64, 32, BitDepth::Eight).unwrap();
    converter.convert(&device, &source, &PointerState::default()).unwrap();
    let planes = read_planes(&device, converter.output(), BitDepth::Eight).unwrap();
    assert_close(&planes, &reference_planes(&bgra, 64, 32, BitDepth::Eight), 1);
}
```
Also: `the_gpu_matches_the_reference_for_10_bit` (same with `Ten`), `an_inverting_cursor_is_composited_like_the_reference` (Masked 8×8 cursor at (−3, 5), visible; expected `reference_planes` of the CPU-composited image, tolerance 1), `a_blended_cursor_is_composited_like_the_reference` (Alpha 8×8 cursor with alpha 128 at (20, 10), tolerance 2), `odd_sizes_are_rejected` (`Converter::new(.., 63, 32, ..)` → `OddSize`), `the_desktop_converts_at_its_own_size` (Duplication + Converter at the output size, `convert` succeeds, output `GetDesc` has that size and `DXGI_FORMAT_NV12`), and `#[ignore = "benchmark"] converting_the_desktop_takes_this_long` (200 conversions of the captured frame, each waited with a `D3D11_QUERY_EVENT`; prints the mean in ms).
- [ ] **Step 2: Run** — expected: compile errors.
- [ ] **Step 3: Implement.**
- [ ] **Step 4: Run** `cargo nextest run -p desktop --run-ignored all` — expected: pass; note the benchmark output in the final report.
- [ ] **Step 5: Commit** `Convert the desktop to NV12 and P010 on the GPU`.

### Task 6: Crate root and final check

**Files:** Modify `packages/desktop/src/lib.rs`.

- [ ] **Step 1:** `lib.rs` declares the modules (`gpu`, `duplication`, `converter` under `#[cfg(windows)]`) and re-exports exactly:
```rust
pub use colour::{BitDepth, Planes, YCbCr, reference_planes, ycbcr_of};
pub use cursor::{CursorImage, CursorKind, POINTER_SHAPE_COLOR, POINTER_SHAPE_MASKED_COLOR, POINTER_SHAPE_MONOCHROME, composite_cursor, decode_cursor_shape};
#[cfg(windows)]
pub use converter::{Converter, read_planes};
#[cfg(windows)]
pub use duplication::{Duplication, FrameUpdate, PointerState};
#[cfg(windows)]
pub use gpu::{AdapterLuid, DesktopError, GpuDevice, NVIDIA_VENDOR_ID, OutputInfo, list_outputs};
```
- [ ] **Step 2: Run** `cargo nextest run --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all --check`, and once more `cargo nextest run -p desktop --run-ignored all` — expected: all green.
- [ ] **Step 3: Commit** `Export the desktop capture API`.
