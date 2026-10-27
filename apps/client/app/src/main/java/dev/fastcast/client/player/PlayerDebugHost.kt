package dev.fastcast.client.player

import android.app.Activity
import dev.fastcast.client.debug.DebugCounters
import dev.fastcast.client.debug.DebugHost
import dev.fastcast.client.debug.DebugStep
import dev.fastcast.client.debug.DecoderFacts
import dev.fastcast.client.debug.DisplayFacts
import dev.fastcast.client.debug.FactSection
import dev.fastcast.client.debug.FastStreamResult
import dev.fastcast.client.debug.FeatureFacts
import dev.fastcast.client.debug.PlatformFacts
import dev.fastcast.client.debug.SettingsFacts
import dev.fastcast.client.debug.SystemProps
import dev.fastcast.client.debug.VsyncMeter
import dev.fastcast.client.display.DisplayModeOption
import dev.fastcast.client.display.DisplayModes
import dev.fastcast.client.display.FrameRateHint

class PlayerDebugHost(
    private val activity: Activity,
    private val player: DebugPlayer,
    private val overlay: DebugOverlay,
) : DebugHost {

    private var savedWindowAsk: Pair<Int, Float>? = null

    override fun factSections(): List<FactSection> = listOf(
        PlatformFacts.of(activity),
        DisplayFacts.of(activity),
        DecoderFacts.of(),
        SettingsFacts.of(activity),
        SystemProps.of(),
        FeatureFacts.of(activity),
    )

    override fun measureVsync(durationMs: Long, done: (LongArray) -> Unit) = VsyncMeter.measure(durationMs, done)

    override fun askRefreshRate(rate: Float) {
        val window = activity.window
        val attributes = window.attributes
        if (savedWindowAsk == null) savedWindowAsk = attributes.preferredDisplayModeId to attributes.preferredRefreshRate
        attributes.preferredDisplayModeId = modeReaching(rate)?.modeId ?: NO_MODE
        attributes.preferredRefreshRate = rate
        window.attributes = attributes
        player.outputSurface()?.let { FrameRateHint.apply(it, rate) }
    }

    override fun clearRefreshRate() {
        val (modeId, rate) = savedWindowAsk ?: return
        savedWindowAsk = null
        val window = activity.window
        val attributes = window.attributes
        attributes.preferredDisplayModeId = modeId
        attributes.preferredRefreshRate = rate
        window.attributes = attributes
        val surface = player.outputSurface() ?: return
        player.usualRefreshRate()?.let { FrameRateHint.apply(surface, it) }
    }

    override fun displayNow(): String {
        val display = DisplayModes.defaultDisplay(activity) ?: return NO_DISPLAY
        val mode = display.mode
        val attributes = activity.window.attributes
        return "now ${mode.physicalWidth}x${mode.physicalHeight}@${mode.refreshRate} mode ${mode.modeId}, " +
            "display ${display.refreshRate}, window asks mode ${attributes.preferredDisplayModeId} " +
            "rate ${attributes.preferredRefreshRate}"
    }

    override fun isPlaying(): Boolean = player.isPlaying()

    override fun send(message: String): Boolean = player.currentSession()?.send(message) ?: false

    override fun counters(): DebugCounters? = player.currentSession()?.debugCounters()

    override fun presentedSince(startNanos: Long): LongArray =
        player.currentSession()?.presentedSince(startNanos) ?: LongArray(0)

    override fun streamFast(): String = player.streamFast()

    override fun streamUsual() = player.streamUsual()

    override fun showProgress(step: DebugStep) = overlay.showProgress(step)

    override fun showDone(result: FastStreamResult?) = overlay.showDone(result)

    private fun modeReaching(rate: Float): DisplayModeOption? {
        val display = DisplayModes.defaultDisplay(activity) ?: return null
        val active = DisplayModes.active(display)
        return DisplayModes.supported(display)
            .filter { it.refreshRate >= rate - RATE_TOLERANCE }
            .sortedWith(compareBy({ it.width != active.width || it.height != active.height }, { it.refreshRate }))
            .firstOrNull()
    }

    private companion object {
        const val NO_MODE = 0
        const val RATE_TOLERANCE = 0.5f
        const val NO_DISPLAY = "no display"
    }
}
