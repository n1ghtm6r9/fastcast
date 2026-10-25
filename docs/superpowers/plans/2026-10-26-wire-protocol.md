# Wire Protocol Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build `packages/wire`, the pure Rust crate with fastcast's own UDP protocol: datagram codec, GF(256) Reed–Solomon erasure code, frame layout, packetizer, frame receiver, loss and FEC control, pacing schedule.

**Architecture:** No I/O and no dependencies. The PC sender (wave 2) uses `Packetizer`, `LossMeter`, `FecController`, `burst_sizes`; the TV library uses `FrameReceiver` and `ReceiveCounters`. Time is always passed in as an `Instant`, so tests control the clock.

**Tech Stack:** Rust 2024, std only, `cargo nextest`.

**Spec:** `docs/superpowers/specs/2026-10-26-native-pipeline-design.md` — sections "Wire protocol v1" and "Session".

## Global Constraints

- One item per file: a type, a function, a constant, or the `impl` block of one method (`FrameReceiver::accept` → `frame_receiver_accept.rs`). File name = item name in snake_case. Copy the layout of `packages/media/src/avc/` and `packages/media/src/flv/types/flv_demuxer_next_event.rs`.
- Feature module `mod.rs`: `mod constants; mod services; mod types;` (only those that exist), then `pub use constants::*;` etc., then `#[cfg(test)] pub(crate) mod test;`. Inner `mod.rs` files: `mod x;` lines, then `pub use x::*;` lines, alphabetical.
- One test per file `<sentence>_test.rs` in the module's `test/` folder; the `#[test] fn` is the sentence without `_test`. Helpers are their own files in `test/`, re-exported from `test/mod.rs`.
- No comments anywhere, tests included. Imports at the top of the file.
- No dependencies and no dev-dependencies. Random data comes from `crate::test_support` (Task 1).
- Invalid input returns `None` or is ignored; nothing in this crate panics on network data.
- `packages/wire/src/packet/constants/protocol_version.rs` (`pub const PROTOCOL_VERSION: u8 = 1;`) already exists. Keep it.
- Commands from the worktree root, never `cd`: `cargo nextest run -p wire`, `cargo clippy -p wire --all-targets -- -D warnings`, `cargo fmt --all --check`.
- Commit after each task: one-line imperative subject, no body. Never push.

## Review Focus

1. Hostile or corrupted datagrams: no byte sequence may make `decode_datagram` or `FrameReceiver::accept` panic, index out of bounds or allocate more than `MAX_FRAME_SIZE` per frame. Tests: Task 1 `random_bytes_never_panic_the_decoder`, Task 6 `shards_with_impossible_headers_are_ignored`, `random_shards_never_panic_the_receiver`.
2. Burst loss: consecutive lost datagrams must spread across blocks. Tests: Task 5 `datagrams_alternate_between_blocks`, Task 6 `a_burst_loss_inside_a_two_block_frame_is_recovered`.
3. Reordering at session start (frame 1 datagrams before frame 0): frame 0 still comes first. Test: Task 6 `frames_are_delivered_in_frame_order_when_datagrams_arrive_reordered`.
4. After any loss the decoder must not get a non-key video frame before the next keyframe. Tests: Task 6 `video_after_a_loss_waits_for_the_next_keyframe`, `a_lossy_stream_delivers_only_decodable_video`.
5. Multi-megabyte 4K keyframes: many blocks, uneven split. Tests: Task 4 `a_3_megabyte_keyframe_has_balanced_blocks`, Task 6 `a_lossy_stream_delivers_only_decodable_video` (uses large keyframes).

---

## File Structure

```
packages/wire/src/
  lib.rs            explicit re-exports (final list in Task 8)
  test_support/     xorshift random data for tests (cfg(test))
  packet/           datagram codec                      Task 1
  gf/               GF(256) arithmetic                  Task 2
  fec/              Cauchy Reed–Solomon                 Task 3
  layout/           frame → blocks → shards arithmetic  Task 4
  packetizer/       frame → datagrams (sender)          Task 5
  receiver/         datagrams → frames (TV)             Task 6
  control/          receive counters, loss, FEC level   Task 7
  pacing/           burst schedule                      Task 8
  session/          session timing constants            Task 8
```

### Task 1: Datagram codec

**Files:**
- Create: `packages/wire/src/packet/{constants,types,services,test}/*.rs`, `packages/wire/src/test_support/{mod.rs,xorshift.rs,random_bytes.rs}`
- Modify: `packages/wire/src/packet/mod.rs`, `packages/wire/src/packet/constants/mod.rs`, `packages/wire/src/lib.rs` (add `#[cfg(test)] mod test_support;`)

