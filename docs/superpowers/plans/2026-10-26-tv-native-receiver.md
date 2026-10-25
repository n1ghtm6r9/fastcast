# TV Native Receiver Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace the OkHttp WebSocket `StreamClient` on the TV with a Rust UDP link in `libfastcast_native.so` (Hello, frame reassembly with FEC, feedback, Bye) driven from Kotlin through JNI, keeping the `StreamListener` contract so the player and the decoder stay as they are.

**Architecture:** `apps/client/native` gets a `link` module (pure Rust, tested on the host over loopback against a fake server built from `wire`) and JNI exports in `jni`. A link owns one UDP socket and one thread: it says Hello until Welcome, feeds shards to `wire::FrameReceiver`, sends Feedback every 20 ms and at once on loss, and queues delivered frames. Kotlin's `NativeStreamClient` polls the queue from its own reader thread and keeps the reconnect, silence and health logic of today's `StreamClient`.

**Tech Stack:** Rust 2024, `std::net::UdpSocket`, `jni-sys` 0.4.1, `wire`; Kotlin, JUnit 4.

**Spec:** `docs/superpowers/specs/2026-10-26-native-pipeline-design.md` — sections "Session" (TV side, frame receiver rules), "Kinds", "TV components". Builds on `docs/superpowers/plans/2026-10-26-android-native-build.md` (the crate, Gradle packaging and `protocolVersion` already exist).

## Global Constraints

- Rust: one item per file, file name = item name in snake_case, layout `constants/ services/ types/ test/` as in `apps/client/native/src/jni/`; one test per file `<sentence>_test.rs`; errors are enums with hand-written `Display`/`Error`; no comments; imports at the top.
- Kotlin: follow the existing `dev.fastcast.client.stream` style (one class per file, constants in a `companion object`, test classes named as sentences, JUnit 4, `RecordingStreamListener`); no comments.
- No new crates and no new Gradle dependencies. OkHttp stays for updates only.
- Constants come from `wire` and are never redefined: `HELLO_INTERVAL` 250 ms, `FEEDBACK_INTERVAL` 20 ms, `DATAGRAM_LIMIT` 1400, `HEADER_SIZE` 12.
- Event kinds shared by Rust and Kotlin, exact: `0` video, `1` config, `2` connected, `3` ended.
- The update path (`ConnectActivity` → `UpdateRunner`) must keep working; do not touch `dev.fastcast.client.update`.
- Never install anything on the TV, never run `adb install`; never print or commit the keystore or its password.
- Rust checks: `cargo nextest run -p fastcast-native`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all --check`. Gradle (Git Bash, no `cd`):
  ```bash
  export JAVA_HOME="/c/Users/n1ghtm6r9/AppData/Local/Programs/Java/jdk-17.0.20.1+1"
  export ANDROID_HOME="/c/Users/n1ghtm6r9/AppData/Local/Android/Sdk"
  C=<worktree>/apps/client
  rm -rf "$C/app/build/test-results/testDebugUnitTest"
  "$C/gradlew" -p "$C" testDebugUnitTest lintDebug assembleRelease
  ```
  Lint verdict: last line of `app/build/reports/lint-results-debug.txt` must be "No issues found."
- Commit after each task: one-line imperative subject, no body. Never push.

## Review Focus

1. A Kotlin thread blocked in `receive` while another thread closes the link must not touch freed memory: handles are ids into a registry of `Arc<Link>`, never raw pointers. Test: Task 3 `closing_a_link_wakes_a_blocked_receive`.
2. A Rust panic must never cross JNI (it would abort the TV app): every export runs inside `catch_unwind` and returns its failure value. Test: Task 3 `a_panic_inside_an_export_returns_the_fallback`.
3. A decoder that falls behind must not grow memory without bound: the queue holds at most `MAX_QUEUED_FRAMES` frames, then drops to the next key frame and asks the server for one. Test: Task 1 `a_full_queue_drops_its_frames_and_waits_for_a_key_frame`.
4. Datagrams from another address or another session (a stale server, a second PC) must not reach the decoder. Tests: Task 2 `datagrams_from_another_address_are_ignored`, `shards_of_another_session_are_ignored`.
5. Stats text longer than one datagram must be refused, not split or truncated mid-character. Test: Task 2 `stats_longer_than_a_datagram_are_refused`.

---

## File Structure

```
apps/client/native/src/lib.rs                       mod jni; mod link; pub use jni::*; pub use link::*;
apps/client/native/src/link/constants/              EVENT_VIDEO, EVENT_CONFIG, EVENT_CONNECTED, EVENT_ENDED, MAX_QUEUED_FRAMES,
                                                    LINK_TICK, RECEIVE_BUFFER_BYTES, STATS_LIMIT, CLOSED_BY_SERVER
