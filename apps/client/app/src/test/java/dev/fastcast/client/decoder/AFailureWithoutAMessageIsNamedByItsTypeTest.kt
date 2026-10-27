package dev.fastcast.client.decoder

import org.junit.Assert.assertEquals
import org.junit.Test

class AFailureWithoutAMessageIsNamedByItsTypeTest {

    @Test
    fun `a failure with a message is told by its message`() {
        assertEquals("No decoder for video/hevc", FailureText.of(UnsupportedStreamException("No decoder for video/hevc")))
    }

    @Test
    fun `a failure without a message is named by its type`() {
        assertEquals("IllegalStateException", FailureText.of(IllegalStateException()))
    }

    @Test
    fun `a failure with a blank message is named by its type`() {
        assertEquals("IllegalArgumentException", FailureText.of(IllegalArgumentException(" ")))
    }
}
