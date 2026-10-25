package dev.fastcast.client.discovery

import dev.fastcast.client.testing.aServerReply
import dev.fastcast.client.testing.aServerReplyOfSize
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Test

class RepliesThatAreNotValidServerRepliesAreIgnoredTest {

    private val responder = FakeDiscoveryResponder()
    private val listener = RecordingServerFinderListener()
    private val finder = aFinderFor(responder, listener)

    @After
    fun tearDown() {
        finder.stop()
        responder.close()
    }

    @Test
    fun `garbage and foreign messages are skipped and the valid reply is listed`() {
        responder.answerWith(
            "garbage",
            """{"type":"discover","version":1}""",
            """{"type":"server","version":1,"name":"No port"}""",
            """{"type":"server","version":1,"name":"Bad port","port":70000}""",
            aServerReply("Gaming PC", 3278),
        )

        finder.start()

        assertEquals(listOf("Gaming PC"), listener.nextReport().map { it.name })
    }

    @Test
    fun `a reply larger than the limit is ignored even when its first bytes are a valid reply`() {
        responder.answerWithBytes(
            aServerReplyOfSize(2000, name = "Too big"),
            aServerReply("Small", 3278).toByteArray(Charsets.UTF_8),
        )

        finder.start()

        assertEquals(listOf("Small"), listener.nextReport().map { it.name })
    }

    @Test
    fun `a reply of exactly the limit is listed`() {
        responder.answerWithBytes(aServerReplyOfSize(1024, name = "Just fits"))

        finder.start()

        assertEquals(listOf("Just fits"), listener.nextReport().map { it.name })
    }
}
