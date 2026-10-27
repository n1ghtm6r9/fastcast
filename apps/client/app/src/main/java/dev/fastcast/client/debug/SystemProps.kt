package dev.fastcast.client.debug

import java.util.concurrent.TimeUnit

object SystemProps {

    private const val STEP = "props"
    private const val GETPROP = "getprop"
    private const val WAIT_SECONDS = 3L

    fun snapshot(): Map<String, String> = try {
        PropertyLines.all(output())
    } catch (e: Exception) {
        emptyMap()
    }

    fun of(): FactSection = FactSection(
        STEP,
        try {
            PropertyLines.matching(output())
        } catch (e: Exception) {
            listOf("$GETPROP: ${e.javaClass.simpleName} ${e.message}")
        },
    )

    private fun output(): String {
        val process = ProcessBuilder(GETPROP).redirectErrorStream(true).start()
        val text = process.inputStream.bufferedReader().use { it.readText() }
        process.waitFor(WAIT_SECONDS, TimeUnit.SECONDS)
        return text
    }
}
