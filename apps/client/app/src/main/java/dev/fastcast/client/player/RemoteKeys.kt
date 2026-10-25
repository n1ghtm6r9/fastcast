package dev.fastcast.client.player

import android.view.KeyEvent

object RemoteKeys {

    private val menuKeys = setOf(
        KeyEvent.KEYCODE_DPAD_CENTER,
        KeyEvent.KEYCODE_ENTER,
        KeyEvent.KEYCODE_NUMPAD_ENTER,
        KeyEvent.KEYCODE_BUTTON_A,
        KeyEvent.KEYCODE_MENU,
        KeyEvent.KEYCODE_DPAD_UP,
        KeyEvent.KEYCODE_DPAD_DOWN,
        KeyEvent.KEYCODE_DPAD_LEFT,
        KeyEvent.KEYCODE_DPAD_RIGHT,
    )
    private val leaveKeys = setOf(
        KeyEvent.KEYCODE_BACK,
        KeyEvent.KEYCODE_ESCAPE,
        KeyEvent.KEYCODE_MEDIA_STOP,
    )

    fun actionFor(keyCode: Int): RemoteAction? = when (keyCode) {
        in menuKeys -> RemoteAction.ShowMenu
        KeyEvent.KEYCODE_INFO -> RemoteAction.ToggleStats
        in leaveKeys -> RemoteAction.Leave
        else -> null
    }
}
