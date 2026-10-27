package dev.fastcast.client.display

import android.content.Context

class ScreenModeStore(context: Context) {

    private val preferences = context.getSharedPreferences(PREFERENCES_NAME, Context.MODE_PRIVATE)

    fun load(): String? = preferences.getString(KEY_SCREEN_MODE, null)

    fun save(mode: DisplayModeOption) {
        preferences.edit().putString(KEY_SCREEN_MODE, mode.label).apply()
    }

    private companion object {
        const val PREFERENCES_NAME = "fastcast"
        const val KEY_SCREEN_MODE = "screen_mode"
    }
}
