package dev.fastcast.client.decoder

class DecoderSetup(
    val pace: DecoderPace,
    val tweak: DecoderTweak,
    val inspectsVendorParameters: Boolean,
    val timing: TunnelTiming = TunnelTiming.USUAL,
) {

    companion object {
        val USUAL = DecoderSetup(DecoderPace.REALTIME, DecoderTweak.NONE, inspectsVendorParameters = false)
    }
}
