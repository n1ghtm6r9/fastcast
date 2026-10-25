# Server UDP Endpoint Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace the WebSocket/HTTP server (`apps/server/src/web`) and the separate discovery responder (`apps/server/src/discovery`) with one UDP endpoint on port 3278 (`apps/server/src/stream`) that answers discovery probes and streams the hub to TV sessions with the `wire` protocol: FEC, pacing, feedback-driven FEC level, keyframe requests.

**Architecture:** One `std::net::UdpSocket` on `0.0.0.0:3278` shared through `Arc`. A receive thread owns the session table: it answers discovery probes, starts a session per Hello, applies Feedback, logs Stats, ends sessions on Bye or silence. Every session has its own sender thread that joins the hub, admits frames (keyframe wait, config re-send), cuts them into datagrams with `wire::Packetizer` and paces them with `wire::burst_sizes`. All datagrams of a session are written by its sender thread, so its sequence numbers stay consecutive. The tokio runtime keeps running the capture supervisor only.

**Tech Stack:** Rust 2024, `std::net::UdpSocket`, `std::thread`, `tokio::sync` (hub broadcast, `Notify`) driven by a per-thread current-thread runtime, `wire` (workspace crate).

**Spec:** `docs/superpowers/specs/2026-10-26-native-pipeline-design.md` — sections "Wire protocol v1" (Session, PC side), "Discovery", "Server components", "Testing policy".

## Global Constraints

- One item per file (type, function, constant, or the `impl` of one method), file name = item name in snake_case; module layout `constants/ services/ types/ test/` like `apps/server/src/hub/`. `mod.rs` of a feature module: `pub mod constants; mod services; mod types;`, then `pub use constants::*; pub use services::*; pub use types::*;`, then `#[cfg(test)] mod test;`. Inner `mod.rs`: `mod x;` lines then `pub use x::*;` lines, alphabetical.
- One test per file named `<sentence>_test.rs`, the test function is the sentence. Test helpers are their own files in `test/` (unit) or `tests/stream/support/` (integration).
- No comments anywhere. Imports at the top. Errors are enums with hand-written `Display` and `Error`.
- No new crates. `wire` is already a dependency of `app`. Remove `axum`, `futures-util`, `tokio-tungstenite` and `tower` (Task 6).
- Constants from the spec, verbatim: discovery and stream port `3278`; `SESSION_TIMEOUT = 3 s`, `BASE_FEC_PERCENT = 10`, `PACE_TICK = 500 µs`, `SHARD_SIZE = 1370` come from `wire`, never redefined.
- Never run the server binary, never leave anything listening after a test, never bind port 3278 in tests: tests bind `127.0.0.1:0`.
- The ignored `display` test (it switches the monitors off) is never run: no `--run-ignored` for `-p app`.
- Bash: no `cd`; `cargo nextest run -p app`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all --check` from the repo root.
- Commit after each task: the task's one-line imperative subject, no body. Never push.

## Review Focus

1. A TV that keeps sending Feedback after the server restarted must reconnect quickly: Feedback or Stats for an unknown session is answered with Bye carrying that session id. Test: Task 5 `feedback_for_an_unknown_session_is_answered_with_bye`.
2. Windows reports ICMP "port unreachable" from an earlier `send_to` as `ConnectionReset` on the next `recv_from` or `send_to` of the same socket, so one vanished TV could break reads and sends of every other session: the socket is created with `SIO_UDP_CONNRESET` off, and the receive thread still treats `ConnectionReset` as harmless. Tests: Task 1 `a_send_to_a_closed_port_does_not_reset_the_stream_socket`, Task 5 `the_endpoint_keeps_serving_after_a_viewer_vanished`.
3. A TV whose keyframe is still in flight keeps flagging "keyframe needed" every 20 ms; the server must not answer each flag with another keyframe request. Test: Task 2 `a_key_frame_need_reported_before_the_last_key_frame_arrived_is_ignored`.
4. A long GOP (native encoder, infinite GOP) must not be replayed to a joining TV: more than `MAX_REPLAY_FRAMES` frames → skip the replay and ask the hub for a keyframe. Test: Task 4 `a_session_joining_a_long_gop_asks_for_a_key_frame_instead_of_replaying_it`.
5. Stopping the endpoint must not hang on a session that is waiting for frames that never come. Test: Task 5 `stopping_the_endpoint_says_bye_to_every_session` (no frames published, stop returns within 1 s).

---

## File Structure

```
apps/server/src/stream/mod.rs
apps/server/src/stream/constants/   STREAM_PORT, LISTEN_IP, READ_TIMEOUT, RECEIVE_BUFFER_BYTES, MAX_PROBE_BYTES,
                                    PROBE_KIND, SERVER_REPLY_KIND, DISCOVERY_VERSION, VIEWER_REPORT_LIMIT,
                                    MAX_REPLAY_FRAMES, DISPLAY_SIDE_RANGE, DISPLAY_RATE_RANGE, FALLBACK_STREAM_FPS
apps/server/src/stream/services/    stream_address, bind_stream_socket, is_discovery_probe, server_reply_json,
                                    clipped, codecs_of, viewer_request_of, new_session_id, frame_interval_of,
                                    send_paced, next_hub_event, run_session, run_receiver
