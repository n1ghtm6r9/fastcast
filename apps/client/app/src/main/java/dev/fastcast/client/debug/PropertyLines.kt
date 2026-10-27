package dev.fastcast.client.debug

object PropertyLines {

    private val PROPERTY = Regex("""\[(.+?)]: \[(.*)]""")

    fun matching(output: String): List<String> =
        pairsOf(output)
            .filter { (key, _) -> DebugKeys.interesting(key) }
            .map { (key, value) -> "$key=$value" }
            .toList()

    fun all(output: String): Map<String, String> = pairsOf(output).toMap()

    private fun pairsOf(output: String): Sequence<Pair<String, String>> =
        output.lineSequence()
            .mapNotNull { PROPERTY.matchEntire(it.trim()) }
            .map { it.groupValues[1] to it.groupValues[2] }
}
