package dev.fastcast.client.player

import android.annotation.TargetApi
import android.os.Build
import android.view.View
import android.view.Window
import android.view.WindowInsets
import android.view.WindowInsetsController

object ImmersiveMode {

    @Suppress("DEPRECATION")
    private const val LEGACY_FLAGS = View.SYSTEM_UI_FLAG_IMMERSIVE_STICKY or
        View.SYSTEM_UI_FLAG_FULLSCREEN or
        View.SYSTEM_UI_FLAG_HIDE_NAVIGATION or
        View.SYSTEM_UI_FLAG_LAYOUT_STABLE or
        View.SYSTEM_UI_FLAG_LAYOUT_FULLSCREEN or
        View.SYSTEM_UI_FLAG_LAYOUT_HIDE_NAVIGATION

    fun enter(window: Window) {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.R) {
            hideSystemBars(window)
        } else {
            hideLegacySystemBars(window)
        }
    }

    @TargetApi(Build.VERSION_CODES.R)
    private fun hideSystemBars(window: Window) {
        val controller = window.insetsController ?: return
        controller.hide(WindowInsets.Type.systemBars())
        controller.systemBarsBehavior = WindowInsetsController.BEHAVIOR_SHOW_TRANSIENT_BARS_BY_SWIPE
    }

    @Suppress("DEPRECATION")
    private fun hideLegacySystemBars(window: Window) {
        window.decorView.systemUiVisibility = LEGACY_FLAGS
    }
}
