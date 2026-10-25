package dev.fastcast.client.update

import dev.fastcast.client.testing.anUpdateRunner
import java.io.File
import java.util.concurrent.atomic.AtomicInteger
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test

class TheRunnerReportsWhenThereIsNoUpdateTest {

    private val listener = RecordingUpdateListener()
    private val downloads = AtomicInteger()
    private val installs = AtomicInteger()
    private val runner = anUpdateRunner(
        listener,
        fetch = { null },
        download = { _, _, _ ->
            downloads.incrementAndGet()
            File("unused")
        },
        install = { _, _ -> installs.incrementAndGet() },
    )

    @After
    fun tearDown() = runner.stop()

    @Test
    fun `no newer version is reported as no update`() {
        runner.start()

        assertEquals(RecordingUpdateListener.NO_UPDATE, listener.nextEvent())
    }

    @Test
    fun `nothing else is reported after the no update report`() {
        runner.start()
        listener.nextEvent()

        assertTrue(listener.hasNoEventWithin(200L))
    }

    @Test
    fun `nothing is downloaded or installed without a newer version`() {
        runner.start()
        listener.nextEvent()
        listener.hasNoEventWithin(200L)

        assertEquals(listOf(0, 0), listOf(downloads.get(), installs.get()))
    }
}
