package dev.fastcast.client.debug

object PropertyChanges {

    private const val LISTED = 40
    private const val MISSING = "-"
    private const val UNCHANGED = "props unchanged"

    fun between(before: Map<String, String>, after: Map<String, String>): List<String> {
        val changes = (before.keys + after.keys)
            .sorted()
            .filter { before[it] != after[it] }
            .map { "$it: ${before[it] ?: MISSING} -> ${after[it] ?: MISSING}" }
        return when {
            changes.isEmpty() -> listOf(UNCHANGED)
            changes.size <= LISTED -> changes
            else -> changes.take(LISTED) + "and ${changes.size - LISTED} more changed props"
        }
    }
}
