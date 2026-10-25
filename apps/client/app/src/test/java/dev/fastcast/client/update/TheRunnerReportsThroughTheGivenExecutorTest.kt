package dev.fastcast.client.update

import dev.fastcast.client.discovery.QueueingExecutor
import dev.fastcast.client.testing.anUpdateRunner
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test

class TheRunnerReportsThroughTheGivenExecutorTest {

    private val listener = RecordingUpdateListener()
    private val executor = QueueingExecutor()
    private val runner = anUpdateRunner(listener, deliver = executor, fetch = { null })

    @After
    fun tearDown() = runner.stop()

    @Test
    fun `nothing reaches the listener before the executor runs the report`() {
        runner.start()
        executor.next()

        assertTrue(listener.hasNoEventWithin(200L))
    }

    @Test
    fun `the report reaches the listener when the executor runs it`() {
        runner.start()

        executor.next().run()

        assertEquals(RecordingUpdateListener.NO_UPDATE, listener.nextEvent())
    }

    @Test
    fun `a report still queued when the runner stops is dropped`() {
        runner.start()
        val queued = executor.next()

        runner.stop()
        queued.run()

        assertTrue(listener.hasNoEventWithin(200L))
    }
}
