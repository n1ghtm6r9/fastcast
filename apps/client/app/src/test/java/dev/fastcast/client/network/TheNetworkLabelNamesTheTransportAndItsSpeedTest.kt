package dev.fastcast.client.network

import org.junit.Assert.assertEquals
import org.junit.Test

class TheNetworkLabelNamesTheTransportAndItsSpeedTest {

    @Test
    fun `a known speed follows the transport`() {
        assertEquals("ethernet 100 Mbit/s", NetworkLabel.of("ethernet", 100))
    }

    @Test
    fun `an unknown speed leaves the transport alone`() {
        assertEquals("wifi", NetworkLabel.of("wifi", null))
    }
}
