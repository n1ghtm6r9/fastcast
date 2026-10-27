package dev.fastcast.client.debug

object PropertyLines {

    private val PROPERTY = Regex("""\[(.+?)]: \[(.*)]""")

    fun matching(output: String): List<String> =
        output.lineSequence()
            .mapNotNull { PROPERTY.matchEntire(it.trim()) }
            .map { it.groupValues[1] to it.groupValues[2] }
            .filter { (key, _) -> DebugKeys.interesting(key) }
            .map { (key, value) -> "$key=$value" }
            .toList()
}
