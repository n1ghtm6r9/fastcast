package dev.fastcast.client.debug

import org.json.JSONObject

object DebugMessages {

    const val MAX_BYTES = 600
    private const val TYPE = "type"
    private const val TYPE_DEBUG = "debug"
    private const val STEP = "step"
    private const val PART = "part"
    private const val TEXT = "text"
    private const val SEPARATOR = "; "
    private const val WIDEST_PART = "000/000"

    fun pack(step: String, facts: List<String>): List<String> {
        val texts = textsOf(step, facts)
        return texts.mapIndexed { index, text -> render(step, "${index + 1}/${texts.size}", text) }
    }

    private fun textsOf(step: String, facts: List<String>): List<String> {
        val texts = mutableListOf<String>()
        var current: String? = null
        for (fact in facts) {
            val joined = current?.let { it + SEPARATOR + fact } ?: fact
            if (fits(step, joined)) {
                current = joined
                continue
            }
            current?.let(texts::add)
            current = null
            if (fits(step, fact)) current = fact else texts += piecesOf(step, fact)
        }
        current?.let(texts::add)
        return texts.ifEmpty { listOf("") }
    }

    private fun piecesOf(step: String, fact: String): List<String> {
        val pieces = mutableListOf<String>()
        var rest = fact
        while (rest.isNotEmpty()) {
            val length = wholeCharacters(rest, longestFittingPrefix(step, rest))
            pieces += rest.substring(0, length)
            rest = rest.substring(length)
        }
        return pieces
    }

    private fun longestFittingPrefix(step: String, text: String): Int {
        var low = 1
        var high = text.length
        while (low < high) {
            val middle = (low + high + 1) / 2
            if (fits(step, text.substring(0, middle))) low = middle else high = middle - 1
        }
        return low
    }

    private fun wholeCharacters(text: String, length: Int): Int = when {
        length >= text.length || !text[length - 1].isHighSurrogate() -> length
        length > 1 -> length - 1
        else -> length + 1
    }

    private fun fits(step: String, text: String): Boolean =
        render(step, WIDEST_PART, text).toByteArray(Charsets.UTF_8).size <= MAX_BYTES

    private fun render(step: String, part: String, text: String): String =
        JSONObject().put(TYPE, TYPE_DEBUG).put(STEP, step).put(PART, part).put(TEXT, text).toString()
}
