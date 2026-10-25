package dev.fastcast.client.update

import android.content.Context

class RejectedVersionStore(context: Context) {

    private val preferences = context.getSharedPreferences(PREFERENCES_NAME, Context.MODE_PRIVATE)

    val versions = RejectedVersions(read = ::read, write = ::write)

    private fun read(): Set<Int> =
        preferences.getStringSet(KEY_VERSIONS, null).orEmpty().mapNotNull(String::toIntOrNull).toSet()

    private fun write(versionCodes: Set<Int>) {
        preferences.edit().putStringSet(KEY_VERSIONS, versionCodes.map(Int::toString).toSet()).apply()
    }

    private companion object {
        const val PREFERENCES_NAME = "fastcast_update"
        const val KEY_VERSIONS = "rejected_versions"
    }
}
