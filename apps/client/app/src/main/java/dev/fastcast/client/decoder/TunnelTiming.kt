package dev.fastcast.client.decoder

class TunnelTiming(val videoDelayUs: Long, val audioStart: AudioStart) {

    companion object {
        val USUAL = TunnelTiming(TunnelClock.VIDEO_DELAY_US, AudioStart.WITH_DECODER)
    }
}