apps/client/native/src/link/types/                  LinkEvent, EventQueue (+ methods), LinkTarget, LinkShared, Link (+ methods), LinkState
apps/client/native/src/link/services/               run_link, hello_of, unspecified_address_for
apps/client/native/src/link/test/                   unit tests + fake_server support
apps/client/native/src/jni/services/                JNI exports, guarded, links, next_link_id, string_of, ints_of, bytes_of, event_array
apps/client/native/src/jni/test/                    fake JNI environment + tests
apps/client/app/src/main/java/dev/fastcast/client/stream/
    NativeLink.kt, StreamLink.kt, NativeStreamLink.kt, StreamTarget.kt, HelloCodecs.kt, NativeStreamClient.kt (new)
    FrameMessage.kt (offset), StreamClient.kt and StreamUrl.kt (deleted)
apps/client/app/src/main/java/dev/fastcast/client/player/PlayerSession.kt, StreamOffer.kt, PlayerActivity.kt
apps/client/app/src/test/java/dev/fastcast/client/stream/   FakeStreamLink.kt + ported tests
```

---

### Task 1: Event queue

**Files:** Create `apps/client/native/src/link/{mod.rs,constants/*,types/{link_event.rs,event_queue.rs,event_queue_*.rs}}`, `apps/client/native/src/link/test/*`; modify `apps/client/native/src/lib.rs`.

**Interfaces — Consumes:** `wire::{ReceivedFrame, FrameKind}`. **Produces:**
```rust
pub const EVENT_VIDEO: u8 = 0;
pub const EVENT_CONFIG: u8 = 1;
pub const EVENT_CONNECTED: u8 = 2;
pub const EVENT_ENDED: u8 = 3;
pub const MAX_QUEUED_FRAMES: usize = 32;
pub enum LinkEvent { Connected, Config(Vec<u8>), Video(Vec<u8>), Ended(String) }
impl LinkEvent { pub fn kind(&self) -> u8; pub fn payload(&self) -> &[u8]; }
pub struct EventQueue { state: Mutex<QueueState>, ready: Condvar }
impl EventQueue {
    pub fn new() -> EventQueue;
    pub fn push_frame(&self, frame: ReceivedFrame);
    pub fn push(&self, event: LinkEvent);
    pub fn next(&self, timeout: Duration) -> Option<LinkEvent>;
    pub fn close(&self);
    pub fn is_closed(&self) -> bool;
    pub fn needs_key_frame(&self) -> bool;
}
```
Rules:
- `QueueState` (its own file): `events: VecDeque<LinkEvent>`, `frames: usize` (video and config events queued), `waiting_for_key_frame: bool`, `closed: bool`.
- `push_frame`: config → `Config(payload)`; video while waiting and not key → dropped; key video ends the wait; a frame that would make `frames` exceed `MAX_QUEUED_FRAMES` instead removes every queued video and config event, sets `waiting_for_key_frame`, and is itself dropped unless it is a key video frame or config (then it ends the wait / is queued). Every push wakes one waiter.
- `push` queues `Connected` or `Ended` regardless of the limit.
- `next` returns the oldest event, waiting up to `timeout`; after `close`, it returns what is still queued, then `None` at once.
- `close` sets `closed` and wakes every waiter. `needs_key_frame` returns `waiting_for_key_frame`. `LinkEvent::kind` returns the matching `EVENT_*`; `payload` returns the bytes (`Ended`: the reason as UTF-8, `Connected`: empty). `EventQueue` implements `Default`.

- [ ] **Step 1: Write the failing tests**, one file each:
  - `events_come_out_in_the_order_they_went_in` — `Connected`, config, key video, video → same order.
  - `an_empty_queue_waits_for_the_timeout` — `next(30 ms)` returns `None` after at least 30 ms.
  - `a_waiting_reader_is_woken_by_a_push` — a thread blocked in `next(2 s)` gets a frame pushed 50 ms later within 1 s.
  - `a_full_queue_drops_its_frames_and_waits_for_a_key_frame` — 32 video frames (first key) → 33rd non-key video: `next` yields nothing but the `Connected` pushed before; `needs_key_frame()`; a non-key video is dropped; a key video is queued and ends the wait.
  - `config_passes_while_waiting_for_a_key_frame` — after an overflow, a config frame is queued.
  - `connected_and_ended_are_kept_when_the_queue_is_full` — `Ended("x")` after 32 frames is delivered.
  - `a_closed_queue_hands_out_what_it_holds_then_nothing` — push 2, `close()`, `next` → 2 events then `None` without waiting (under 10 ms with a 1 s timeout).
  - `closing_wakes_a_blocked_reader` — a thread in `next(5 s)` returns `None` within 500 ms of `close()`.
  - `event_kinds_match_the_kotlin_constants` — `kind()` of the four events is 0, 1, 2, 3 in the order video, config, connected, ended; `Ended("Closed by server").payload() == b"Closed by server"`.
- [ ] **Step 2: Run** `cargo nextest run -p fastcast-native` — expected: compile errors.
- [ ] **Step 3: Implement.**
- [ ] **Step 4: Run** — expected: pass.
- [ ] **Step 5: Commit** `Queue the frames the TV link delivers`.

### Task 2: Link

**Files:** Create `apps/client/native/src/link/types/{link_target.rs,link_shared.rs,link_state.rs,link.rs,link_open.rs,link_next_event.rs,link_send_stats.rs,link_close.rs,link_drop.rs}`, `apps/client/native/src/link/services/{run_link.rs,hello_of.rs,unspecified_address_for.rs}`, constants, tests and `apps/client/native/src/link/test/fake_server.rs` (+ its method files).

**Interfaces — Consumes:** Task 1; `wire::{Hello, CodecSupport, Codec, Datagram, Packet, Feedback, FrameReceiver, ReceiveCounters, decode_datagram, encode_datagram, HELLO_INTERVAL, FEEDBACK_INTERVAL, DATAGRAM_LIMIT, HEADER_SIZE}`. **Produces:**
```rust
pub const LINK_TICK: Duration = Duration::from_millis(2);
pub const RECEIVE_BUFFER_BYTES: usize = 2048;
pub const STATS_LIMIT: usize = DATAGRAM_LIMIT - HEADER_SIZE;
pub const CLOSED_BY_SERVER: &str = "Closed by server";
pub struct LinkTarget { pub server: SocketAddr, pub hello: Hello }
pub struct LinkShared { pub queue: EventQueue, pub session: AtomicU32, pub next_sequence: AtomicU32, pub stop: AtomicBool }
pub struct Link { socket: Arc<UdpSocket>, server: SocketAddr, shared: Arc<LinkShared>, thread: Mutex<Option<JoinHandle<()>>> }
impl Link {
    pub fn open(target: LinkTarget) -> io::Result<Link>;
    pub fn next_event(&self, timeout: Duration) -> Option<LinkEvent>;
    pub fn send_stats(&self, text: &[u8]) -> bool;
    pub fn close(&self);
}
impl Drop for Link
pub fn hello_of(nonce: u32, width: i32, height: i32, fps: i32, codecs: &[i32]) -> Hello;
pub fn unspecified_address_for(server: &SocketAddr) -> SocketAddr;
pub fn run_link(socket: Arc<UdpSocket>, target: LinkTarget, shared: Arc<LinkShared>);
```
Rules:
- `hello_of`: width/height/fps outside `0..=65535` become 0; `codecs` is pairs `(id, max_bit_depth)`, ids 1/2/3 → `Avc`/`Hevc`/`Av1`, other ids and a trailing odd element skipped, depth outside `0..=255` → 8.
- `Link::open`: bind `unspecified_address_for(&server)` (`0.0.0.0:0` or `[::]:0`), read timeout `LINK_TICK`, spawn thread `fastcast-link` with `run_link`. The nonce comes from the caller's `Hello`.
- `run_link` keeps a `LinkState` (its own file: `session: Option<u32>`, `receiver: FrameReceiver`, `counters: ReceiveCounters`, `next_hello: Instant`, `next_feedback: Instant`, `reported_losses: u32`). Loop until `shared.stop`:
  1. Without a session and at `next_hello`: send Hello (session 0), `next_hello += HELLO_INTERVAL`.
  2. `recv_from`; `WouldBlock`/`TimedOut` → step 4; another error → push `Ended(error text)` and leave.
  3. Only datagrams from `target.server` that decode: a `Welcome` with the Hello's nonce while there is no session → store `datagram.session` in `state` and `shared.session`, push `Connected`, `next_feedback = now`. Any datagram whose session is ours → `counters.record(sequence)`; `Shard` → `receiver.accept(shard, now)`; `Bye` → push `Ended(CLOSED_BY_SERVER)`, close the queue, leave without answering. Everything else is ignored.
  4. `poll` the receiver until `None`, `push_frame` each delivered frame.
  5. With a session, at `next_feedback` or when `receiver.lost_frames() > reported_losses`: send Feedback (`highest_frame`, `received_packets`, `highest_sequence`, `lost_frames`, `recovered_shards`, `keyframe_needed = receiver.keyframe_needed() || queue.needs_key_frame()`), `reported_losses = lost_frames`, `next_feedback = now + FEEDBACK_INTERVAL`.
  On `stop`: with a session, send Bye. Every datagram the link sends takes `shared.next_sequence.fetch_add(1)` and the current session.
- `send_stats`: `false` without a session or when `text.len() > STATS_LIMIT` or the text is not UTF-8; otherwise one Stats datagram, `true` when `send_to` succeeds.
- `close`: set `stop`, join the thread (once), close the queue. `Drop` calls `close`.

Test support `fake_server.rs`: `FakeServer` bound on `127.0.0.1:0` with a 2 s read timeout: `address()`, `expect_hello(&self) -> (Hello, SocketAddr)`, `welcome(&mut self, peer, nonce, session)`, `send_frame(&mut self, peer, kind: FrameKind, keyframe: bool, payload: &[u8], fec_percent: u8) -> Vec<Vec<u8>>` (through a `Packetizer` of its session; returns the datagrams it sent), `send_datagrams(&self, peer, datagrams: &[Vec<u8>])`, `send_bye(&mut self, peer)`, `next_packet(&self, timeout) -> Option<(Datagram owned parts, SocketAddr)>` (decode into an owned enum of what the tests need: Hello, Feedback, Bye, Stats text); `connected_link() -> (FakeServer, Link, SocketAddr)` (opens a link to the fake server, answers its Hello, waits for `Connected`).

- [ ] **Step 1: Write the failing tests**, one file each:
  - `the_hello_carries_the_mode_and_codecs` — `hello_of(7, 3840, 2160, 120, &[2, 10, 1, 8])` → `Hello { nonce: 7, width: 3840, height: 2160, fps: 120, codecs: [Hevc/10, Avc/8] }`.
  - `impossible_hello_values_are_left_out` — width −1 and 70 000 → 0; codecs `[9, 8, 2]` → empty.
  - `a_link_says_hello_until_it_is_welcomed` — the fake server reads two Hellos at least 200 ms apart with the same nonce, then welcomes → `next_event` gives `Connected`.
  - `a_welcome_with_another_nonce_is_ignored` — no `Connected` within 300 ms; Hellos keep coming.
  - `config_and_video_arrive_in_frame_order` — config, key video, video → events `Connected`, `Config`, `Video`, `Video` with the sent payloads.
  - `a_lost_shard_is_rebuilt_on_the_tv` — 20 000-byte key frame at fec 20, its first datagram not sent → the payload arrives whole.
  - `datagrams_from_another_address_are_ignored` — a second socket sends valid shards of the link's session → no frame within 300 ms.
  - `shards_of_another_session_are_ignored` — the fake server sends shards with session + 1 → no frame.
  - `feedback_flows_every_twenty_milliseconds` — at least 8 Feedback datagrams with the link's session within 250 ms of `Connected`.
  - `a_lost_frame_is_reported_at_once_with_keyframe_needed` — frame 0 key delivered; frame 1 (3 data shards, fec 0) sent without its first datagram; frame 2 sent whole → a Feedback with `keyframe_needed` and `lost_frames == 1` arrives within 150 ms of frame 2.
  - `a_bye_from_the_server_ends_the_link` — `Ended("Closed by server")`, then `next_event` returns `None` at once.
  - `closing_a_link_says_bye` — `close()` → the fake server gets Bye with the session; `close()` again is harmless.
  - `stats_reach_the_server_as_text` — `send_stats(b"{\"type\":\"stats\"}")` is `true`; the fake server reads that text.
  - `stats_before_the_welcome_are_refused` — a link whose Hello is never answered → `false`.
  - `stats_longer_than_a_datagram_are_refused` — 1389 bytes → `false`; 1388 bytes → `true`.
- [ ] **Step 2: Run** — expected: compile errors.
- [ ] **Step 3: Implement.**
- [ ] **Step 4: Run** three times — expected: pass every time.
- [ ] **Step 5: Commit** `Receive the stream on the TV over the wire protocol`.

### Task 3: JNI surface

**Files:** Create `apps/client/native/src/jni/services/{java_dev_fastcast_client_stream_native_link_open.rs,…_receive.rs,…_send_stats.rs,…_close.rs,guarded.rs,links.rs,next_link_id.rs,string_of.rs,ints_of.rs,bytes_of.rs,event_array.rs,link_of.rs}`, `apps/client/native/src/jni/test/{fake_jvm.rs,…}` and tests.

**Interfaces — Consumes:** Tasks 1–2; `jni_sys::{JNIEnv, jclass, jstring, jint, jlong, jintArray, jbyteArray, jboolean, JNI_TRUE, JNI_FALSE}`. **Produces:**
```rust
pub extern "system" fn Java_dev_fastcast_client_stream_NativeLink_open(env: *mut JNIEnv, class: jclass, host: jstring, port: jint, nonce: jint, width: jint, height: jint, fps: jint, codecs: jintArray) -> jlong;
pub extern "system" fn Java_dev_fastcast_client_stream_NativeLink_receive(env: *mut JNIEnv, class: jclass, link: jlong, timeout_ms: jint) -> jbyteArray;
pub extern "system" fn Java_dev_fastcast_client_stream_NativeLink_sendStats(env: *mut JNIEnv, class: jclass, link: jlong, text: jbyteArray) -> jboolean;
pub extern "system" fn Java_dev_fastcast_client_stream_NativeLink_close(env: *mut JNIEnv, class: jclass, link: jlong);
pub fn guarded<T>(fallback: T, body: impl FnOnce() -> T) -> T;
pub fn links() -> &'static Mutex<HashMap<i64, Arc<Link>>>;
pub fn next_link_id() -> i64;
pub fn link_of(id: i64) -> Option<Arc<Link>>;
```
Rules:
- Each export carries `#[allow(non_snake_case)] #[unsafe(no_mangle)]` like `protocolVersion`, and its whole body runs in `guarded` (`catch_unwind(AssertUnwindSafe(body))`, the fallback on a panic): `open` → 0, `receive` → null, `sendStats` → `JNI_FALSE`, `close` → nothing.
- `open`: host through `GetStringUTFChars`/`ReleaseStringUTFChars`, parsed as an `IpAddr` (no DNS); port must be 1..=65535; `codecs` through `GetArrayLength` + `GetIntArrayRegion`; `Link::open(LinkTarget { server, hello: hello_of(nonce as u32, …) })`; on success store `Arc<Link>` under `next_link_id()` (an `AtomicI64` from 1) and return the id; any failure → 0.
- `receive`: `link_of(id)` (clone the `Arc` under the lock, release the lock before waiting) → `next_event(timeout_ms clamped to 0..=1000 ms)`; an event becomes a new `byte[]` of `1 + payload.len()` (`NewByteArray`, then `SetByteArrayRegion` for the kind byte and for the payload); `NewByteArray` returning null → null.
- `sendStats`: `GetArrayLength` + `GetByteArrayRegion` → `link.send_stats`.
- `close`: remove the id from `links()`, then `link.close()` (a `receive` still holding its `Arc` wakes with `None`).
- Test support `fake_jvm.rs`: a `JNINativeInterface_` from `unsafe { std::mem::zeroed() }` with `GetStringUTFChars`, `ReleaseStringUTFChars`, `GetArrayLength`, `GetIntArrayRegion`, `GetByteArrayRegion`, `NewByteArray`, `SetByteArrayRegion` set to `extern "system"` functions that treat `jobject` as a pointer to a test-owned `FakeObject` enum (`Text(CString)`, `Ints(Vec<i32>)`, `Bytes(RefCell<Vec<i8>>)`); `NewByteArray` leaks a new `FakeObject::Bytes` that the test reads back. Helpers: `fake_env() -> FakeEnv` (owns the table and a `*mut JNIEnv` to it), `text(&str) -> FakeObject`, `ints(&[i32]) -> FakeObject`, `bytes(&[u8]) -> FakeObject`, `bytes_back(jbyteArray) -> Vec<u8>`.

- [ ] **Step 1: Write the failing tests**, one file each, against a `FakeServer` from Task 2 where a server is needed:
  - `an_opened_link_reports_connected_through_receive` — `open(env, null, text("127.0.0.1"), port, 5, 1920, 1080, 60, ints(&[2, 8]))` returns `> 0`; the fake server welcomes nonce 5; `receive(…, 1000)` → bytes `[2]`.
  - `a_frame_comes_out_as_its_kind_byte_and_its_payload` — a video frame `[1, 2, 3]` → `[0, 1, 2, 3]`; a config frame of `{"type":"config"}` → `[1]` followed by that text.
  - `a_host_that_is_not_an_ip_address_opens_nothing` — `"tv.local"` → 0; port 0 → 0; port 70 000 → 0.
  - `receive_on_an_unknown_link_returns_null_at_once` — id 999 → null in under 50 ms.
  - `stats_are_sent_through_the_link` — `sendStats(…, bytes(b"{}"))` → `JNI_TRUE`, the fake server reads `{}`.
  - `closing_a_link_wakes_a_blocked_receive` — a thread in `receive(…, 1000)` returns null within 500 ms of `close` from another thread; later calls with that id return null; the fake server gets Bye.
  - `a_panic_inside_an_export_returns_the_fallback` — `guarded(7, || panic!("boom")) == 7`.
- [ ] **Step 2: Run** — expected: compile errors.
- [ ] **Step 3: Implement.**
- [ ] **Step 4: Run** `cargo nextest run -p fastcast-native` and clippy — expected: pass.
- [ ] **Step 5: Run** the Gradle command — expected: BUILD SUCCESSFUL; `llvm-nm -D --defined-only` on both packaged `.so` files lists the five `Java_dev_fastcast_client_stream_NativeLink_*` symbols.
- [ ] **Step 6: Commit** `Expose the TV link to Kotlin over JNI`.

### Task 4: Kotlin stream client on the native link

**Files:** Create in `apps/client/app/src/main/java/dev/fastcast/client/stream/`: `NativeLink.kt`, `StreamLink.kt`, `NativeStreamLink.kt`, `StreamTarget.kt`, `HelloCodecs.kt`, `NativeStreamClient.kt`; modify `FrameMessage.kt`. Tests in `apps/client/app/src/test/java/dev/fastcast/client/stream/`: `FakeStreamLink.kt` and the tests below.

**Interfaces — Produces:**
```kotlin
object NativeLink {
    const val EVENT_VIDEO = 0; const val EVENT_CONFIG = 1; const val EVENT_CONNECTED = 2; const val EVENT_ENDED = 3
    @JvmStatic external fun protocolVersion(): Int
    @JvmStatic external fun open(host: String, port: Int, nonce: Int, width: Int, height: Int, fps: Int, codecs: IntArray): Long
    @JvmStatic external fun receive(link: Long, timeoutMs: Int): ByteArray?
    @JvmStatic external fun sendStats(link: Long, text: ByteArray): Boolean
    @JvmStatic external fun close(link: Long)
}
interface StreamLink {
    fun open(target: StreamTarget): Long
    fun receive(link: Long, timeoutMs: Int): ByteArray?
    fun sendStats(link: Long, text: ByteArray): Boolean
    fun close(link: Long)
}
object NativeStreamLink : StreamLink
class StreamTarget(val host: String, val port: Int, val request: StreamRequest?)
object HelloCodecs { fun of(names: List<String>): IntArray }
class NativeStreamClient(
    private val targetFor: () -> StreamTarget,
    private val listener: StreamListener,
    private val link: StreamLink = NativeStreamLink,
    private val backoff: ReconnectBackoff = ReconnectBackoff(),
    private val silence: SilenceTimer = SilenceTimer(),
    private val health: StreamHealth = StreamHealth(),
) { val bytesReceived: AtomicLong; fun start(); fun send(text: String): Boolean; fun reconnect(); fun stop() }
FrameMessage.parse(data: ByteArray, offset: Int = 0): FrameMessage?
```
Rules:
- `NativeLink` loads the library in `init { System.loadLibrary("fastcast_native") }`. `NativeStreamLink` maps a target to `NativeLink.open(host, port, nonce, width, height, fps, HelloCodecs.of(codecs))` with `0, 0, 0, empty` when `request` is null and a nonce from `Random.nextInt()`.
- `HelloCodecs.of`: `"avc"` → `[1, 8]`, `"hevc"` → `[2, 8]`, other names skipped, order kept.
- `FrameMessage` gets a `private val offset: Int`; `payloadOffset = offset + HEADER_SIZE`, `payloadSize = data.size - payloadOffset`; flags and timestamp are read at `offset`; `parse` rejects `data.size <= offset + HEADER_SIZE`. Existing callers keep offset 0.
- `NativeStreamClient` keeps every behaviour of `StreamClient` (read it first): the same thread name `fastcast-stream`, the scheduler for silence checks and reopen, generations, `ReconnectBackoff`, `SilenceTimer`, `StreamHealth`, the strings `Closed by server` and `No data from server`. Differences: opening = `link.open(targetFor())` (0 → treated as a lost connection with reason `Could not open the stream`), then a reader thread per generation loops `link.receive(handle, RECEIVE_POLL_MS = 100)` while that generation is current and dispatches: connected → `silence.restart()`, `onConnected()`; config → `silence.touch()`, `ConfigMessage.parse(String(event, 1, event.size - 1, UTF_8))?.let(onConfig)`; video → `silence.touch()`, `bytesReceived += event.size - 1`, `FrameMessage.parse(event, 1)` → health check → `onHealthy()` and `backoff.reset()` as today → `onFrame`; ended → lost with the event's text (empty → `Closed by server`). Retiring a generation calls `link.close(handle)` once. `send(text)` → `link.sendStats(current handle, text.toByteArray(UTF_8))`, `false` without a handle.
- `FakeStreamLink` (test): records opened targets, hands out handles from 1, lets the test push events per handle (`connect(handle)`, `config(handle, json)`, `frame(handle, bytes)`, `end(handle, reason)`), blocks `receive` on a `LinkedBlockingQueue` per handle up to the timeout, records sent stats and closed handles, can refuse the next `open`.

- [ ] **Step 1: Write the failing tests** — port every `StreamClient` test class to `NativeStreamClient` with `FakeStreamLink` in place of `FakeWebSocketServer` (same class names with `StreamClient` → `NativeStreamClient`, same test functions and assertions; "drop a connection" = `end(handle, "")`; frames are built with `dev.fastcast.client.testing.frameBytes`): `AHealthyStreamRestartsTheReconnectDelaysTest`, `ASilentConnectionIsReplacedByAFreshOneTest`, `AskingTheStreamClientToReconnectOpensAFreshConnectionTest`, `ASlowStartOfAConnectionIsNotMistakenForSilenceTest`, `StoppingTheStreamClientEndsTheConversationForGoodTest`, `TheStreamClientAsksForTheUrlOnEveryConnectionTest` (→ `…AsksForTheTargetOnEveryConnectionTest`), `TheStreamClientDeliversTheConfigAndTheFramesInOrderTest`, `TheStreamClientReconnectsWithGrowingDelaysTest`, `TheStreamClientRefusesTextItCannotDeliverTest`, `TheStreamClientSendsTextToTheServerTest`. New:
  - `AnUnopenableLinkIsRetriedWithGrowingDelaysTest` — the fake refuses two opens → retry delays 40, 80, then a connection.
  - `ALinkEndedByTheServerIsReplacedTest` — `end(handle, "Closed by server")` → `onDisconnected(_, "Closed by server")`, the old handle is closed, a new one opened.
  - `AFrameAfterTheEventByteKeepsItsKeyFlagAndTimestampTest` — `FrameMessage.parse(byteArrayOf(0) + frameBytes(true, 258L, payload), 1)` → keyframe, timestamp 258, `payloadOffset == 6`, payload bytes equal.
  - `HelloCodecsAreTheWireIdsInPreferenceOrderTest` — `of(listOf("hevc", "avc", "vp9")) == intArrayOf(2, 8, 1, 8)`.
- [ ] **Step 2: Run** `"$C/gradlew" -p "$C" testDebugUnitTest` — expected: compile failures.
- [ ] **Step 3: Implement.**
- [ ] **Step 4: Run** — expected: all tests pass (the old `StreamClient` tests still pass too).
- [ ] **Step 5: Commit** `Drive the stream from the native link in Kotlin`.

### Task 5: Switch the player to the native link and remove the WebSocket client

**Files:** Modify `player/PlayerSession.kt`, `player/StreamOffer.kt`, `player/PlayerActivity.kt` (only what the type change needs); delete `stream/StreamClient.kt`, `stream/StreamUrl.kt`, the `StreamClient` tests ported in Task 4, `FakeWebSocketServer.kt`, `FakeConnection.kt`, `TheStreamUrlCarriesTheDisplayModeAndTheCodecsTest.kt`, `TheServerSeesTheRequestedDisplayModeOnTheWireTest.kt`; modify `README.md`.

- [ ] **Step 1: Write the failing tests** (player tests):
  - `TheOfferTargetsTheServerWithTheRequestedModeAndCodecsTest`: `StreamOffer("192.168.0.10:3278", StreamRequest(3840, 2160, 120, listOf("hevc", "avc"))).target()` → host `192.168.0.10`, port 3278, request 3840×2160@120 with `["hevc", "avc"]`; an address without a port → port 3278.
  - Rewrite `ADecoderThatKeepsFailingOnHevcIsOfferedAvcInsteadTest` against `target()`: every `assertEquals(urlWithCodecs(x), offer.url())` becomes an assertion on `offer.target().request?.codecs`; the plain-offer case asserts `request == null`; the no-codecs case asserts the mode with an empty codec list.
  - Rewrite `TheServerIsAskedForAvcOnceHevcKeepsFailingTest` with `NativeStreamClient(offer::target, …, FakeStreamLink)`: the codecs of the targets the fake link was opened with go from `["hevc", "avc"]` to `["avc"]`, with the same failure sequence as today.
- [ ] **Step 2: Run** — expected: compile failure (`target` missing).
- [ ] **Step 3: Implement:** `StreamOffer.target(): StreamTarget` replaces `url()` (split the address at its last `:`; a missing or bad port → 3278); `PlayerSession` uses `NativeStreamClient(offer::target, this)`; delete the files above.
- [ ] **Step 4: Run** the Gradle command — expected: BUILD SUCCESSFUL, lint "No issues found.", the test count reported; `grep -rn "okhttp3.WebSocket\|StreamUrl\|StreamClient(" apps/client/app/src` → nothing (OkHttp imports remain only in `update`).
- [ ] **Step 5: Update** `README.md`: the TV section says the app receives the stream over the UDP protocol through its native library.
- [ ] **Step 6: Commit** `Play the stream from the native UDP link`.
