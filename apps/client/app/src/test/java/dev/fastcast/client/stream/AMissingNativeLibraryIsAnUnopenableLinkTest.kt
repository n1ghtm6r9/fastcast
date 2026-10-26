package dev.fastcast.client.stream

import dev.fastcast.client.testing.aStreamTarget
import org.junit.Assert.assertEquals
import org.junit.Test

class AMissingNativeLibraryIsAnUnopenableLinkTest {

    @Test
    fun `opening a link without the native library gives no handle`() {
        assertEquals(NativeLink.NO_HANDLE, NativeStreamLink.open(aStreamTarget()))
    }

    @Test
    fun `opening again after the library failed to load still gives no handle`() {
        NativeStreamLink.open(aStreamTarget())

        assertEquals(NativeLink.NO_HANDLE, NativeStreamLink.open(aStreamTarget()))
    }

    @Test
    fun `a client that cannot load the native library reports a lost connection and retries`() {
        val listener = RecordingStreamListener()
        val client = NativeStreamClient(
            { aStreamTarget() },
            listener,
            NativeStreamLink,
            ReconnectBackoff(listOf(40L, 80L)),
        )

        try {
            client.start()

            assertEquals("Could not open the stream", listener.nextReason())
            assertEquals(40L, listener.nextRetryDelay())
        } finally {
            client.stop()
        }
    }
}
