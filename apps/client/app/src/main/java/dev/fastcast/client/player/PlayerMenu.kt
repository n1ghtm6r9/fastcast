package dev.fastcast.client.player

import android.os.Handler
import android.view.View

class PlayerMenu(
    private val panel: View,
    private val firstButton: View,
    private val handler: Handler,
) {

    private val autoClose = Runnable { close() }

    val isOpen: Boolean
        get() = panel.visibility == View.VISIBLE

    fun open() {
        panel.visibility = View.VISIBLE
        firstButton.requestFocus()
        keepOpen()
    }

    fun keepOpen() {
        handler.removeCallbacks(autoClose)
        handler.postDelayed(autoClose, AUTO_CLOSE_MS)
    }

    fun close() {
        handler.removeCallbacks(autoClose)
        panel.visibility = View.GONE
    }

    private companion object {
        const val AUTO_CLOSE_MS = 8_000L
    }
}
