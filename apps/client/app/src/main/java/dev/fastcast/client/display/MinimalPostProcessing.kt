package dev.fastcast.client.display

import android.os.Build
import android.view.Window

object MinimalPostProcessing {

    fun request(window: Window) {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.R) window.setPreferMinimalPostProcessing(true)
    }
}
