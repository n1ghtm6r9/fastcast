package dev.fastcast.client.debug

import org.json.JSONObject
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test

class TheDebugRunWalksEveryStepTest {

    private val clock = FakeDebugClock()
    private val host = FakeDebugHost(clock)
    private val run = DebugRun(host, clock)

    private fun runToTheEnd() {
        run.start()
        clock.advanceBy(WHOLE_RUN_MS)
    }

    @Test
    fun `the steps run in order and the usual stream comes back at the end`() {
        runToTheEnd()

        assertEquals(
            listOf(
                "step FACTS", "step VSYNC", "vsync", "step ASK", "ask 120.0", "vsync", "ask 144.0", "vsync", "clear",
                "step STREAM", "hide", "step TUNNEL", "trial tunnel1440", "hide", "step TUNNEL", "trial tunnel1440noise",
                "hide", "step SURFACE", "trial surface2160", "step SURFACE", "trial surface2160noise", "step RESTORE",
                "usual", "done",
            ),
            host.events,
        )
    }

    @Test
    fun `every video meant to be seen bare is measured with the debug text hidden for a while`() {
        runToTheEnd()

        val bare = host.countedAfterHidingMs.filterNotNull()
        assertEquals(6, bare.size)
        assertTrue(bare.toString(), bare.all { it >= 3_000L })
    }

    @Test
    fun `the trials that need the screen to report their frames are measured with the debug text shown`() {
        runToTheEnd()

        val shownPerTrial = host.countedAfterHidingMs.drop(2).chunked(2).map { counted -> counted.all { it == null } }
        assertEquals(listOf(false, false, true, true), shownPerTrial)
    }

    @Test
    fun `every step reports to the server in order`() {
        runToTheEnd()

        val steps = host.sent.map { JSONObject(it).getString("step") }.distinct()
        assertEquals(listOf("device", "display", "vsync", "ask120", "ask144", "stream") + ALL_TRIALS + "done", steps)
    }

    @Test
    fun `nothing is sent while the video does not play`() {
        host.playing = false
        run.start()
        clock.advanceBy(1_000L)
        assertTrue(host.sent.isEmpty())

        host.playing = true
        clock.advanceBy(1_000L)

        assertEquals("device", JSONObject(host.sent.first()).getString("step"))
    }

    @Test
    fun `the usual stream report names the rates and what its video path saw`() {
        runToTheEnd()

        val stream = textOf("stream")
        assertTrue(stream, stream.startsWith("queued 120.0/s, decoded 120.0/s"))
        assertTrue(stream, stream.endsWith("; audio played 6.0 s"))
    }

    @Test
    fun `a trial report names what was asked, the rates and what the trial saw`() {
        runToTheEnd()

        val tunnel = textOf("tunnel1440")
        assertTrue(tunnel, tunnel.startsWith("asked 2560x1440@120 tunneled; queued 120.0/s, decoded 120.0/s"))
        assertTrue(tunnel, tunnel.contains("presented: "))
        assertTrue(tunnel, tunnel.endsWith("; audio played 6.0 s"))
    }

    @Test
    fun `one run tries the 10-bit tunnel at 1440p and 4K hevc the usual way, each on the screen and on noise`() {
        runToTheEnd()

        assertEquals(
            listOf(
                "2560x1440@120 hevc 10-bit tunneled REALTIME LOW_LATENCY uninspected text hidden ON_RUNNING_CLOCK 8000 screen",
                "2560x1440@120 hevc 10-bit tunneled REALTIME LOW_LATENCY uninspected text hidden ON_RUNNING_CLOCK 8000 noise",
                "3840x2160@60 hevc 10-bit surface REALTIME LOW_LATENCY uninspected text shown ON_RUNNING_CLOCK 8000 screen",
                "3840x2160@60 hevc 10-bit surface REALTIME LOW_LATENCY uninspected text shown ON_RUNNING_CLOCK 8000 noise",
            ),
            host.trials.map(::describeTrial),
        )
    }

