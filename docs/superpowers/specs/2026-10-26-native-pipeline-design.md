# fastcast native pipeline — design

Date: 2026-10-26. Status: approved by the user ("делай просто свое лучшее решение"), the user opted out of further questions.

## Intent

The user wants the fastest PC → TV screen stream on the market: "максимально мощный быстрый алгоритм трансляции видео без потерь качества, чтобы минимум было 4K 120 fps… пиши самый низкоуровневый алгоритм с 0 сам, никаких ws и т. д." Existing products (Moonlight/Sunshine, Parsec, Steam Link) already reach 4K 120; fastcast must be at least as good and better where it can.

## Goals

- 3840x2160 at up to 120 fps whenever the TV's display and decoder support it; otherwise the best mode the TV has.
- No visible quality loss: the highest bitrate the link holds, 10-bit encoding when the decoder supports it, correct BT.709 colour, carefully filtered chroma.
- Minimal latency from desktop change to picture on the TV.
- Our own low-level code for everything around the codec: screen capture, GPU colour conversion, encoder control, transport, forward error correction, pacing, the TV receive path.

## Non-goals

- Literal lossless video: 4K 120 4:2:0 uncompressed is about 12 Gbit/s, the TV link is about 0.1–0.5 Gbit/s.
- An own video codec: the TV decodes only in hardware (HEVC, H.264, maybe AV1); its CPU cannot decode 4K in software.
- Audio, remote input, browsers, desktop clients (later projects).

## Anti-goals (a failure even if it works)

- Breaking the TV self-update path (ConnectActivity → UpdateRunner); it must keep working whatever the stream code does.
- Any manual input: no typed IPs or ports; discovery stays automatic on UDP 3278.
- Automated tests that switch the PC monitors off, run the fastcast server in the background, or install builds on the TV.
- A regression against the current pipeline in picture quality or stability.

## Constraints

- Server: Windows 11, Rust (edition 2024, workspace in `Cargo.toml`), NVIDIA RTX 5070 (driver 621, NVENC API 13), AMD iGPU present, Parsec VDD for the TV-sized virtual display.
- TV: TCL "Smart TV Pro" G17_4K_GB, Android 14, panel 3840x2160; whether it has a 4K 120 Hz mode and a decoder for it is unknown and must be detected at run time. ABI unknown: ship both `arm64-v8a` and `armeabi-v7a`.
- Client: Kotlin, plain Activities, minSdk 26, targetSdk 35, MediaCodec into SurfaceView. Releases come from CI on every push to master; the TV updates itself.
- Code rules: one type, function or constant per file; `mod.rs` lists `mod x;` lines then `pub use x::*;` lines, alphabetical; modules split into `constants/`, `services/`, `types/`, `test/`; one test per file named `<sentence>_test.rs` whose function is the sentence; no comments in code (Rust, Kotlin, HLSL, C); imports at the top; errors are enums with hand-written `Display` and `Error`.
- Third-party code: only FFI declarations and build tools — `windows` (Win32/D3D11/DXGI), `jni-sys`, `libc`, `cc` (build script only). No ready-made streaming, FEC, codec-wrapper, WebSocket or HTTP crates in the new path.

## Architecture

```
PC                                                         TV
Desktop Duplication (BGRA texture on the GPU)
  → own HLSL pass: cursor + BT.709 limited RGB→YUV 4:2:0 (NV12 or P010)
  → NVENC through our own FFI (texture registered, never leaves the GPU)
  → Hub (config + access units)            ← ffmpeg backend stays as fallback
  → wire: frame → FEC blocks → shards → paced UDP datagrams  ⇢  UDP 3278
                                                           Rust receiver (JNI, own thread)
                                                             → reassembly + Reed–Solomon recovery
                                                             → Kotlin → MediaCodec (low latency) → SurfaceView
                                     feedback every 20 ms  ⇠  (loss, keyframe requests, keepalive)
```

Discovery and the stream share one UDP socket on port 3278. The WebSocket server (axum), the HTTP health endpoint and the separate discovery socket are removed.

## Wire protocol v1

All integers little-endian. Every datagram is at most `DATAGRAM_LIMIT = 1400` bytes (no IP fragmentation on a 1500-byte MTU).

### Common header (12 bytes)

| offset | size | field |
|---|---|---|
| 0 | 1 | magic `0xFC` |
| 1 | 1 | version `1` |
| 2 | 1 | kind |
| 3 | 1 | reserved, `0` |
| 4 | 4 | session id (`0` in Hello) |
| 8 | 4 | sequence: per sender and session, +1 for every datagram |

A datagram with another magic or version, an unknown kind, or a payload shorter than its kind needs is ignored. Datagrams whose first byte is `{` are discovery JSON (unchanged format, see below).

