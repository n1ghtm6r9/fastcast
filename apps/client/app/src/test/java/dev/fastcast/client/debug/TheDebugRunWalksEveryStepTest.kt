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
                "step STREAM", "hide", "step TUNNEL", "trial tunnel2160bare", "hide", "step TUNNEL",
                "trial tunnel2160lowlatency", "hide", "step FAST", "trial fast2160bare", "hide", "step FAST",
                "trial fast2160discard", "hide", "step FAST", "trial fast2160stretched", "hide", "step RESTORE", "usual",
                "done",
            ),
            host.events,
        )
    }

    @Test
    fun `every video is measured with the debug text hidden for a while`() {
        runToTheEnd()

        assertEquals(12, host.countedAfterHidingMs.size)
        assertTrue(host.countedAfterHidingMs.toString(), host.countedAfterHidingMs.all { it != null && it >= 3_000L })
    }

    @Test
    fun `every step reports to the server in order`() {
        runToTheEnd()

        val steps = host.sent.map { JSONObject(it).getString("step") }.distinct()
        assertEquals(
            listOf(
                "device", "display", "vsync", "ask120", "ask144", "stream", "tunnel2160bare", "tunnel2160lowlatency",
                "fast2160bare", "fast2160discard", "fast2160stretched", "done",
            ),
            steps,
        )
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
    fun `a trial report names what was asked, the rates and what the trial saw`() {
        runToTheEnd()

        val tunnel = textOf("tunnel2160bare")
        assertTrue(tunnel, tunnel.startsWith("asked 3840x2160@120 tunneled; queued 120.0/s, decoded 120.0/s"))
        assertTrue(tunnel, tunnel.contains("presented: "))
        assertTrue(tunnel, tunnel.endsWith("; audio played 6.0 s"))
    }

    @Test
    fun `4K at 120 is tried in the tunnel plain and with low latency and on the surface plain, unshown and restamped`() {
        runToTheEnd()

        assertEquals(
            listOf(
                "3840x2160@120 tunneled UNDECLARED NONE",
                "3840x2160@120 tunneled UNDECLARED LOW_LATENCY",
                "3840x2160@120 surface UNDECLARED NONE",
                "3840x2160@120 surface UNDECLARED DISCARD_OUTPUT",
                "3840x2160@120 surface UNDECLARED STRETCHED_TIMESTAMPS",
            ),
            host.trials.map {
                "${it.width}x${it.height}@${it.fps} ${if (it.tunneled) "tunneled" else "surface"} ${it.pace} ${it.tweak}"
            },
        )
    }

    @Test
    fun `the end shows what every trial showed`() {
        host.presentIntervalNanos = 16_666_667L

        runToTheEnd()

        val results = requireNotNull(host.shownResults)
        assertEquals(
            listOf("tunnel2160bare", "tunnel2160lowlatency", "fast2160bare", "fast2160discard", "fast2160stretched"),
            results.map { it.trial.step },
        )
        results.forEach {
            assertEquals(120.0, it.queuedPerSecond, 0.5)
            assertEquals(60.0, it.presentedPerSecond, 0.5)
        }
    }

    @Test
    fun `a trial that never plays is reported with the last error and the next trial still runs`() {
        host.trialStartsAfterMs = mapOf("tunnel2160lowlatency" to null)
        host.lastError = "Decoder error: no tunnel"

        runToTheEnd()

        assertEquals(
            "asked 3840x2160@120 tunneled; no video within 20 s; Decoder error: no tunnel; audio played 6.0 s",
            textOf("tunnel2160lowlatency"),
        )
        assertEquals(listOf("trial fast2160stretched", "hide", "step RESTORE", "usual", "done"), host.events.takeLast(5))
        assertEquals(
            listOf("tunnel2160bare", "fast2160bare", "fast2160discard", "fast2160stretched"),
            requireNotNull(host.shownResults).map { it.trial.step },
        )
    }

    @Test
    fun `when no trial plays the usual stream still comes back`() {
        host.trialStartsAfterMs = listOf(
            "tunnel2160bare", "tunnel2160lowlatency", "fast2160bare", "fast2160discard", "fast2160stretched",
        ).associateWith { null }

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
        private const val WHOLE_RUN_MS = 200_000L
    }
}