    private fun describeTrial(trial: StreamTrial): String {
        val path = if (trial.tunneled) "tunneled" else "surface"
        val inspection = if (trial.setup.inspectsVendorParameters) "inspected" else "uninspected"
        val text = if (trial.showsText) "text shown" else "text hidden"
        val depth = if (trial.tenBit) "10-bit" else "8-bit"
        val timing = trial.setup.timing
        val picture = if (trial.noise) "noise" else "screen"
        return "${trial.width}x${trial.height}@${trial.fps} ${trial.codec} $depth $path ${trial.setup.pace} " +
            "${trial.setup.tweak} $inspection $text ${timing.videoStart} ${timing.videoDelayUs} $picture"
    }

    @Test
    fun `the end shows what every trial showed`() {
        host.presentIntervalNanos = 16_666_667L

        runToTheEnd()

        val results = requireNotNull(host.shownResults)
        assertEquals(ALL_TRIALS, results.map { it.trial.step })
        results.forEach {
            assertEquals(120.0, it.queuedPerSecond, 0.5)
            assertEquals(60.0, it.presentedPerSecond, 0.5)
        }
    }

    @Test
    fun `a trial that never plays is reported with the last error and the next trial still runs`() {
        host.trialStartsAfterMs = mapOf("tunnel1440noise" to null)
        host.lastError = "Decoder error: no tunnel"

        runToTheEnd()

        assertEquals(
            "asked 2560x1440@120 tunneled; no video within 20 s; Decoder error: no tunnel; audio played 6.0 s",
            textOf("tunnel1440noise"),
        )
        assertEquals(listOf("trial surface2160noise", "step RESTORE", "usual", "done"), host.events.takeLast(4))
        assertEquals(ALL_TRIALS - "tunnel1440noise", requireNotNull(host.shownResults).map { it.trial.step })
    }

    @Test
    fun `when no trial plays the usual stream still comes back`() {
        host.trialStartsAfterMs = ALL_TRIALS.associateWith { null }

        runToTheEnd()

        assertEquals(listOf("step RESTORE", "usual", "done"), host.events.takeLast(3))
        assertEquals(emptyList<TrialResult>(), host.shownResults)
    }

    @Test
    fun `a cancelled run sends nothing more and leaves the stream alone`() {
        run.start()
        clock.advanceBy(500L)
        run.cancel()
        val sentBefore = host.sent.size

        clock.advanceBy(WHOLE_RUN_MS)

        assertEquals(sentBefore, host.sent.size)
        assertFalse(host.events.any { it.startsWith("trial") })
        assertFalse(run.isRunning)
    }

    @Test
    fun `a run cancelled while it waits goes no further`() {
        run.start()
        clock.advanceBy(10_000L)
        run.cancel()
        val sentBefore = host.sent.size

        clock.advanceBy(WHOLE_RUN_MS)

        assertEquals(sentBefore, host.sent.size)
        assertFalse(host.events.any { it.startsWith("trial") })
    }

    @Test
    fun `a cancelled run drops what it has not sent yet`() {
        host.playing = false
        run.start()
        clock.advanceBy(500L)
        run.cancel()

        host.playing = true
        clock.advanceBy(WHOLE_RUN_MS)

        assertTrue(host.sent.isEmpty())
    }

    @Test
    fun `a run sends at most four messages every fifty milliseconds`() {
        host.sections = List(10) { FactSection("section$it", listOf("fact $it")) }

        run.start()
        assertEquals(4, host.sent.size)

        clock.advanceBy(50)
        assertEquals(8, host.sent.size)

        clock.advanceBy(50)
        assertEquals(10, host.sent.size)
    }

    @Test
    fun `starting a run twice runs it once`() {
        run.start()

        runToTheEnd()

        assertEquals(1, host.events.count { it == "step FACTS" })
    }

    @Test
    fun `the run keeps running until everything is sent`() {
        run.start()
        assertTrue(run.isRunning)

        clock.advanceBy(WHOLE_RUN_MS)

        assertFalse(run.isRunning)
    }

    private fun textOf(step: String): String =
        host.sent.map(::JSONObject).first { it.getString("step") == step }.getString("text")

    companion object {
        private const val WHOLE_RUN_MS = 400_000L
        private val ALL_TRIALS = listOf(
            "tunnel1440", "tunnel1440noise", "surface2160", "surface2160noise",
        )
    }
}