### Kinds

| kind | name | direction | payload |
|---|---|---|---|
| 1 | Hello | TV → PC | `nonce u32, width u16, height u16, fps u16, codec_count u8, codec_count × (codec u8, max_bit_depth u8)` |
| 2 | Welcome | PC → TV | `nonce u32` (echo), `shard_size u16` |
| 3 | Shard | PC → TV | shard header (18 bytes) + shard bytes |
| 4 | Feedback | TV → PC | `highest_frame u32, received_packets u32, highest_sequence u32, lost_frames u32, recovered_shards u32, flags u8 (bit 0 = keyframe needed), reserved 3 bytes` |
| 5 | Bye | both | empty |
| 6 | Stats | TV → PC | UTF-8 JSON text (the existing client stats message) |

Codec ids: `1` H.264, `2` HEVC, `3` AV1; listed in the TV's order of preference. Unknown ids are skipped. `width = height = 0` means "no display mode" (the server captures its configured monitor). `fps = 0` means the server default.

### Shard header (18 bytes, after the common header)

| offset | size | field |
|---|---|---|
| 0 | 4 | frame number (per session, starts at 0, +1 per frame) |
| 4 | 1 | frame kind: `0` video, `1` config |
| 5 | 1 | frame flags: bit 0 = keyframe |
| 6 | 2 | block index |
| 8 | 2 | block count |
| 10 | 1 | data shards `k` in this block |
| 11 | 1 | parity shards `m` in this block |
| 12 | 1 | shard index in the block (`0..k` data, `k..k+m` parity) |
| 13 | 1 | reserved `0` |
| 14 | 4 | frame size in bytes |

`SHARD_SIZE = 1400 − 12 − 18 = 1370` bytes. A frame's payload is cut into data shards of `SHARD_SIZE` bytes; the last data shard of the frame is sent without its zero padding, every other shard (all parity shards included) is exactly `SHARD_SIZE`.

Frame payloads:
- video: the hub's wire bytes as today — `[flags u8: bit 0 keyframe][timestamp_ms u32][access unit, 4-byte length-prefixed NAL units]`;
- config: the hub's config JSON text as today — `{"type":"config","codec":…,"width":…,"height":…,"fps":…,"description":"<base64 avcC/hvcC>"}`.

So the Kotlin decoder path (`FrameMessage.parse`, `ConfigMessage.parse`) keeps working unchanged.

### Frame layout

- `total = max(1, ceil(frame_size / SHARD_SIZE))` data shards.
- `blocks = ceil(total / MAX_DATA_SHARDS)`, `MAX_DATA_SHARDS = 128`; data shards are spread evenly, the first `total % blocks` blocks get one more.
- Parity per block: `m = ceil(k × fec_percent / 100)`, at least 1 when `fec_percent > 0`, at most `MAX_PARITY_SHARDS = 127`.
- Send order interleaves blocks: shard 0 of every block, then shard 1 of every block, …, so a burst loss is spread across blocks.

### Forward error correction

Systematic Reed–Solomon erasure code over GF(2^8), field polynomial `0x11D`, generator 2. Parity row `i` of a block is `p_i = Σ_j C[i][j] · d_j` with the Cauchy matrix `C[i][j] = 1 / (x_i ⊕ y_j)`, `y_j = j` (`j < k ≤ 128`), `x_i = 128 + i` (`i < m ≤ 127`). Any `k` of the `k + m` shards rebuild the block. Decoding picks as many parity rows as there are missing data shards, removes the known data from them and solves the small system by Gaussian elimination. The last data shard is zero-padded to `SHARD_SIZE` for both encoding and decoding.

`fec_percent` starts at 10 and is driven by the server: when the packet loss reported over the last feedback interval times 3 exceeds it, it jumps to `min(50, ceil(loss% × 3))`; otherwise it drops by 1 per second back towards 10.

### Pacing

Datagrams of one frame are not sent in one burst. They are spread over `PACE_SPREAD = 0.5` of the frame interval in bursts `PACE_TICK = 500 µs` apart: `bursts = clamp(floor(0.5 × interval / 500 µs), 1, packets)`, packets spread evenly, the first bursts take the remainder. Sending happens on a dedicated OS thread (sub-millisecond `std::thread::sleep`).

### Session

TV side:
1. Bind an ephemeral UDP port, send Hello every `HELLO_INTERVAL = 250 ms` until Welcome with the same nonce arrives.
2. Accept datagrams only from the server's address and with the Welcome's session id; feed every Shard into the frame receiver (below); deliver frames to Kotlin in frame order.
3. Send Feedback every `FEEDBACK_INTERVAL = 20 ms` and at once when a frame is lost.
4. On Bye, or when Kotlin's silence timer fires (15 s for the first message, 5 s after), the session ends and Kotlin reconnects with its existing backoff.