**Interfaces — Produces:**
```rust
pub const MAGIC: u8 = 0xFC;
pub const HEADER_SIZE: usize = 12;
pub const SHARD_HEADER_SIZE: usize = 18;
pub const FEEDBACK_SIZE: usize = 24;
pub const DATAGRAM_LIMIT: usize = 1400;
pub const SHARD_SIZE: usize = 1370;
pub enum PacketKind { Hello = 1, Welcome = 2, Shard = 3, Feedback = 4, Bye = 5, Stats = 6 }
pub enum Codec { Avc = 1, Hevc = 2, Av1 = 3 }
pub struct CodecSupport { pub codec: Codec, pub max_bit_depth: u8 }
pub struct Hello { pub nonce: u32, pub width: u16, pub height: u16, pub fps: u16, pub codecs: Vec<CodecSupport> }
pub struct Welcome { pub nonce: u32, pub shard_size: u16 }
pub enum FrameKind { Video = 0, Config = 1 }
pub struct ShardHeader { pub frame: u32, pub frame_kind: FrameKind, pub keyframe: bool, pub block: u16, pub blocks: u16, pub data_shards: u8, pub parity_shards: u8, pub shard: u8, pub frame_size: u32 }
pub struct Shard<'a> { pub header: ShardHeader, pub data: &'a [u8] }
pub struct Feedback { pub highest_frame: u32, pub received_packets: u32, pub highest_sequence: u32, pub lost_frames: u32, pub recovered_shards: u32, pub keyframe_needed: bool }
pub enum Packet<'a> { Hello(Hello), Welcome(Welcome), Shard(Shard<'a>), Feedback(Feedback), Bye, Stats(&'a str) }
pub struct Datagram<'a> { pub session: u32, pub sequence: u32, pub packet: Packet<'a> }
pub fn encode_datagram(datagram: &Datagram<'_>, out: &mut Vec<u8>);
pub fn write_datagram(datagram: &Datagram<'_>, out: &mut Vec<u8>);
pub fn decode_datagram(bytes: &[u8]) -> Option<Datagram<'_>>;
pub fn xorshift(state: &mut u64) -> u64;
pub fn random_bytes(state: &mut u64, length: usize) -> Vec<u8>;
```
- All types derive `Clone, Debug, PartialEq, Eq`; add `Copy` where every field is `Copy`, `Default` on `Feedback`.
- `encode_datagram` clears `out` and writes; `write_datagram` appends (the packetizer uses it to fill one buffer with many datagrams).
- Byte layout and decoding rules are the spec's: wrong magic or version, unknown kind, payload shorter than its kind needs → `None`; a Hello with fewer pairs than `codec_count` → `None`; unknown codec ids are skipped; unknown frame kind byte → `None`; non-UTF-8 Stats → `None`; bytes after the fixed part of Welcome, Feedback and Bye are ignored; everything after the shard header is the shard `data`. `PacketKind` stays crate-internal.
- `xorshift`: `x ^= x << 13; x ^= x >> 7; x ^= x << 17;` stores and returns `x`; seeds are non-zero.

- [ ] **Step 1: Write the failing tests** in `packages/wire/src/packet/test/`, one file each:

```rust
#[test]
fn a_bye_datagram_is_the_twelve_byte_header() {
    let mut out = Vec::new();
    encode_datagram(&Datagram { session: 0x0102_0304, sequence: 0x0A0B_0C0D, packet: Packet::Bye }, &mut out);
    assert_eq!(out, [0xFC, 1, 5, 0, 4, 3, 2, 1, 0x0D, 0x0C, 0x0B, 0x0A]);
}

#[test]
fn a_shard_header_follows_the_common_header() {
    let header = ShardHeader { frame: 7, frame_kind: FrameKind::Config, keyframe: true, block: 1, blocks: 2, data_shards: 3, parity_shards: 1, shard: 2, frame_size: 5000 };
    let mut out = Vec::new();
    encode_datagram(&Datagram { session: 9, sequence: 1, packet: Packet::Shard(Shard { header, data: &[0xAA] }) }, &mut out);
    assert_eq!(out[..3], [0xFC, 1, 3]);
    assert_eq!(out[12..], [7, 0, 0, 0, 1, 1, 1, 0, 2, 0, 3, 1, 2, 0, 0x88, 0x13, 0, 0, 0xAA]);
}

#[test]
fn a_hello_lists_its_codecs_after_the_mode() {
    let hello = Hello { nonce: 0x1122_3344, width: 3840, height: 2160, fps: 120, codecs: vec![CodecSupport { codec: Codec::Hevc, max_bit_depth: 10 }, CodecSupport { codec: Codec::Avc, max_bit_depth: 8 }] };
    let mut out = Vec::new();
    encode_datagram(&Datagram { session: 0, sequence: 0, packet: Packet::Hello(hello) }, &mut out);
    assert_eq!(out[12..], [0x44, 0x33, 0x22, 0x11, 0x00, 0x0F, 0x70, 0x08, 120, 0, 2, 2, 10, 1, 8]);
}

#[test]
fn the_keyframe_request_is_bit_zero_of_the_feedback_flags() {
    let mut out = Vec::new();
    encode_datagram(&Datagram { session: 1, sequence: 1, packet: Packet::Feedback(Feedback { keyframe_needed: true, ..Feedback::default() }) }, &mut out);
    assert_eq!(out.len(), 36);
    assert_eq!(out[32..], [1, 0, 0, 0]);
}

#[test]
fn every_packet_kind_survives_a_round_trip() {
    // one Datagram per variant: Hello (2 codecs), Welcome { nonce: 5, shard_size: 1370 },
    // Shard (data [1, 2, 3]), Feedback (all five counters distinct, keyframe_needed true), Bye, Stats("{\"fps\":120}")
    // for each: encode_datagram, then assert_eq!(decode_datagram(&out), Some(datagram))
}
```
One-assertion files (expected result in brackets): `a_datagram_with_another_magic_is_ignored` (first byte 0xFB → `None`), `a_datagram_with_another_version_is_ignored` (version 2 → `None`), `an_unknown_kind_is_ignored` (kind 7 → `None`), `a_datagram_shorter_than_the_header_is_ignored` (11 bytes → `None`), `a_truncated_shard_header_is_ignored` (12 + 17 bytes → `None`), `a_hello_announcing_more_codecs_than_it_carries_is_ignored` (`None`), `unknown_codec_ids_in_a_hello_are_skipped` (pairs (9, 8), (2, 10) → `codecs == [Hevc/10]`), `stats_that_are_not_utf8_are_ignored` (payload [0xFF] → `None`), `an_unknown_frame_kind_is_ignored` (frame kind byte 2 → `None`), `random_bytes_never_panic_the_decoder` (10 000 buffers from `random_bytes` of length `xorshift % 1501`, each starting with `[0xFC, 1, (xorshift % 8) as u8]`, passed to `decode_datagram`).

