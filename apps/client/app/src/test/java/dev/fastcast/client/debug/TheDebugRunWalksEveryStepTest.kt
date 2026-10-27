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
                "step STREAM", "hide", "step FAST", "trial fast2160game", "hide", "step FAST", "trial fast2160forced60",
                "hide", "step FAST", "trial fast2160forced120", "hide", "step FAST", "trial fast2160limit120", "hide",
                "step TUNNEL", "trial tunnel2160game", "step TUNNEL", "trial tunnel1440hidden", "hide", "step RESTORE",
                "usual", "done",
            ),
            host.events,
        )
    }

    @Test
    fun `every video meant to be seen bare is measured with the debug text hidden for a while`() {
        runToTheEnd()

        val bare = host.countedAfterHidingMs.filterNotNull()
        assertEquals(12, bare.size)
        assertTrue(bare.toString(), bare.all { it >= 3_000L })
    }

    @Test
    fun `the trial that needs the screen to report its frames is measured with the debug text shown`() {
        runToTheEnd()

        assertEquals(14, host.countedAfterHidingMs.size)
        assertEquals(listOf(null, null), host.countedAfterHidingMs.subList(10, 12))
    }

    @Test
    fun `every step reports to the server in order`() {
        runToTheEnd()

        val steps = host.sent.map { JSONObject(it).getString("step") }.distinct()
        assertEquals(
            listOf(
                "device", "display", "vsync", "ask120", "ask144", "stream", "fast2160game", "fast2160forced60",
                "fast2160forced120", "fast2160limit120", "tunnel2160game", "tunnel1440hidden", "done",
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

        val tunnel = textOf("tunnel2160game")
        assertTrue(tunnel, tunnel.startsWith("asked 3840x2160@120 tunneled; queued 120.0/s, decoded 120.0/s"))
        assertTrue(tunnel, tunnel.contains("presented: "))
        assertTrue(tunnel, tunnel.endsWith("; audio played 6.0 s"))
    }

    @Test
    fun `4K at 120 is tried on the surface as a game, with its rate forced and its limit raised, then in the tunnel`() {
        runToTheEnd()

        assertEquals(
            listOf(
                "3840x2160@120 surface UNDECLARED MEDIATEK_GAME inspected text hidden",
                "3840x2160@120 surface UNDECLARED FORCED_DISPLAY_RATE inspected text hidden",
                "3840x2160@120 surface UNDECLARED FORCED_STREAM_RATE inspected text hidden",
                "3840x2160@120 surface UNDECLARED RAISED_RATE_LIMIT inspected text hidden",
                "3840x2160@120 tunneled UNDECLARED MEDIATEK_GAME uninspected text shown",
                "2560x1440@120 tunneled REALTIME NONE uninspected text hidden",
            ),
            host.trials.map(::describeTrial),
        )
    }

    private fun describeTrial(trial: StreamTrial): String {
        val path = if (trial.tunneled) "tunneled" else "surface"
        val inspection = if (trial.setup.inspectsVendorParameters) "inspected" else "uninspected"
        val text = if (trial.showsText) "text shown" else "text hidden"
        return "${trial.width}x${trial.height}@${trial.fps} $path ${trial.setup.pace} ${trial.setup.tweak} $inspection $text"
    }

    @Test
    fun `the end shows what every trial showed`() {
        host.presentIntervalNanos = 16_666_667L

        runToTheEnd()

        val results = requireNotNull(host.shownResults)
        assertEquals(
            listOf(
                "fast2160game", "fast2160forced60", "fast2160forced120", "fast2160limit120", "tunnel2160game",
                "tunnel1440hidden",
            ),
            results.map { it.trial.step },
        )
        results.forEach {
            assertEquals(120.0, it.queuedPerSecond, 0.5)
            assertEquals(60.0, it.presentedPerSecond, 0.5)
        }
    }

    @Test
    fun `a trial that never plays is reported with the last error and the next trial still runs`() {
        host.trialStartsAfterMs = mapOf("fast2160forced60" to null)
        host.lastError = "Decoder error: no tunnel"

        runToTheEnd()

        assertEquals(
            "asked 3840x2160@120; no video within 20 s; Decoder error: no tunnel; audio played 6.0 s",
            textOf("fast2160forced60"),
        )
        assertEquals(listOf("trial tunnel1440hidden", "hide", "step RESTORE", "usual", "done"), host.events.takeLast(5))
        assertEquals(
            listOf("fast2160game", "fast2160forced120", "fast2160limit120", "tunnel2160game", "tunnel1440hidden"),
            requireNotNull(host.shownResults).map { it.trial.step },
        )
    }

    @Test
    fun `when no trial plays the usual stream still comes back`() {
        host.trialStartsAfterMs = listOf(
            "fast2160game", "fast2160forced60", "fast2160forced120", "fast2160limit120", "tunnel2160game",
            "tunnel1440hidden",
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