Frame receiver rules:
- Frame numbers start at 0 in every session, so the receiver starts by expecting frame 0.
- A shard is ignored when its frame is larger than `MAX_FRAME_SIZE = 16 MiB`, or when its block count, data shard count, shard index or length disagrees with the layout of its frame size, or its parity count is more than `ceil(k × MAX_FEC_PERCENT / 100)` for the `k` data shards of its block, or its frame kind, keyframe flag, frame size or parity count disagrees with earlier shards of the same frame or block. The packetizer clamps the `fec_percent` it is given to `MAX_FEC_PERCENT = 50`, so a sender never builds a shard that the receiver ignores for this reason.
- A complete frame equal to the next frame is delivered at once, then any following complete frames.
- A complete later frame waits at most `REORDER_WINDOW = 5 ms` for the gap before the missing frames are declared lost.
- The next frame is declared lost when it is still incomplete `FRAME_TIMEOUT = 100 ms` after the first shard of it or of any later frame arrived.
- More than `MAX_PENDING_FRAMES = 64` pending frames: the oldest are declared lost.
- The block buffers of all pending frames together never exceed `MAX_PENDING_BYTES = 2 × MAX_FRAME_SIZE = 32 MiB`: while they do and more than one frame is pending, the oldest pending frame is declared lost, with any missing frames before it. A full-size frame at the maximum FEC level needs about 24 MiB, so a single frame always fits.
- After a lost frame the receiver waits for a keyframe: non-key video frames are dropped, config frames still pass, the next keyframe clears the wait. While it waits, Feedback carries "keyframe needed" once a video frame has been lost or dropped; a lost frame that was never seen at all counts as a video frame.
- A lost config frame is asked for again. A lost frame that was a config frame, or was never seen at all and so may have been one, makes Feedback carry "keyframe needed" until a config frame is delivered, even after a keyframe has cleared the wait, because the server answers every such request with the current config frame followed by a keyframe.
- The `highest_frame` of Feedback is the last frame the receiver has finished with, delivered or declared lost (0 before any). It never names a frame that is still pending, so a "keyframe needed" whose `highest_frame` is older than the last keyframe the server sent is known to predate that keyframe, and the server ignores it.
- Shards of delivered or lost frames and duplicate shards are ignored.

PC side:
1. Hello from an address creates a session (random non-zero id), replacing any older session of that address; a repeated Hello with the same nonce just repeats Welcome.
2. The session joins the hub (config + GOP snapshot first, then live frames), asks the hub for the Hello's mode and codecs, and streams config and video frames to that address through its paced sender.
3. A session without any datagram from its address for `SESSION_TIMEOUT = 3 s` ends; leaving releases the hub viewer, so capture stops after the existing 5 s grace.
4. On shutdown every session gets Bye.
5. A session that falls behind the hub broadcast skips to the next keyframe.
6. Feedback with "keyframe needed" makes the next frame for that session wait for a keyframe, and the session re-sends the current config frame before that keyframe; with the native backend it also asks the encoder for an IDR at once.
7. Feedback or Stats carrying a session id the server does not know (the server restarted, or the session timed out) is answered with a Bye carrying that session id, so the TV reconnects at once instead of waiting for its silence timer.
8. On Windows the socket is created with `SIO_UDP_CONNRESET` off: an ICMP "port unreachable" caused by one vanished TV must not fail reads and sends for the others.

### Discovery (unchanged messages, new socket)

The TV broadcasts `{"type":"discover","version":1}` to UDP 3278; the server answers from the same socket `{"type":"server","version":1,"name":"<SERVER_NAME>","port":3278}`. The client then sends Hello to the replying address and port.

## Server components

- `packages/wire` — pure protocol logic shared with the TV: packets, layout, GF(256), Reed–Solomon, packetizer, frame receiver, loss meter, FEC controller, pacing schedule. No I/O.
- `apps/server/src/stream` — the UDP endpoint replacing `web` and `discovery`: one `std::net::UdpSocket` on `0.0.0.0:3278`, a receive thread, a sender thread per session, Bye on shutdown. `WEB_HOST` and `WEB_PORT` settings are removed; the port is fixed like discovery.
- `packages/desktop` (Windows) — output enumeration, D3D11 device on the output's adapter, Desktop Duplication, pointer shape, own HLSL conversion.
- `packages/nvenc` (Windows) — encoder session on a D3D11 device. A small C layer of our own, compiled by `cc` against the vendored `nvEncodeAPI.h` (nv-codec-headers 13.0, MIT), talks to `nvEncodeAPI64.dll`; Rust calls that layer through a handful of plain functions. Every NVENC struct is filled in C against the real header, so no struct layout is transcribed by hand.
- Capture backend `native` in `apps/server/src/capture` — Duplication → Converter → Encoder → Annex B to length-prefixed AU + avcC/hvcC record → hub; keyframe requests from the hub force an IDR. The ffmpeg backend stays and is used when no NVIDIA adapter drives the captured output or when `CAPTURE_BACKEND=ffmpeg`.

