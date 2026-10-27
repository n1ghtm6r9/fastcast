package dev.fastcast.client.debug

import android.content.ContentResolver
import android.content.Context
import android.net.Uri
import android.provider.Settings

object SettingsFacts {

    private const val STEP = "settings"
    private const val SYSTEM = "system"
    private const val NO_ROWS = "no rows"
    private val TABLES = listOf(
        SYSTEM to Settings.System.CONTENT_URI,
        "global" to Settings.Global.CONTENT_URI,
        "secure" to Settings.Secure.CONTENT_URI,
    )
    private val KNOWN_SYSTEM_KEYS = listOf("peak_refresh_rate", "min_refresh_rate")

    fun of(context: Context): FactSection {
        val resolver = context.contentResolver
        val rows = TABLES.flatMap { (table, uri) -> rowsOf(resolver, table, uri) }
        return FactSection(STEP, (rows + knownSystemRows(resolver)).distinct())
    }

    private fun rowsOf(resolver: ContentResolver, table: String, uri: Uri): List<String> = try {
        resolver.query(uri, arrayOf(Settings.NameValueTable.NAME, Settings.NameValueTable.VALUE), null, null, null)
            ?.use { cursor ->
                buildList {
                    while (cursor.moveToNext()) {
                        val key = cursor.getString(0) ?: continue
                        if (DebugKeys.interesting(key)) add("$table.$key=${cursor.getString(1)}")
                    }
                }
            }
            ?: listOf("$table: $NO_ROWS")
    } catch (e: Exception) {
        listOf("$table: ${e.javaClass.simpleName} ${e.message}")
    }

    private fun knownSystemRows(resolver: ContentResolver): List<String> = KNOWN_SYSTEM_KEYS.map { key ->
        val value = runCatching { Settings.System.getString(resolver, key) }.getOrElse { it.javaClass.simpleName }
        "$SYSTEM.$key=$value"
    }
}
