package dev.fastcast.client.decoder

class TunnelTiming(val videoDelayUs: Long, val videoStart: VideoStart) {

    companion object {
        val USUAL = TunnelTiming(TunnelClock.VIDEO_DELAY_US, VideoStart.ON_RUNNING_CLOCK)
    }
}
