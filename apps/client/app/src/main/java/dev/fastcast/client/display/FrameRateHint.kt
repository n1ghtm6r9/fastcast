package dev.fastcast.client.display

import android.os.Build
import android.view.Surface
import dev.fastcast.client.decoder.runQuietly

object FrameRateHint {

    fun apply(surface: Surface, refreshRate: Float) {
        runQuietly {
            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
                surface.setFrameRate(
                    refreshRate,
                    Surface.FRAME_RATE_COMPATIBILITY_FIXED_SOURCE,
                    Surface.CHANGE_FRAME_RATE_ALWAYS,
                )
            } else if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.R) {
                surface.setFrameRate(refreshRate, Surface.FRAME_RATE_COMPATIBILITY_FIXED_SOURCE)
            }
        }
    }
}