- [ ] **Step 2: Run** `cargo nextest run -p wire` — expected: compile errors for the missing items.
- [ ] **Step 3: Implement** the constants, types and the three services (private helpers such as `decode_hello`, `decode_shard`, `write_header` are their own files under `services/`).
- [ ] **Step 4: Run** `cargo nextest run -p wire` and clippy — expected: all pass, no warnings.
- [ ] **Step 5: Commit** `Encode and decode wire datagrams`.

### Task 2: GF(256) arithmetic

**Files:** Create `packages/wire/src/gf/…`; modify `packages/wire/src/lib.rs` (`mod gf;`).

**Interfaces — Produces:**
```rust
pub const FIELD_POLYNOMIAL: u16 = 0x11D;
pub static GF_EXP: [u8; 512];
pub static GF_LOG: [u8; 256];
pub fn gf_mul(a: u8, b: u8) -> u8;
pub fn gf_inv(a: u8) -> u8;
pub fn mul_add_slice(coefficient: u8, source: &[u8], target: &mut [u8]);
```
- `GF_EXP[i] = 2^i` for `i < 512` (doubled so `exp[log a + log b]` needs no modulo); `GF_LOG[0] = 0` and is never used. Both are `static`s computed by `const fn` — no runtime init, no locks.
- `gf_inv(0) == 0`.
- `mul_add_slice`: `target[i] ^= coefficient · source[i]` over `min(len)` bytes; builds a 256-byte product row for the coefficient once, then one lookup and XOR per byte; coefficient 0 returns at once, coefficient 1 is a plain XOR.

- [ ] **Step 1: Write the failing tests**: `two_times_128_wraps_through_the_field_polynomial` (`gf_mul(2, 128) == 0x1D`), `the_generator_two_has_order_255` (`GF_EXP[255] == 1` and `(1..255).all(|i| GF_EXP[i] != 1)`), `every_non_zero_element_times_its_inverse_is_one` (`(1..=255).all(|a| gf_mul(a, gf_inv(a)) == 1)`), `multiplying_by_zero_gives_zero` (both argument orders, all values), `multiplication_distributes_over_xor` (all `a`, `b` with `c` in `[1, 2, 0x53, 0xCA, 255]`), `mul_add_slice_xors_the_scaled_source_into_the_target` (target `[1, 2, 3]`, source `[1, 2, 3]`, coefficient 2 → `[3, 6, 5]`), `mul_add_slice_agrees_with_gf_mul_for_every_byte` (every coefficient, source `0..=255`, zero target → `target[i] == gf_mul(c, i)`).
- [ ] **Step 2: Run** the tests — expected: compile errors.
- [ ] **Step 3: Implement.**
- [ ] **Step 4: Run** the tests and clippy — expected: pass.
- [ ] **Step 5: Commit** `Add GF(256) arithmetic`.

### Task 3: Cauchy Reed–Solomon

**Files:** Create `packages/wire/src/fec/…`; modify `lib.rs` (`mod fec;`).

