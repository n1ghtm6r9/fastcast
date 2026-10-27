package dev.fastcast.client.debug

import org.json.JSONObject
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertNull
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
                "step STREAM", "step FAST", "fast", "step RESTORE", "usual", "done",
            ),
            host.events,
        )
    }

    @Test
    fun `every step reports to the server in order`() {
        runToTheEnd()

        val steps = host.sent.map { JSONObject(it).getString("step") }.distinct()
        assertEquals(listOf("device", "display", "vsync", "ask120", "ask144", "stream", "fast", "done"), steps)
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
    fun `the fast stream report names what was asked and what came`() {
        runToTheEnd()

        val fast = host.sent.map(::JSONObject).first { it.getString("step") == "fast" }.getString("text")
        assertTrue(fast, fast.startsWith("asked 3840x2160@120; decoded 120.0/s"))
        assertTrue(fast, fast.contains("presented: "))
    }

    @Test
    fun `the end shows the decoded and the presented rates of the fast stream`() {
        host.presentIntervalNanos = 16_666_667L

        runToTheEnd()

        val result = requireNotNull(host.shownResult)
        assertEquals(120.0, result.decodedPerSecond, 0.5)
        assertEquals(60.0, result.presentedPerSecond, 0.5)
    }

    @Test
    fun `a fast stream that never plays is reported and the usual stream comes back`() {
        host.fastStartsAfterMs = null

        runToTheEnd()

        val fast = host.sent.map(::JSONObject).first { it.getString("step") == "fast" }
        assertTrue(fast.getString("text").contains("no video"))
        assertEquals(listOf("fast", "step RESTORE", "usual", "done"), host.events.takeLast(4))
        assertNull(host.shownResult)
    }

    @Test
    fun `a cancelled run sends nothing more and leaves the stream alone`() {
        run.start()
        clock.advanceBy(500L)
        run.cancel()
        val sentBefore = host.sent.size

        clock.advanceBy(WHOLE_RUN_MS)

        assertEquals(sentBefore, host.sent.size)
        assertFalse(host.events.contains("fast"))
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
        assertFalse(host.events.contains("fast"))
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

    companion object {
        private const val WHOLE_RUN_MS = 120_000L
    }
}
