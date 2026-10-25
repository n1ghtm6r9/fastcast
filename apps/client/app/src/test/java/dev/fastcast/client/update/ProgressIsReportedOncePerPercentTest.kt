package dev.fastcast.client.update

import dev.fastcast.client.testing.anUpdateManifest
import dev.fastcast.client.testing.anUpdateRunner
import java.io.File
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test

class ProgressIsReportedOncePerPercentTest {

    private val listener = RecordingUpdateListener()
    private var steps: ((Long, Long) -> Unit) -> Unit = {}
    private val runner = anUpdateRunner(
        listener,
        fetch = { anUpdateManifest(versionCode = 101) },
        download = { _, onProgress, _ ->
            steps(onProgress)
            File("unused")
        },
        install = { _, _ -> },
    )

    @After
    fun tearDown() = runner.stop()

    @Test
    fun `every percent is reported exactly once`() {
        steps = { progress -> (1..250).forEach { progress(it * 4L, 1_000L) } }

        runner.start()

        assertEquals("found 0.1.101", listener.nextEvent())
        assertEquals((0..100).map { "progress $it" }, List(101) { listener.nextEvent() })
        assertTrue(listener.hasNoEventWithin(200L))
    }

    @Test
    fun `a percent that did not change is not repeated`() {
        steps = { progress ->
            progress(10L, 1_000L)
            progress(11L, 1_000L)
            progress(19L, 1_000L)
            progress(20L, 1_000L)
        }

        runner.start()

        assertEquals(
            listOf("found 0.1.101", "progress 1", "progress 2"),
            List(3) { listener.nextEvent() },
        )
        assertTrue(listener.hasNoEventWithin(200L))
    }

    @Test
    fun `progress beyond the total is capped at one hundred percent`() {
        steps = { progress -> progress(150L, 100L) }

        runner.start()

        assertEquals(listOf("found 0.1.101", "progress 100"), List(2) { listener.nextEvent() })
    }

    @Test
    fun `a download without progress reports no percent`() {
        steps = {}

        runner.start()

        assertEquals("found 0.1.101", listener.nextEvent())
        assertTrue(listener.hasNoEventWithin(200L))
    }
}