apps/server/src/stream/types/       StreamError, ServerReply, EndpointOptions, Admission, SessionCursor (+ method files),
                                    SessionControl (+ method files), HubEvent, SessionEntry, StreamEndpoint (+ method files)
apps/server/src/stream/test/        unit tests
apps/server/tests/stream/           integration tests (main.rs, support/, one file per test)
apps/server/src/lib.rs              pub mod stream; web and discovery removed (Task 6)
apps/server/src/app/                run.rs wiring, AppError::Stream, log_stream_address (Task 6)
apps/server/src/settings/           WEB_HOST and WEB_PORT removed (Task 6)
README.md, .env.example             updated (Task 6)
```

---

### Task 1: Stream module basics

**Files:** Create `apps/server/src/stream/{mod.rs,constants/*,services/*,types/*,test/*}` for the items below; modify `apps/server/src/lib.rs` (add `pub mod stream;`, keep `web` and `discovery` for now); modify `apps/server/Cargo.toml` (add the `Win32_Networking_WinSock` feature to `windows`).

**Interfaces — Consumes:** `wire::{CodecSupport, Codec, Hello}`, `crate::hub::ViewerRequest`, `crate::display::DisplayMode`, `media::VideoCodec`. **Produces:**
```rust
pub const STREAM_PORT: u16 = 3278;
pub const LISTEN_IP: Ipv4Addr = Ipv4Addr::UNSPECIFIED;
pub const READ_TIMEOUT: Duration = Duration::from_millis(50);
pub const RECEIVE_BUFFER_BYTES: usize = 2048;
pub const MAX_PROBE_BYTES: usize = 1024;
pub const PROBE_KIND: &str = "discover";
pub const SERVER_REPLY_KIND: &str = "server";
pub const DISCOVERY_VERSION: u32 = 1;
pub const VIEWER_REPORT_LIMIT: usize = 600;
pub const MAX_REPLAY_FRAMES: usize = 30;
pub const DISPLAY_SIDE_RANGE: RangeInclusive<u32> = 320..=7680;
pub const DISPLAY_RATE_RANGE: RangeInclusive<u32> = 1..=240;
pub const FALLBACK_STREAM_FPS: u32 = 60;
pub const SEND_BUFFER_BYTES: i32 = 4 * 1024 * 1024;
pub enum StreamError { Bind { address: SocketAddr, source: io::Error }, Socket { source: io::Error } }
pub struct ServerReply<'a> { pub kind: &'a str, pub version: u32, pub name: &'a str, pub port: u16 }
pub fn stream_address(port: u16) -> SocketAddr;
pub fn bind_stream_socket(address: SocketAddr) -> Result<UdpSocket, StreamError>;
pub fn tune_stream_socket(socket: &UdpSocket) -> io::Result<()>;
pub fn is_discovery_probe(datagram: &[u8]) -> bool;
pub fn server_reply_json(name: &str, port: u16) -> String;
pub fn clipped(text: &str, limit: usize) -> &str;
pub fn codecs_of(codecs: &[CodecSupport]) -> Vec<VideoCodec>;
pub fn viewer_request_of(hello: &Hello, default_fps: u32) -> ViewerRequest;
pub fn new_session_id(avoid: u32) -> u32;
pub fn frame_interval_of(fps: u32) -> Duration;
```
Rules:
- `StreamError` `Display`: `Bind` → `could not listen on udp {address}: {source} (is another fastcast already running?)`; `Socket` → `could not set up the stream socket: {source}`. `source()` returns the `io::Error`.
- `bind_stream_socket` binds, then calls `tune_stream_socket` (its error → `StreamError::Socket`). `tune_stream_socket` on Windows (`services/tune_stream_socket.rs`, `#[cfg(windows)]`): `WSAIoctl(SIO_UDP_CONNRESET)` with a `FALSE` `u32` input, then `setsockopt(SOL_SOCKET, SO_SNDBUF, SEND_BUFFER_BYTES)`, through `windows::Win32::Networking::WinSock` on `SOCKET(socket.as_raw_socket() as usize)`; `SOCKET_ERROR` → `io::Error::last_os_error()`. Elsewhere (`services/tune_stream_socket_unsupported.rs`, `#[cfg(not(windows))]`) it returns `Ok(())`; the `mod` lines follow the `saved_path_dirs` pattern in `capture/services/mod.rs`.
- `ServerReply` serializes with `#[serde(rename = "type")]` on `kind`, exactly like `discovery::ServerReply` today. `is_discovery_probe`, `server_reply_json` and `clipped` keep the exact behaviour of `discovery::is_discovery_probe`, `discovery::server_reply_json` and `web::clipped` (copy them; the originals go away in Task 6).
- `codecs_of`: `Codec::Avc` → `VideoCodec::Avc`, `Codec::Hevc` → `VideoCodec::Hevc`, `Codec::Av1` skipped (no AV1 capture yet); order kept, duplicates dropped. `max_bit_depth` is ignored in this plan.
- `viewer_request_of`: `codecs = codecs_of(&hello.codecs)`; `mode = None` when `width` or `height` is 0 or outside `DISPLAY_SIDE_RANGE`; otherwise `refresh_hz = fps` (or `default_fps` when `fps == 0`) and `mode = None` when `refresh_hz` is outside `DISPLAY_RATE_RANGE`.
- `new_session_id`: random, never 0, never `avoid`. No new crates: hash a fresh `std::collections::hash_map::RandomState` hasher fed with `Instant`-based nanos and a static `AtomicU32` counter, take the low 32 bits, retry on 0 or `avoid`.
- `frame_interval_of(fps)` = `Duration::from_secs(1) / fps`, with `FALLBACK_STREAM_FPS` when `fps == 0`.

- [ ] **Step 1: Write the failing tests** in `apps/server/src/stream/test/`, one file each:
  - `a_bind_failure_names_the_udp_address_and_the_cause` — `StreamError::Bind { address: "0.0.0.0:3278".parse(), source: io::Error::new(AddrInUse, "taken") }.to_string()` starts with `could not listen on udp 0.0.0.0:3278: taken`.
  - `binding_port_zero_picks_a_free_udp_port` — `bind_stream_socket("127.0.0.1:0")` → `local_addr().port() != 0`.
  - `a_taken_udp_port_is_reported_with_the_address_that_was_asked_for` — bind a socket on `127.0.0.1:0`, bind again on its address → `Err(StreamError::Bind { address, .. })` with that address.
  - `the_stream_listens_on_every_ipv4_interface_at_the_given_port` — `stream_address(3278) == "0.0.0.0:3278"`.
  - `the_stream_port_is_3278_because_the_tv_probes_only_that_port` — `STREAM_PORT == 3278`.
  - `a_send_to_a_closed_port_does_not_reset_the_stream_socket` — `bind_stream_socket("127.0.0.1:0")` with a 300 ms read timeout; bind a second socket, note its address, drop it; `send_to` that address twice (both `Ok`); `recv_from` fails with `WouldBlock` or `TimedOut`, never `ConnectionReset`.
  - `a_json_object_of_type_discover_is_a_probe_whatever_else_it_carries`, `anything_that_is_not_json_is_not_a_probe`, `json_of_another_shape_or_type_is_not_a_probe` — copies of the discovery tests of the same names against `stream::is_discovery_probe`; `a_probe_of_exactly_1024_bytes_is_a_probe` and `a_probe_one_byte_over_the_limit_is_not_a_probe` — the lengths of the discovery tests `a_probe_of_exactly_1024_bytes_is_answered` and `a_probe_one_byte_over_the_limit_gets_no_reply` checked with `is_discovery_probe` (copy the helpers `probe_of_length.rs` and `valid_probe.rs`).
  - `the_reply_is_the_server_type_the_protocol_version_the_name_and_the_port` — `server_reply_json("KOMPUTER", 3278)` parses to `{"type":"server","version":1,"name":"KOMPUTER","port":3278}`.
  - `a_name_with_quotes_and_cyrillic_letters_stays_valid_json` — `server_reply_json("Мой \"ПК\"", 3278)` parses and its `name` round-trips.
  - `a_long_report_is_clipped_on_a_character_boundary`, `a_short_report_is_kept_whole` — copies of the `web` tests against `stream::clipped`.
  - `a_hello_with_a_mode_asks_for_that_mode_and_its_codecs` — Hello 3840×2160@120, codecs `[Hevc/10, Avc/8]` → `ViewerRequest { mode: Some(DisplayMode { width: 3840, height: 2160, refresh_hz: 120 }), codecs: vec![Hevc, Avc] }`.
  - `a_hello_without_a_size_asks_for_the_configured_monitor` — width 0, height 0, fps 60 → `mode: None`.
  - `a_hello_without_a_rate_asks_for_the_default_rate` — 1920×1080, fps 0, default 60 → `refresh_hz: 60`.
  - `a_mode_outside_the_supported_range_asks_for_no_mode` — 100×100@60 → `None`; 1920×1080@241 → `None`.
  - `av1_and_repeated_codecs_are_left_out` — `[Av1, Hevc, Hevc, Avc]` → `[Hevc, Avc]`.
  - `a_session_id_is_never_zero_nor_the_avoided_one` — 10 000 calls of `new_session_id(7)`: none is 0 or 7; at least 9 990 distinct.
  - `the_frame_interval_is_one_second_over_the_rate` — 60 → 16 666 666 ns (`Duration::from_secs(1) / 60`), 120 → `Duration::from_secs(1) / 120`.
  - `a_rate_of_zero_falls_back_to_sixty` — 0 → `Duration::from_secs(1) / 60`.
- [ ] **Step 2: Run** `cargo nextest run -p app stream::` — expected: compile errors.
- [ ] **Step 3: Implement** the items above.
- [ ] **Step 4: Run** — expected: all new tests pass, all old ones still pass.
- [ ] **Step 5: Commit** `Add the stream module with discovery replies and hello parsing`.

### Task 2: Session cursor and session control

**Files:** Create `apps/server/src/stream/types/{admission.rs,session_cursor.rs,session_cursor_admit.rs,session_cursor_lagged.rs,session_cursor_resync.rs,session_cursor_is_waiting.rs,session_control.rs,session_control_*.rs}` and tests.

**Interfaces — Consumes:** `crate::hub::{Frame, StreamConfig}`, `wire::{Feedback, BASE_FEC_PERCENT}`. **Produces:**
```rust
pub enum Admission { Skip, Send, SendWithConfig(Arc<StreamConfig>) }
pub struct SessionCursor { pub(crate) sent_config: Option<Arc<StreamConfig>>, pub(crate) waiting_for_key_frame: bool }
impl SessionCursor {
    pub fn starting_with(sent_config: Option<Arc<StreamConfig>>) -> SessionCursor;
    pub fn admit(&mut self, frame: &Frame) -> Admission;
    pub fn lagged(&mut self);
    pub fn resync(&mut self);
    pub fn is_waiting(&self) -> bool;
}
pub struct SessionControl { stop: AtomicBool, welcome_wanted: AtomicBool, key_frame_needed: AtomicBool, peer_highest_frame: AtomicU32, fec_percent: AtomicU8, wake: tokio::sync::Notify }
impl SessionControl {
    pub fn new() -> SessionControl;
    pub fn stop(&self);
    pub fn is_stopped(&self) -> bool;
    pub fn want_welcome(&self);
    pub fn take_welcome(&self) -> bool;
    pub fn report(&self, feedback: &Feedback, fec_percent: u8);
    pub fn take_key_frame_need(&self, last_key_frame_sent: Option<u32>) -> bool;
    pub fn fec_percent(&self) -> u8;
    pub async fn woken(&self);
}
```
Rules:
- `SessionCursor::admit` and `lagged` are `web::ViewerCursor::admit`/`lagged` unchanged. `resync` sets `waiting_for_key_frame = true` and `sent_config = None`, so the next key frame is preceded by its config. `is_waiting` returns `waiting_for_key_frame`.
- `SessionControl::new`: everything false/0, `fec_percent = BASE_FEC_PERCENT`. `stop` and `want_welcome` set their flag and call `wake.notify_one()`. `take_welcome` swaps the flag to false and returns the old value. `report` stores `feedback.highest_frame` and `fec_percent`; when `feedback.keyframe_needed` it sets `key_frame_needed` and calls `wake.notify_one()`, so a sender waiting on a still screen acts at once. `take_key_frame_need(last)` swaps `key_frame_needed` to false and returns true only if it was set and (`last` is `None` or `peer_highest_frame >= last`). `woken` awaits `wake.notified()`. `SessionControl` implements `Default`.

- [ ] **Step 1: Write the failing tests**: move every `ViewerCursor` test from `apps/server/src/web/test/` to `apps/server/src/stream/test/` against `SessionCursor` (keep their names; copy the helpers `frame_under.rs`, `shared_nvenc_config.rs`, `shared_x264_config.rs` into `stream/test/`); the `web` copies stay until Task 6. New tests:
  - `a_resynced_cursor_announces_the_config_again_before_the_next_key_frame` — cursor that already sent config C and a key frame under C; `resync()`; a delta under C → `Skip`; a key frame under C → `SendWithConfig(C)`.
  - `a_fresh_control_streams_with_the_base_fec_level` — `SessionControl::new().fec_percent() == 10`.
  - `a_requested_welcome_is_taken_once` — `want_welcome()`; `take_welcome()` true then false.
  - `a_stopped_control_wakes_its_sender` — a current-thread runtime `block_on(control.woken())` returns after `stop()` called before the wait (the permit is kept); `is_stopped()` is true.
  - `a_key_frame_need_before_any_key_frame_is_taken` — `report(feedback with keyframe_needed, 10)`; `take_key_frame_need(None)` true; again → false.
  - `a_key_frame_need_reported_before_the_last_key_frame_arrived_is_ignored` — `report(keyframe_needed, highest_frame 40)`; `take_key_frame_need(Some(41))` false; the flag is cleared (`take_key_frame_need(None)` false).
  - `a_key_frame_need_after_the_last_key_frame_is_taken` — highest 41, last 41 → true.
  - `feedback_sets_the_fec_level_the_sender_uses` — `report(_, 27)` → `fec_percent() == 27`.
  - `a_key_frame_need_wakes_the_sender` — `report(keyframe_needed, 10)` before the wait → `block_on(control.woken())` returns; `is_stopped()` stays false.
- [ ] **Step 2: Run** — expected: compile errors.
- [ ] **Step 3: Implement.**
- [ ] **Step 4: Run** — expected: pass.
- [ ] **Step 5: Commit** `Track what each stream session was sent and asked for`.

### Task 3: Session sender

**Files:** Create `apps/server/src/stream/types/hub_event.rs`, `apps/server/src/stream/services/{send_paced.rs,next_hub_event.rs,run_session.rs}` and tests.

**Interfaces — Consumes:** Task 1 and 2 items; `wire::{Packetizer, Datagrams, OutgoingFrame, FrameKind, Packet, Welcome, burst_sizes, PACE_TICK, SHARD_SIZE}`; `crate::hub::{Hub, Frame, Subscription, ViewerRequest}`. **Produces:**
```rust
pub enum HubEvent { Frame(Arc<Frame>), Lagged, Closed, Woken }
pub async fn next_hub_event(frames: &mut broadcast::Receiver<Arc<Frame>>, control: &SessionControl) -> HubEvent;
pub fn send_paced(socket: &UdpSocket, peer: SocketAddr, datagrams: &Datagrams, frame_interval: Duration);
pub fn run_session(socket: Arc<UdpSocket>, peer: SocketAddr, session: u32, nonce: u32, hub: Arc<Hub>, request: ViewerRequest, control: Arc<SessionControl>);
```
Behaviour:
- `next_hub_event`: `tokio::select! { biased; () = control.woken() => Woken, result = frames.recv() => Frame / Lagged / Closed }`.
- `send_paced`: `bursts = burst_sizes(datagrams.len(), frame_interval)`, `start = Instant::now()`; burst `i` waits with `std::thread::sleep` until `start + PACE_TICK * i`, then `send_to`s its datagrams in order. Send errors are logged at `debug!` and skipped.
- `run_session` (body of the session's thread, returns when the session ends):
  1. Build a `tokio::runtime::Builder::new_current_thread().build()` runtime (on error `warn!` and return).
  2. `Packetizer::new(session)`; send `Welcome { nonce, shard_size: SHARD_SIZE as u16 }` with `packetizer.control`.
  3. `hub.request(request)`; `let Subscription { config, gop, frames, guard } = hub.join();` keep `guard` until the function returns.
  4. `cursor = SessionCursor::starting_with(config.clone())`; if `config` is `Some`, send it as a config frame (`OutgoingFrame { kind: FrameKind::Config, keyframe: false, payload: config.json.as_bytes() }`).
  5. If `1 ..= MAX_REPLAY_FRAMES` frames are in `gop`, send each admitted one; otherwise call `hub.request_key_frame()` and skip the replay.
  6. Loop: if `control.is_stopped()` leave the loop; if `control.take_welcome()` send Welcome again; if `control.take_key_frame_need(last_key_frame)` (always called, so the flag is consumed) and `!cursor.is_waiting()` → `cursor.resync()` and `hub.request_key_frame()`; then `match runtime.block_on(next_hub_event(&mut frames, &control))`:
     - `Frame(frame)`: `cursor.admit(&frame)`: `Skip` → nothing; `Send` → send the video frame; `SendWithConfig(c)` → send config `c`, then the video frame. A video frame is `OutgoingFrame { kind: FrameKind::Video, keyframe: frame.key, payload: &frame.wire }`; when `frame.key`, remember the returned frame number as `last_key_frame`.
     - `Lagged` → `cursor.lagged()` and `hub.request_key_frame()`.
     - `Woken` → continue. `Closed` → leave the loop.
  7. Send Bye with `packetizer.control(&Packet::Bye, …)`, drop the guard, return.
  Every frame goes through `packetizer.frame(&outgoing, control.fec_percent(), &mut datagrams)` and `send_paced(…, frame_interval_of(stream fps))`, the stream fps being `frame.config.fps` for video and `config.fps` for config frames.

- [ ] **Step 1: Write the failing tests** in `apps/server/src/stream/test/` (bind a receiving `UdpSocket` on `127.0.0.1:0` with a 2 s read timeout, decode with `wire::decode_datagram`):
  - `paced_datagrams_reach_the_peer_in_order` — 40 datagrams from a `Packetizer` frame of 40 × 1370 bytes, interval 1/60 s → all 40 arrive, sequences strictly increasing.
  - `pacing_spreads_a_frame_over_half_its_interval` — 64 datagrams, interval 1/60 s → 16 bursts, the last one due 7.5 ms after the start: `send_paced` takes at least 7 ms (sleeps never end early) and less than 50 ms (slack for CI runners).
  - `a_session_says_welcome_then_sends_the_config_and_the_last_gop` — hub with the x264 config, key + 2 deltas published; `run_session` with `ViewerRequest::default()` (equal to the hub's current request, so the hub keeps its config) on a thread, `control.stop()` after 500 ms → the peer gets Welcome (nonce echoed, session id as given), then the shards of frame 0 (config, payload = config JSON), frames 1–3 (video, payload = the frames' wire bytes, frame 1 keyframe), then Bye; reassemble with `wire::FrameReceiver`.
  - `a_session_joining_a_long_gop_asks_for_a_key_frame_instead_of_replaying_it` — key + 30 deltas published (31 frames) → after Welcome and config nothing but Bye arrives; `hub.take_key_frame_request()` is true.
  - `a_stopped_session_leaves_the_hub` — `hub.viewer_count()` is 1 while running, 0 after `control.stop()` and the thread's join.
- [ ] **Step 2: Run** — expected: compile errors.
- [ ] **Step 3: Implement.**
- [ ] **Step 4: Run** — expected: pass.
- [ ] **Step 5: Commit** `Send each stream session its frames paced over half the frame interval`.

### Task 4: Receive thread and the endpoint

**Files:** Create `apps/server/src/stream/types/{endpoint_options.rs,endpoint_options_new.rs,session_entry.rs,stream_endpoint.rs,stream_endpoint_start.rs,stream_endpoint_local_addr.rs,stream_endpoint_stop.rs,stream_endpoint_drop.rs}`, `apps/server/src/stream/services/run_receiver.rs`; create `apps/server/tests/stream/main.rs`, `apps/server/tests/stream/support/*`, the integration tests below.

**Interfaces — Consumes:** Tasks 1–3; `wire::{decode_datagram, encode_datagram, Datagram, Packet, Feedback, LossMeter, FecController, SESSION_TIMEOUT}`. **Produces:**
```rust
pub struct EndpointOptions { pub server_name: String, pub default_fps: u32, pub session_timeout: Duration }
impl EndpointOptions { pub fn new(server_name: String, default_fps: u32) -> EndpointOptions; }
pub struct SessionEntry { pub id: u32, pub nonce: u32, pub last_seen: Instant, pub control: Arc<SessionControl>, pub loss: LossMeter, pub fec: FecController, pub thread: JoinHandle<()> }
pub struct StreamEndpoint { local: SocketAddr, stop: Arc<AtomicBool>, receiver: Option<JoinHandle<()>> }
impl StreamEndpoint {
    pub fn start(socket: UdpSocket, hub: Arc<Hub>, options: EndpointOptions) -> Result<StreamEndpoint, StreamError>;
    pub fn local_addr(&self) -> SocketAddr;
    pub fn stop(self);
}
impl Drop for StreamEndpoint
pub fn run_receiver(socket: Arc<UdpSocket>, hub: Arc<Hub>, options: EndpointOptions, stop: Arc<AtomicBool>);
```
Behaviour:
- `EndpointOptions::new` sets `session_timeout = wire::SESSION_TIMEOUT`.
- `start`: `set_read_timeout(Some(READ_TIMEOUT))` and `local_addr()` (errors → `StreamError::Socket`), spawn the thread `fastcast-stream` running `run_receiver`. `stop` sets the flag and joins the thread; `Drop` does the same when `stop` was not called. `stop` returns within `READ_TIMEOUT` plus the time the sessions need to send Bye.
- `run_receiver`: `reply = server_reply_json(&options.server_name, local port)`; `sessions: HashMap<SocketAddr, SessionEntry>`, `retired: Vec<JoinHandle<()>>`. Loop until the stop flag is set: `recv_from` into a `[u8; RECEIVE_BUFFER_BYTES]` buffer; `WouldBlock`, `TimedOut` and `ConnectionReset` are not errors (go on), other errors are logged at `debug!`. Handle the datagram, then end sessions whose `last_seen` is older than `session_timeout`, then join the `retired` threads that `is_finished()`. On stop: end every session and join every thread.
- Datagram handling (peer = sender address, now = `Instant::now()`):
  - First byte `{`: if `is_discovery_probe`, `send_to(reply, peer)`; nothing else.
  - `decode_datagram` → `None`: ignore.
  - `Hello`: the peer's session with the same nonce → `last_seen = now`, `control.want_welcome()`. Otherwise end the peer's old session (if any), `id = new_session_id(old id or 0)`, spawn thread `fastcast-session` with `run_session(…, viewer_request_of(&hello, options.default_fps), …)`, store the entry with `LossMeter::new()` and `FecController::new(now)`, `info!` the peer, session id (`{id:08x}`), mode and codecs.
  - `Feedback`, `Stats`, `Bye` whose `datagram.session` is the peer's session id → `last_seen = now`; Feedback → `if let Some(loss) = entry.loss.update(&feedback) { entry.fec.observe(loss, now) }`, then `entry.control.report(&feedback, entry.fec.percent())`; Stats → `info!("viewer report: {}", clipped(text, VIEWER_REPORT_LIMIT))`; Bye → end the session.
  - `Feedback` or `Stats` with any other session id (or from a peer without a session) → answer with a Bye datagram carrying that session id and sequence 0.
  - Anything else: ignore.
- Ending a session: `control.stop()`, move its `thread` to `retired`, `info!` why (`left`, `went silent`, `replaced`, `server stopping`).

Integration test support in `apps/server/tests/stream/support/` (one item per file): `start_endpoint(hub: Arc<Hub>, session_timeout: Duration) -> StreamEndpoint` (binds `127.0.0.1:0` through `bind_stream_socket`, name `test-pc`, default fps 60); `TestViewer` with `connect(server: SocketAddr, hello: Hello) -> TestViewer` (binds `127.0.0.1:0`, resends Hello every 100 ms until a Welcome with its nonce, panics after 2 s), `session(&self) -> u32`, `next_frame(&mut self, timeout: Duration) -> Option<ReceivedFrame>` (decodes datagrams, records sequences in `ReceiveCounters`, feeds shards of its session to a `FrameReceiver`, polls it), `next_datagram(&self, timeout: Duration) -> Option<Vec<u8>>`, `feedback(&self, keyframe_needed: bool) -> Feedback` (from its counters and receiver), `send(&mut self, packet: &Packet<'_>)` (its session id, its own sequence from 0), `send_feedback(&mut self, keyframe_needed: bool)`, `send_raw(&self, bytes: &[u8])`, `local_addr(&self) -> SocketAddr`; `a_bare_hello() -> Hello` (no mode, no codecs: asks the hub for `ViewerRequest::default()`, so a hub with a config keeps it); `a_hello(width: u16, height: u16, fps: u16, codecs: &[Codec]) -> Hello` (bit depth 8); both take the nonce from a static counter; `random_payload(seed: u64, length: usize) -> Vec<u8>` (xorshift; `wire`'s own random helpers are test-only); `hub_with_config() -> Arc<Hub>` and `hub_without_config() -> Arc<Hub>` (x264 1920×1080 from `tests/web/support/x264_avcc.rs`, moved here with `config_from.rs`, `frame_data.rs`, `hex_bytes.rs`, `publish_frame.rs`, `received_frame_of.rs`, `wait_for_request.rs`, `wait_for_viewers.rs`, adapted to `ReceivedFrame` payloads, which carry the same bytes as the old binary messages). `main.rs` declares `mod support;` and one `mod` per test file.

- [ ] **Step 1: Write the failing integration tests**, one file each in `apps/server/tests/stream/`:
  - `a_discovery_probe_is_answered_with_the_server_name_and_the_stream_port` — reply JSON `type server`, `version 1`, `name test-pc`, `port` = endpoint port.
  - `a_datagram_that_is_neither_a_probe_nor_a_packet_gets_no_reply` — `b"hello"`, `[0xFC, 9, 1, 0, …]`, `b"{not json"` → nothing within 200 ms; a valid probe afterwards is answered.
  - `a_hello_is_welcomed_with_its_nonce_and_a_session_id` — Welcome nonce = Hello nonce, `shard_size == 1370`, session ≠ 0.
  - `a_repeated_hello_keeps_the_session_and_repeats_the_welcome` — same nonce twice → two Welcomes, same session id; `hub.viewer_count() == 1`.
  - `a_hello_with_a_new_nonce_starts_a_new_session_for_that_address` — new session id; the old one gets Bye; viewer count settles at 1.
  - `the_session_asks_the_hub_for_the_mode_and_codecs_of_its_hello` — Hello 3840×2160@120 `[Hevc, Avc]` → `hub.watch_requests()` shows that request within 1 s.
  - `a_session_gets_the_config_then_the_frames_since_the_last_key_frame` — published key + 2 deltas before connecting with `a_bare_hello()` → frames 0 (config), 1 (key), 2, 3 with equal payloads.
  - `live_frames_reach_a_session_in_order` — 20 frames published after connecting (key first, 5 ms apart) → all 20 in order.
  - `every_prober_gets_its_reply_at_its_own_address`, `one_probe_gets_exactly_one_reply` — the discovery tests of the same names against the endpoint.
  - Ports of the web integration tests, same scenario and assertions over UDP: `a_video_frame_is_a_flag_byte_a_timestamp_and_the_access_unit` (from `a_binary_message_is_…`), `a_new_config_is_announced_before_its_first_key_frame`, `a_second_viewer_asking_for_the_same_picture_gets_the_stream_at_once`, `a_second_viewer_joining_mid_stream_starts_at_the_last_key_frame`, `a_viewer_asking_for_another_picture_waits_for_the_new_stream`, `a_viewer_that_joins_before_any_config_waits_for_the_stream`, `delta_frames_are_not_delivered_before_a_key_frame`, `the_config_message_describes_the_stream`, `the_viewer_count_follows_sessions` (from `the_viewer_count_follows_connections`), `a_viewer_that_sends_stats_keeps_receiving_frames` (from `a_viewer_that_sends_a_report_…`).
- [ ] **Step 2: Run** `cargo nextest run -p app --test stream` — expected: compile errors.
- [ ] **Step 3: Implement.**
- [ ] **Step 4: Run** — expected: pass.
- [ ] **Step 5: Commit** `Serve discovery and stream sessions from one UDP endpoint`.

### Task 5: Loss, feedback, timeouts and shutdown

**Files:** integration tests in `apps/server/tests/stream/`, a `lossy_relay(server: SocketAddr, drop_every: usize) -> SocketAddr` support item (a thread forwarding viewer → server untouched and server → viewer dropping every `drop_every`-th datagram), fixes in `apps/server/src/stream/`.

- [ ] **Step 1: Write the failing tests**, one file each:
  - `a_frame_of_many_shards_arrives_whole` — one 300 000-byte key frame of `random_payload` → identical payload.
  - `frames_survive_a_relay_that_drops_every_twentieth_datagram` — through `lossy_relay(…, 20)`, 30 key frames of 40 000 random bytes, 20 ms apart, the viewer sending feedback every 20 ms → all 30 arrive intact.
  - `reported_loss_raises_the_parity_of_later_frames` — two hand-built feedbacks (`received_packets` 100 / `highest_sequence` 99, then 190 / 199: 10 % loss → FEC 30 %), wait 200 ms, publish a 50 000-byte key frame → its shards say 37 data and 12 parity shards.
  - `a_session_that_falls_behind_resumes_at_a_key_frame` — port of the web test: `BROADCAST_CAPACITY + 10` deltas published at once after a key frame, then a key frame → the frames after the lag start with that key frame, and `hub.take_key_frame_request()` was set by the lag.
  - `a_datagram_far_over_the_limit_gets_no_reply_and_the_endpoint_goes_on` — 5000 bytes of `{` and spaces → no reply in 200 ms; a valid probe afterwards is answered.
  - `a_session_without_a_key_frame_to_start_from_asks_the_hub_for_one` — config set, empty GOP → `hub.take_key_frame_request()` true within 1 s of connecting.
  - `a_lost_frame_makes_the_session_resend_the_config_before_the_next_key_frame` — connected and received key frame 1; send feedback with `keyframe_needed` and `highest_frame = 1`; within 1 s `hub.take_key_frame_request()` is true; publish a delta then a key → the viewer receives a config frame immediately followed by the key frame, and no delta between them.
  - `a_silent_session_ends_after_the_session_timeout` — timeout 300 ms, viewer sends nothing after Welcome → viewer count 0 within 1 s.
  - `feedback_keeps_a_session_alive` — timeout 300 ms, feedback every 100 ms for 1 s → viewer count still 1.
  - `a_bye_from_the_viewer_ends_its_session` — viewer count 0 within 500 ms.
  - `feedback_for_an_unknown_session_is_answered_with_bye` — feedback with session 12345 from a fresh socket → a Bye datagram with session 12345 within 500 ms.
  - `the_endpoint_keeps_serving_after_a_viewer_vanished` — connect a viewer, publish frames, drop the viewer's socket (its port now answers ICMP unreachable), keep publishing for 200 ms → a second viewer still connects and receives a key frame.
  - `stopping_the_endpoint_says_bye_to_every_session` — two viewers, no frames published; `stop()` returns within 1 s; each viewer gets a Bye with its session id; viewer count 0.
- [ ] **Step 2: Run** — expected: failures where behaviour is missing.
- [ ] **Step 3: Fix** what fails.
- [ ] **Step 4: Run** `cargo nextest run -p app` — expected: pass, run it three times to catch timing flakiness.
- [ ] **Step 5: Commit** `Cover loss, feedback, timeouts and shutdown of the stream endpoint`.

### Task 6: Switch the server to the stream endpoint and remove WebSocket and HTTP

**Files:** Modify `apps/server/src/app/services/run.rs`, `apps/server/src/app/types/app_error.rs`, `apps/server/src/app/services/mod.rs`, `apps/server/src/lib.rs`, `apps/server/src/settings/**`, `apps/server/Cargo.toml`, root `Cargo.toml`, `Cargo.lock`, `README.md`, `.env.example`. Create `apps/server/src/app/services/log_stream_address.rs`. Delete `apps/server/src/web/`, `apps/server/src/discovery/`, `apps/server/tests/web/`, `apps/server/src/app/services/{log_urls.rs,urls_to_open.rs}` and the tests that only cover deleted code.

**Interfaces — Produces:**
```rust
pub enum AppError { Dotenv(DotenvError), Settings(SettingsError), Stream(StreamError) }
pub fn log_stream_address(port: u16);
```
Behaviour:
- `run`: dotenv, logger, settings, hub; `bind_stream_socket(stream_address(STREAM_PORT))?`; `log_stream_address(STREAM_PORT)` (`info!("fastcast is listening on udp {lan ip or 0.0.0.0}:{port}, the TV finds it by itself")`); `log_ffmpeg`; `StreamEndpoint::start(socket, hub.clone(), EndpointOptions::new(settings.server_name.clone(), settings.capture_fps))?`; capture supervisor as today; `shutdown_signal().await`; abort and await the supervisor; `tokio::task::spawn_blocking(move || endpoint.stop()).await` (a join error is logged, not returned).
- `AppError::Stream` displays the `StreamError` as it is; `a_web_error_is_reported_as_it_is_test` becomes `a_stream_error_is_reported_as_it_is_test`.
- Settings: remove `web_host`, `web_port`, `WEB_HOST_SETTING`, `WEB_PORT_SETTING` and their tests; every other setting keeps its behaviour.
- Dependencies: remove `axum` and `futures-util` from `[dependencies]`, `tokio-tungstenite` and `tower` from `[dev-dependencies]` of `apps/server/Cargo.toml`; remove the same four from `[workspace.dependencies]` in the root `Cargo.toml` when no member uses them; `cargo build` updates `Cargo.lock`.
- README (Russian, same tone): line 9 says the server streams over its own UDP protocol; replace the section "Протокол (`ws://<host>:3278/ws`)" with a short "Протокол (UDP 3278)" section: one socket for discovery and video, Hello with the TV's mode and codecs, frames cut into shards with Reed–Solomon parity, paced over half the frame interval, feedback every 20 ms, details in `docs/superpowers/specs/2026-10-26-native-pipeline-design.md`; discovery section: the reply's `port` is the stream port; firewall sentence: only UDP 3278 is needed; remove the `WEB_HOST` / `WEB_PORT` row; remove the finished roadmap item about UDP/FEC transport. `.env.example`: remove the `WEB_HOST` and `WEB_PORT` lines.
- Search the repo (outside `docs/superpowers/`) for `WEB_HOST`, `WEB_PORT`, `/ws`, `api/health`, `axum`, `tungstenite`: nothing may remain.

- [ ] **Step 1: Change `run.rs` and `AppError`**, delete the old modules, fix the compile errors.
- [ ] **Step 2: Run** `cargo nextest run --workspace` — expected: pass (the ignored display test stays ignored); `cargo clippy --workspace --all-targets -- -D warnings` and `cargo fmt --all --check` clean; `cargo build --release -p app` succeeds.
- [ ] **Step 3: Update** README and `.env.example`; run the repo search above.
- [ ] **Step 4: Commit** `Stream over UDP and drop the WebSocket and HTTP server`.
