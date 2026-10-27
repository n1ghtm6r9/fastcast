package dev.fastcast.client.decoder

class DecoderSetup(
    val pace: DecoderPace,
    val tweak: DecoderTweak,
    val inspectsVendorParameters: Boolean,
) {

    companion object {
        val USUAL = DecoderSetup(DecoderPace.REALTIME, DecoderTweak.NONE, inspectsVendorParameters = false)
    }
}
