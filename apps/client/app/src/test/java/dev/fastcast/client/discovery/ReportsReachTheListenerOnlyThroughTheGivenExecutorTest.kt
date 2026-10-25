package dev.fastcast.client.discovery

import dev.fastcast.client.testing.aServerReply
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test

class ReportsReachTheListenerOnlyThroughTheGivenExecutorTest {

    private val responder = FakeDiscoveryResponder()
    private val listener = RecordingServerFinderListener()
    private val executor = QueueingExecutor()
    private val finder = aFinderFor(responder, listener, deliver = executor)

    @After
    fun tearDown() {
        finder.stop()
        responder.close()
    }

    @Test
    fun `the listener hears nothing until the executor runs the report`() {
        responder.answerWith(aServerReply("Gaming PC", 3278))
        finder.start()

        val report = executor.next()

        assertTrue(listener.hasNoReportWithin(150L))
        report.run()
        assertEquals(listOf("Gaming PC"), listener.nextReport().map { it.name })
    }

    @Test
    fun `a report queued before the finder stopped is dropped when the executor finally runs it`() {
        responder.answerWith(aServerReply("Gaming PC", 3278))
        finder.start()
        val report = executor.next()

        finder.stop()
        report.run()

        assertTrue(listener.hasNoReportWithin(200L))
    }
}
