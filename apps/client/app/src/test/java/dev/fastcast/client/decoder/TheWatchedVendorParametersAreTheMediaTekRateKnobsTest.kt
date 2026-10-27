package dev.fastcast.client.decoder

import org.junit.Assert.assertEquals
import org.junit.Test

class TheWatchedVendorParametersAreTheMediaTekRateKnobsTest {

    @Test
    fun `only the rate and latency knobs the decoder exposes are watched, in its order`() {
        val supported = listOf(
            "vendor.mtk-dtv.afd",
            "vendor.mtk-codec2.max-frame-rate",
            "vendor.mtk-codec2.game-mode",
            "vendor.mtk-watermark.enable",
            "vendor.mtk-pq.restriction",
        )

        assertEquals(
            listOf("vendor.mtk-codec2.max-frame-rate", "vendor.mtk-codec2.game-mode", "vendor.mtk-pq.restriction"),
            WatchedVendorParameters.among(supported),
        )
    }

    @Test
    fun `every knob a trial changes is watched`() {
        val changed = DecoderTweak.values().flatMap { it.vendorValues(120).keys }.distinct()

        assertEquals(changed, WatchedVendorParameters.among(changed))
    }

    @Test
    fun `a decoder without MediaTek knobs has nothing to watch`() {
        assertEquals(
            emptyList<String>(),
            WatchedVendorParameters.among(listOf("vendor.qti-ext-dec-low-latency.enable")),
        )
    }
}
