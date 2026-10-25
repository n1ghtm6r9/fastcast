package dev.fastcast.client.player

import android.view.KeyEvent
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Test

class TheRemoteKeysChooseThePlayerActionsTest {

    @Test
    fun `ok, the arrows and the menu key open the menu`() {
        val keys = listOf(
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

        keys.forEach { key -> assertEquals("key $key", RemoteAction.ShowMenu, RemoteKeys.actionFor(key)) }
    }

    @Test
    fun `the info key toggles the stats`() {
        assertEquals(RemoteAction.ToggleStats, RemoteKeys.actionFor(KeyEvent.KEYCODE_INFO))
    }

    @Test
    fun `back, escape and stop leave the stream`() {
        val keys = listOf(KeyEvent.KEYCODE_BACK, KeyEvent.KEYCODE_ESCAPE, KeyEvent.KEYCODE_MEDIA_STOP)

        keys.forEach { key -> assertEquals("key $key", RemoteAction.Leave, RemoteKeys.actionFor(key)) }
    }

    @Test
    fun `volume, home and channel keys are left to the TV`() {
        val keys = listOf(
            KeyEvent.KEYCODE_VOLUME_UP,
            KeyEvent.KEYCODE_VOLUME_DOWN,
            KeyEvent.KEYCODE_VOLUME_MUTE,
            KeyEvent.KEYCODE_HOME,
            KeyEvent.KEYCODE_CHANNEL_UP,
            KeyEvent.KEYCODE_0,
        )

        keys.forEach { key -> assertNull("key $key", RemoteKeys.actionFor(key)) }
    }
}