**Interfaces — Consumes:** `gf_mul`, `gf_inv`, `mul_add_slice`. **Produces:**
```rust
pub const MAX_DATA_SHARDS: usize = 128;
pub const MAX_PARITY_SHARDS: usize = 127;
pub fn cauchy_coefficient(parity_row: usize, data_column: usize) -> u8;
pub fn encode_parity(data_shards: usize, parity_shards: usize, shard_size: usize, block: &mut [u8]);
pub fn recover_data(data_shards: usize, parity_shards: usize, shard_size: usize, block: &mut [u8], present: &[bool]) -> Option<usize>;
```
- `cauchy_coefficient(i, j) = gf_inv((128 + i) as u8 ^ j as u8)`.
- `block` holds `(data_shards + parity_shards) * shard_size` bytes, shard `n` at `n * shard_size`, data first. `encode_parity` overwrites the parity slots. `recover_data` rebuilds the absent data shards in place and returns how many it rebuilt; it returns `None` and leaves `block` untouched when fewer than `data_shards` shards are present. It never rebuilds parity and never edits `present`.
- Algorithm: with `r` absent data shards, take the first `r` present parity rows; syndrome `s_a = parity_a ⊕ Σ_{present j} C[a][j]·d_j`; matrix `M[a][b] = C[row_a][absent_b]` (any square Cauchy submatrix is invertible); Gauss–Jordan inverse over GF(256); `d_absent_b = Σ_a M⁻¹[b][a]·s_a` with `mul_add_slice`.

- [ ] **Step 1: Write the failing tests**:

```rust
#[test]
fn the_cauchy_coefficient_is_the_inverse_of_x_xor_y() {
    assert_eq!(cauchy_coefficient(0, 0), gf_inv(128));
    assert_eq!(cauchy_coefficient(2, 5), gf_inv(130 ^ 5));
}

#[test]
fn one_parity_shard_of_one_data_shard_is_the_data_times_its_coefficient() {
    let mut block = vec![1, 2, 3, 4, 0, 0, 0, 0];
    encode_parity(1, 1, 4, &mut block);
    assert_eq!(block[4..], [1, 2, 3, 4].map(|byte| gf_mul(gf_inv(128), byte)));
}

#[test]
fn any_lost_shards_up_to_the_parity_count_are_rebuilt() {
    // k = 10, m = 4, shard_size = 64, random data, encode_parity.
    // For every subset of the 14 shards with 1..=4 members: copy the block, zero the absent shards,
    // present[n] = !absent; recover_data == Some(absent data count) and copy[..640] == original[..640].
}
```
Also: `parity_of_an_all_zero_block_is_zero`, `a_block_with_every_data_shard_present_needs_no_recovery` (`Some(0)`, block unchanged), `losing_more_shards_than_the_parity_count_cannot_be_recovered` (k 5, m 2, three data shards absent → `None`, block unchanged), `the_largest_block_recovers_from_losing_all_but_one_data_shard` (k 128, m 127, shard_size 16, data shards 0..127 absent, all parity present → `Some(127)` and data equal), and `#[ignore = "benchmark"] parity_for_a_two_megabyte_frame_takes_this_long` (16 blocks of k 96, m 10, `SHARD_SIZE` bytes; prints MB/s with `println!`).
- [ ] **Step 2: Run** — expected: compile errors.
- [ ] **Step 3: Implement** (`encode_parity`, `recover_data`, plus private helpers such as `invert_matrix` in their own files).
- [ ] **Step 4: Run** tests and clippy — expected: pass.
- [ ] **Step 5: Commit** `Add Cauchy Reed–Solomon erasure coding`.

### Task 4: Frame layout

**Files:** Create `packages/wire/src/layout/…`; modify `lib.rs`.

**Interfaces — Consumes:** `SHARD_SIZE`, `MAX_DATA_SHARDS`, `MAX_PARITY_SHARDS`. **Produces:**
```rust
pub const MAX_FRAME_SIZE: usize = 16 * 1024 * 1024;
pub struct FrameLayout { pub frame_size: u32, pub data_shards: usize, pub blocks: usize }
impl FrameLayout {
    pub fn new(frame_size: u32) -> FrameLayout;
    pub fn data_shards_in(&self, block: usize) -> usize;
    pub fn first_data_shard_of(&self, block: usize) -> usize;
    pub fn data_shard_length(&self, data_shard: usize) -> usize;
}
pub fn parity_shards_for(data_shards: usize, fec_percent: u8) -> usize;
```
- `data_shards = max(1, ceil(frame_size / SHARD_SIZE))`; `blocks = ceil(data_shards / MAX_DATA_SHARDS)`; the first `data_shards % blocks` blocks get one shard more; `data_shard_length` is `SHARD_SIZE` except the last, which is `frame_size − (data_shards − 1)·SHARD_SIZE`.
- `parity_shards_for`: 0 when `fec_percent == 0`, else `max(1, ceil(k·p / 100))` capped at `MAX_PARITY_SHARDS`. `FrameLayout` derives `Clone, Copy, Debug, PartialEq, Eq`.

