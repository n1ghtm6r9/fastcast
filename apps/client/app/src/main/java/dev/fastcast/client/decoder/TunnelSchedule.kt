package dev.fastcast.client.decoder

import java.util.Locale
import kotlin.math.abs
import kotlin.math.roundToLong

class TunnelSchedule(private val videoDelayUs: Long = TunnelClock.VIDEO_DELAY_US) {

    private var anchored = false
    private var onClock = false
    private var offsetUs = 0L
    private var lastFrameUs = 0L

    @Volatile
    private var slackUs = videoDelayUs.toDouble()

    @Volatile
    private var anchors = 0

    @Volatile
    private var clockAnchors = 0

    @Volatile
    private var retimes = 0

    fun presentationUs(frameUs: Long, clockUs: Long?): Long {
        if (needsAnchor(frameUs, clockUs)) {
            anchor(frameUs, clockUs)
        } else if (clockUs != null) {
            follow(frameUs + offsetUs - clockUs)
        }
        lastFrameUs = frameUs
        return frameUs + offsetUs
    }

    fun restart() {
        anchored = false
    }

    fun describe(): String =
        DESCRIPTION.format(Locale.ROOT, anchors, clockAnchors, retimes, slackUs / MICROS_PER_MILLISECOND)

    private fun needsAnchor(frameUs: Long, clockUs: Long?): Boolean =
        !anchored || (clockUs != null && !onClock) || frameUs < lastFrameUs || frameUs - lastFrameUs > TIMELINE_JUMP_US

    private fun anchor(frameUs: Long, clockUs: Long?) {
        offsetUs = (clockUs ?: TunnelClock.AUDIO_START_US) + videoDelayUs - frameUs
        anchored = true
        onClock = clockUs != null
        slackUs = videoDelayUs.toDouble()
        anchors++
        if (onClock) clockAnchors++
    }

    private fun follow(sampleUs: Long) {
        slackUs += (sampleUs - slackUs) / SMOOTHING_FRAMES
        if (abs(slackUs - videoDelayUs) <= videoDelayUs / 2.0) return
        offsetUs += (videoDelayUs - slackUs).roundToLong()
        slackUs = videoDelayUs.toDouble()
        retimes++
    }

    private companion object {
        const val SMOOTHING_FRAMES = 512.0
        const val TIMELINE_JUMP_US = 1_000_000L
        const val MICROS_PER_MILLISECOND = 1_000.0
        const val DESCRIPTION = "video anchors %d (%d on the running audio clock), retimes %d, slack %.1f ms"
    }
}