### Colour conversion

Input: the duplicated desktop texture, `B8G8R8A8_UNORM`, gamma-encoded sRGB values used as BT.709 R'G'B'.
- `Y' = 0.2126 R' + 0.7152 G' + 0.0722 B'`, `Pb = (B' − Y') / 1.8556`, `Pr = (R' − Y') / 1.5748`.
- 8-bit limited range: `Y = 16 + 219 Y'`, `Cb = 128 + 224 Pb`, `Cr = 128 + 224 Pr`, rounded.
- 10-bit limited range (P010, value in the top 10 bits of 16): `Y = 64 + 876 Y'`, `Cb = 512 + 896 Pb`, `Cr = 512 + 896 Pr`.
- Chroma siting "left" (H.264/HEVC default type 0): horizontal taps `[1/4, 1/2, 1/4]` on columns `2x−1, 2x, 2x+1`, vertical average of rows `2y, 2y+1`.
- The cursor is composited before conversion: colour cursors alpha-blended, monochrome and masked-colour cursors with AND/XOR semantics.

### NVENC settings

- Tuning ultra-low-latency, preset P4 by default; rate control CBR, VBV of one frame (`bitrate / fps`).
- Infinite GOP, no B-frames, IDR only at start and on request, SPS/PPS (and VPS) repeated with every IDR.
- VUI: BT.709 primaries, transfer and matrix, limited range.
- HEVC Main or Main10 and H.264 High in Phase 2, AV1 in Phase 3 if the TV decodes it; Annex B output; synchronous encode on the capture thread; bitrate changed live with `nvEncReconfigureEncoder` without an IDR.

## TV components

- `apps/client/native` — Rust `cdylib` `libfastcast_native.so` (crate `fastcast-native`), uses `wire`. Owns the socket thread (plain `recv_from` in Phase 1, `recvmmsg` batches in Phase 4), Hello retries, frame receiver, feedback timer and a bounded queue of delivered frames. JNI surface in `dev.fastcast.client.stream.NativeLink`.
- Kotlin `NativeStreamClient` replaces the OkHttp `StreamClient` and keeps the `StreamListener` contract (`onConnecting`, `onConnected`, `onConfig`, `onFrame`, `onHealthy`, `onDisconnected`), reusing `ReconnectBackoff`, `SilenceTimer`, `StreamHealth`. OkHttp stays only for updates.
- Display mode: prefer the largest picture, then the highest refresh rate up to 120 Hz that the chosen decoder can sustain at that size (`VideoCapabilities.areSizeAndRateSupported`); `Surface.setFrameRate` on API 30+.

## Testing policy

- `wire` is pure and covered by unit tests, including randomized loss.
- Hardware tests (GPU, NVENC, Desktop Duplication) carry `#[ignore = "<reason>"]` and run locally with `cargo nextest run -p <crate> --run-ignored only`. They only read the current monitor; nothing in them switches displays.
- The existing `display` test that turns the monitors off is never run by agents.
- The server endpoint is tested in-process on loopback with ephemeral ports, including a lossy relay that drops datagrams.
- The TV is verified by the user after the release reaches it; adb is used read-only.

## Phases

1. Transport: `wire`, server UDP endpoint, Rust receiver on the TV, Kotlin integration, removal of WebSocket and HTTP. Capture stays ffmpeg. Shipped as one release.
2. Native capture and encode: `desktop`, `nvenc`, native backend with on-demand IDR and the automatic ffmpeg fallback. `nvenc` can already change the bitrate live; its caller arrives with the phase 4 congestion controller.
3. 4K 120: display mode and decoder selection up to 120 Hz, virtual display at 120 Hz, 10-bit when supported, bitrate defaults for 4K 120, NVENC preset P1 at 4K 120 (on the RTX 5070 P4 takes 7.4 ms of the 8.3 ms frame, P1 3.6 ms).
4. Optimizations: SIMD GF(256) (AVX2, NEON; scalar parity runs at about 146 MB/s on one core, so a 2 MB key frame costs 14 ms), UDP segmentation offload, sub-frame slices, delay-based congestion control that sets the encoder bitrate live, latency telemetry overlay, the AVC VUI bitstream restriction (no reordered frames) so Android decoders release every frame at once.

Work that does not depend on another phase may start early (the Phase 2 crates are built in parallel with Phase 1).