- [ ] **Step 1: Write the failing tests**: `a_frame_that_fits_one_shard_is_one_block_of_one_data_shard` (`new(100)`: 1 data shard, 1 block, `data_shards_in(0) == 1`, `data_shard_length(0) == 100`), `an_empty_frame_still_has_one_data_shard` (`new(0)`, length 0), `one_byte_over_a_shard_needs_a_second_data_shard` (`new(1371)`: 2, `data_shard_length(1) == 1`), `129_data_shards_are_split_into_blocks_of_65_and_64` (`new(129 * 1370)`: 2 blocks, 65 and 64, `first_data_shard_of(1) == 65`), `a_3_megabyte_keyframe_has_balanced_blocks` (`new(3_000_000)`: 2190 data shards, 18 blocks, blocks 0..12 have 122 and blocks 12..18 have 121), `parity_is_the_rounded_up_percentage_of_the_data_shards` (`(10, 10) → 1`, `(11, 10) → 2`, `(1, 10) → 1`, `(128, 50) → 64`, `(128, 100) → 127`, `(5, 0) → 0`).
- [ ] **Step 2: Run** — expected: compile errors.
- [ ] **Step 3: Implement.**
- [ ] **Step 4: Run** — expected: pass.
- [ ] **Step 5: Commit** `Lay out frames as blocks of shards`.

### Task 5: Packetizer

**Files:** Create `packages/wire/src/packetizer/…`; modify `lib.rs`.

