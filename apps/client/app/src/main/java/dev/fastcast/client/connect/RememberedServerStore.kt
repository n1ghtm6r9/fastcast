package dev.fastcast.client.connect

import android.content.Context

class RememberedServerStore(context: Context) {

    private val preferences = context.getSharedPreferences(PREFERENCES_NAME, Context.MODE_PRIVATE)

    fun loadName(): String? = preferences.getString(KEY_NAME, null)

    fun saveName(name: String) {
        preferences.edit().putString(KEY_NAME, name).apply()
    }

    private companion object {
        const val PREFERENCES_NAME = "fastcast"
        const val KEY_NAME = "server_name"
    }
}
