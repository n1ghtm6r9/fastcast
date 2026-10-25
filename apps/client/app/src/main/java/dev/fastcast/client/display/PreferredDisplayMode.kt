package dev.fastcast.client.display

import android.view.Window

object PreferredDisplayMode {

    fun apply(window: Window, mode: DisplayModeOption) {
        val attributes = window.attributes
        attributes.preferredDisplayModeId = mode.modeId
        window.attributes = attributes
    }
}