**Interfaces — Consumes:** `write_datagram`, `encode_datagram`, `FrameLayout`, `parity_shards_for`, `encode_parity`, `MAX_FRAME_SIZE`. **Produces:**
```rust
pub struct OutgoingFrame<'a> { pub kind: FrameKind, pub keyframe: bool, pub payload: &'a [u8] }
pub struct Datagrams { bytes: Vec<u8>, ends: Vec<usize> }
impl Datagrams {
    pub fn new() -> Datagrams;
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
    pub fn get(&self, index: usize) -> &[u8];
    pub fn iter(&self) -> impl Iterator<Item = &[u8]> + '_;
    pub fn clear(&mut self);
}
pub struct Packetizer { session: u32, next_sequence: u32, next_frame: u32, block: Vec<u8> }
impl Packetizer {
    pub fn new(session: u32) -> Packetizer;
    pub fn session(&self) -> u32;
    pub fn control(&mut self, packet: &Packet<'_>, out: &mut Vec<u8>);
    pub fn frame(&mut self, frame: &OutgoingFrame<'_>, fec_percent: u8, out: &mut Datagrams) -> u32;
}
```
- Sequence starts at 0 and grows by 1 per datagram, control and shard alike. Frame numbers start at 0 and grow by 1 per `frame` call, which clears `out` and returns the frame's number.
- Per block: copy its data into the scratch `block` (zero-padded to `SHARD_SIZE`), `encode_parity` with `parity_shards_for(k, fec_percent)`.
- Send order: shard index 0 of every block, then index 1 of every block, and so on; a block whose shards are used up is skipped.
- A data shard carries `data_shard_length` bytes (only the frame's last data shard is short); parity always carries `SHARD_SIZE`.
- A payload longer than `MAX_FRAME_SIZE` produces no datagrams but still uses up its frame number (the TV then sees a gap and asks for a keyframe).
- `Datagrams` derives `Default`.

- [ ] **Step 1: Write the failing tests** (decode the output with `decode_datagram` to assert):

```rust
#[test]
fn a_small_frame_is_one_data_and_one_parity_datagram() {
    let payload: Vec<u8> = (0..100).collect();
    let mut packetizer = Packetizer::new(7);
    let mut out = Datagrams::new();
    packetizer.frame(&OutgoingFrame { kind: FrameKind::Video, keyframe: true, payload: &payload }, 10, &mut out);
    assert_eq!(out.len(), 2);
    let Some(Datagram { packet: Packet::Shard(first), .. }) = decode_datagram(out.get(0)) else { panic!("not a shard") };
    assert_eq!(first.header, ShardHeader { frame: 0, frame_kind: FrameKind::Video, keyframe: true, block: 0, blocks: 1, data_shards: 1, parity_shards: 1, shard: 0, frame_size: 100 });
    assert_eq!(first.data, payload);
    let Some(Datagram { packet: Packet::Shard(second), .. }) = decode_datagram(out.get(1)) else { panic!("not a shard") };
    assert_eq!((second.header.shard, second.data.len()), (1, SHARD_SIZE));
}
```
Also: `no_datagram_exceeds_the_datagram_limit` (500 000 random bytes, fec 50: every datagram ≤ 1400 bytes), `sequences_count_every_datagram_across_frames` (two 100-byte frames at fec 10 → sequences 0, 1, 2, 3), `frame_numbers_count_frames` (`frame` returns 0 then 1, headers carry them), `control_packets_take_the_next_sequence` (after one 2-datagram frame, `control(&Packet::Bye, ..)` decodes with session 7 and sequence 2), `datagrams_alternate_between_blocks` (129·1370 bytes at fec 0 → 129 datagrams; the first four are (block 0, shard 0), (1, 0), (0, 1), (1, 1)), `a_frame_without_fec_has_no_parity_datagrams` (3000 bytes at fec 0 → 3 datagrams, `parity_shards == 0`), `a_frame_over_the_size_limit_is_skipped_but_uses_its_number` (`MAX_FRAME_SIZE + 1` bytes → returns 0 and `out` is empty; the next frame returns 1).
- [ ] **Step 2: Run** — expected: compile errors.
- [ ] **Step 3: Implement.**
- [ ] **Step 4: Run** — expected: pass.
- [ ] **Step 5: Commit** `Packetize frames into paced shard datagrams`.

### Task 6: Frame receiver

**Files:** Create `packages/wire/src/receiver/…`; modify `lib.rs`.

**Interfaces — Consumes:** `Shard`, `FrameKind`, `FrameLayout`, `MAX_FRAME_SIZE`, `MAX_PARITY_SHARDS`, `recover_data`, `SHARD_SIZE`; tests use `Packetizer`, `Datagrams`, `decode_datagram`. **Produces:**
```rust
pub const REORDER_WINDOW: Duration = Duration::from_millis(5);
pub const FRAME_TIMEOUT: Duration = Duration::from_millis(100);
pub const MAX_PENDING_FRAMES: usize = 64;
pub struct ReceivedFrame { pub number: u32, pub kind: FrameKind, pub keyframe: bool, pub payload: Vec<u8> }
pub struct FrameReceiver { /* private */ }
impl FrameReceiver {
    pub fn new() -> FrameReceiver;
    pub fn accept(&mut self, shard: &Shard<'_>, now: Instant);
    pub fn poll(&mut self, now: Instant) -> Option<ReceivedFrame>;
    pub fn next_deadline(&self) -> Option<Instant>;
    pub fn highest_frame(&self) -> u32;
    pub fn lost_frames(&self) -> u32;
    pub fn recovered_shards(&self) -> u32;
    pub fn keyframe_needed(&self) -> bool;
}
```
Behaviour (the spec's "Frame receiver rules", made exact):
- The expected frame starts at 0. Shards of frames below it, of complete frames, and duplicates are ignored.
- Validation: `frame_size ≤ MAX_FRAME_SIZE`; `blocks`, `data_shards` and the data length agree with `FrameLayout::new(frame_size)`; `parity_shards ≤ MAX_PARITY_SHARDS`; `shard < data_shards + parity_shards`; parity data is `SHARD_SIZE` long; frame kind, keyframe flag and frame size match the frame's first shard, the parity count matches the block's first shard. Anything else is ignored.
- A block is complete when all its data shards are present. When a block holds at least `k` shards but misses data, `accept` runs `recover_data` and adds the rebuilt count to `recovered_shards`.
- The expected frame becomes known when the first shard of it or of any later frame arrives. It is lost if still incomplete at `known + FRAME_TIMEOUT`. If a later frame is complete and the expected one is not, every incomplete frame before the earliest complete later frame is lost at `that frame's completion + REORDER_WINDOW`. Whenever more than `MAX_PENDING_FRAMES` frames are pending (`expected..=highest`), `accept` drops the oldest as lost until 64 remain. Dropping must cost time proportional to the frames actually stored, not to the gap: a shard numbered 4 000 000 000 moves `expected` in one step and adds the gap to `lost_frames` (saturating).
- `poll` first applies the losses due at `now`, then returns the expected frame if complete (call it until `None`). `next_deadline` is the earliest instant at which `poll` would declare a loss, `None` when nothing is pending.
- Keyframes: the receiver starts out waiting for a keyframe, and every lost frame starts the wait again. While waiting, config frames are delivered, non-key video frames are dropped, and a key video frame is delivered and ends the wait. `keyframe_needed()` is true while waiting if at least one video frame was lost or dropped since the wait began.
- `highest_frame` is the highest frame number among accepted shards, 0 before any.
- `FrameReceiver` implements `Default`; `ReceivedFrame` derives `Clone, Debug, PartialEq, Eq`.

Test helpers in `receiver/test/`: `datagrams_of(packetizer: &mut Packetizer, kind: FrameKind, keyframe: bool, payload: &[u8], fec_percent: u8) -> Vec<Vec<u8>>`, `feed(receiver: &mut FrameReceiver, datagram: &[u8], now: Instant)` (decode, accept if it is a Shard), `deliveries(receiver: &mut FrameReceiver, now: Instant) -> Vec<ReceivedFrame>` (poll until `None`).

- [ ] **Step 1: Write the failing tests** (`t0 = Instant::now()`, `ms(n) = Duration::from_millis(n)`):

```rust
#[test]
fn a_frame_missing_more_shards_than_parity_is_lost_after_the_frame_timeout() {
    let mut packetizer = Packetizer::new(1);
    let mut receiver = FrameReceiver::new();
    let t0 = Instant::now();
    for datagram in datagrams_of(&mut packetizer, FrameKind::Video, true, &[1; 100], 10) { feed(&mut receiver, &datagram, t0); }
    assert_eq!(deliveries(&mut receiver, t0).len(), 1);
    let second = datagrams_of(&mut packetizer, FrameKind::Video, false, &[2; 3000], 10);
    for datagram in &second[2..] { feed(&mut receiver, datagram, t0); }
    assert!(deliveries(&mut receiver, t0 + Duration::from_millis(99)).is_empty());
    assert_eq!(receiver.lost_frames(), 0);
    assert!(deliveries(&mut receiver, t0 + Duration::from_millis(100)).is_empty());
    assert_eq!(receiver.lost_frames(), 1);
    assert!(receiver.keyframe_needed());
}

#[test]
fn a_lossy_stream_delivers_only_decodable_video() {
    // 300 frames, keyframe every 30th (keyframes 200 000 random bytes, others 20 000), fec 10,
    // each datagram dropped when xorshift % 100 < 3; frame i fed at t0 + 8 ms · i, then polled at the same time;
    // finally poll at t0 + 10 s.
    // Assert: delivered numbers strictly increase; each payload equals the one sent;
    // a delivered non-key video frame always directly follows the previously delivered video frame (number + 1).
}
```
Also (one file each): `a_frame_arrives_whole_when_every_datagram_does` (5000-byte keyframe, fec 10 → one `ReceivedFrame { number: 0, kind: Video, keyframe: true, payload }`), `frames_are_delivered_in_frame_order_when_datagrams_arrive_reordered` (frame 1's datagrams fed before frame 0's → numbers `[0, 1]`), `a_lost_data_shard_is_rebuilt_from_parity` (10 000-byte keyframe at fec 20, datagram 0 dropped → payload equal, `recovered_shards() == 1`), `a_burst_loss_inside_a_two_block_frame_is_recovered` (200·1370-byte keyframe at fec 10, datagrams 10..30 dropped → delivered intact), `a_complete_later_frame_waits_the_reorder_window` (frame 0 key delivered, frame 1 fully dropped, frame 2 non-key fed at t0 → nothing and 0 lost at t0 + 4 ms; at t0 + 5 ms `lost_frames() == 1` and nothing delivered), `video_after_a_loss_waits_for_the_next_keyframe` (continuing that: frame 3 keyframe → delivered, then `keyframe_needed() == false`), `config_frames_pass_while_waiting_for_a_keyframe` (new receiver, frame 0 Config → delivered), `video_before_the_first_keyframe_is_dropped` (frame 0 non-key video → nothing, `keyframe_needed() == true`), `the_first_keyframe_needs_no_request` (only frame 0 Config delivered → `keyframe_needed() == false`), `duplicate_datagrams_are_ignored` (every datagram fed twice → one delivery), `shards_of_a_delivered_frame_are_ignored` (refeeding after delivery → nothing, `highest_frame()` unchanged), `more_than_64_pending_frames_drop_the_oldest` (frame 0 key delivered, then only the first datagram of each of frames 1..=65, 3000 bytes at fec 0 → `lost_frames() == 1` before any poll), `a_shard_of_a_frame_over_the_size_limit_is_ignored` (hand-built shard with `frame_size = MAX_FRAME_SIZE as u32 + 1` → `next_deadline() == None`, `highest_frame() == 0`), `shards_with_impossible_headers_are_ignored` (for frame 0 with `frame_size` 100: `blocks` 2; `data_shards` 2; `shard` 5 with k 1 m 1; data length 99; `parity_shards` 128 → `next_deadline() == None` after each), `next_deadline_is_the_frame_timeout_after_the_first_shard` (one datagram of a 3-shard frame at t0 → `Some(t0 + 100 ms)`), `random_shards_never_panic_the_receiver` (10 000 random datagrams that start with a valid common header of kind 3, fed and polled).
- [ ] **Step 2: Run** — expected: compile errors.
- [ ] **Step 3: Implement** (internal types such as `PendingFrame` and `PendingBlock` are their own files under `receiver/types/`).
- [ ] **Step 4: Run** tests and clippy — expected: pass.
- [ ] **Step 5: Commit** `Reassemble frames from shards with loss recovery`.

### Task 7: Receive counters, loss meter, FEC controller

**Files:** Create `packages/wire/src/control/…`; modify `lib.rs`.

**Interfaces — Consumes:** `Feedback`. **Produces:**
```rust
pub const BASE_FEC_PERCENT: u8 = 10;
pub const MAX_FEC_PERCENT: u8 = 50;
pub const FEC_LOSS_FACTOR: u32 = 3;
pub const FEC_DECAY_INTERVAL: Duration = Duration::from_secs(1);
pub struct ReceiveCounters { pub received_packets: u32, pub highest_sequence: u32 }
impl ReceiveCounters { pub fn record(&mut self, sequence: u32); }
pub struct LossMeter { /* previous counters */ }
impl LossMeter { pub fn new() -> LossMeter; pub fn update(&mut self, feedback: &Feedback) -> Option<f64>; }
pub struct FecController { /* percent, last_step */ }
impl FecController { pub fn new(now: Instant) -> FecController; pub fn percent(&self) -> u8; pub fn observe(&mut self, loss: f64, now: Instant); }
```
- `record`: `received_packets` grows by 1 (wrapping); `highest_sequence` keeps the maximum. `ReceiveCounters` and `LossMeter` implement `Default`.
- `update`: the first call stores the counters and returns `None`; later calls compare with the stored ones (wrapping subtraction): no new sequences → `None`; else `clamp(1 − Δreceived / Δhighest, 0, 1)`; always store the new counters.
- `observe`: `target = min(MAX, ceil(loss · 100 · FEC_LOSS_FACTOR))`. If `target > percent`, `percent = target` and `last_step = now`. Otherwise every whole `FEC_DECAY_INTERVAL` since `last_step` lowers `percent` by 1 (never below `BASE_FEC_PERCENT`) and moves `last_step` by that many intervals.

- [ ] **Step 1: Write the failing tests**: `received_packets_count_every_datagram` (record 5, 6, 6 → 3 and 6), `the_highest_sequence_ignores_late_datagrams` (10 then 3 → 10), `the_first_feedback_gives_no_loss` (`None`), `loss_is_the_share_of_missing_sequences_between_two_feedbacks` ((100, 99) then (190, 199) → `0.1` within `1e-9`), `duplicates_never_make_loss_negative` ((100, 99) then (220, 199) → `0.0`), `fec_starts_at_ten_percent`, `fec_jumps_to_three_times_the_loss` (0.05 → 15), `fec_is_capped_at_fifty_percent` (0.4 → 50), `fec_falls_one_percent_per_second_back_to_ten` (0.05 at t0 → 15; 0.0 at t0 + 999 ms → 15; t0 + 1 s → 14; t0 + 1.5 s → 14; t0 + 2 s → 13; t0 + 10 s → 10).
- [ ] **Step 2: Run** — expected: compile errors.
- [ ] **Step 3: Implement.**
- [ ] **Step 4: Run** — expected: pass.
- [ ] **Step 5: Commit** `Measure loss and adapt the FEC level`.

### Task 8: Pacing, session constants, crate root

**Files:** Create `packages/wire/src/pacing/…`, `packages/wire/src/session/…`; modify `lib.rs`.

**Interfaces — Produces:**
```rust
pub const PACE_TICK: Duration = Duration::from_micros(500);
pub const PACE_SPREAD_DIVISOR: u32 = 2;
pub fn burst_sizes(packets: usize, frame_interval: Duration) -> Vec<usize>;
pub const HELLO_INTERVAL: Duration = Duration::from_millis(250);
pub const FEEDBACK_INTERVAL: Duration = Duration::from_millis(20);
pub const SESSION_TIMEOUT: Duration = Duration::from_secs(3);
```
- `burst_sizes`: `bursts = clamp(floor((frame_interval / PACE_SPREAD_DIVISOR) / PACE_TICK), 1, packets)`; packets split evenly, the first bursts take the remainder; empty when `packets == 0`.
- Final `lib.rs` (modules private, these names public, nothing else):
```rust
pub use control::{BASE_FEC_PERCENT, FEC_DECAY_INTERVAL, FEC_LOSS_FACTOR, FecController, LossMeter, MAX_FEC_PERCENT, ReceiveCounters};
pub use fec::{MAX_DATA_SHARDS, MAX_PARITY_SHARDS};
pub use layout::{FrameLayout, MAX_FRAME_SIZE, parity_shards_for};
pub use pacing::{PACE_SPREAD_DIVISOR, PACE_TICK, burst_sizes};
pub use packet::{Codec, CodecSupport, DATAGRAM_LIMIT, Datagram, FEEDBACK_SIZE, Feedback, FrameKind, HEADER_SIZE, Hello, MAGIC, PROTOCOL_VERSION, Packet, SHARD_HEADER_SIZE, SHARD_SIZE, Shard, ShardHeader, Welcome, decode_datagram, encode_datagram};
pub use packetizer::{Datagrams, OutgoingFrame, Packetizer};
pub use receiver::{FRAME_TIMEOUT, FrameReceiver, MAX_PENDING_FRAMES, REORDER_WINDOW, ReceivedFrame};
pub use session::{FEEDBACK_INTERVAL, HELLO_INTERVAL, SESSION_TIMEOUT};
```

- [ ] **Step 1: Write the failing tests**: `a_frame_is_spread_over_half_the_frame_interval` (`burst_sizes(80, 8333 µs) == vec![10; 8]`), `the_first_bursts_take_the_remainder` (`(10, 8333 µs) → [2, 2, 1, 1, 1, 1, 1, 1]`), `there_are_never_more_bursts_than_packets` (`(3, 8333 µs) → [1, 1, 1]`), `a_very_short_interval_sends_everything_at_once` (`(50, 600 µs) → [50]`), `no_packets_means_no_bursts` (`(0, 8333 µs) → []`).
- [ ] **Step 2: Run** — expected: compile errors.
- [ ] **Step 3: Implement** and write the final `lib.rs`.
- [ ] **Step 4: Run** `cargo nextest run --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all --check` — expected: all green, no dead-code warnings.
- [ ] **Step 5: Commit** `Add the pacing schedule and session timings`.
