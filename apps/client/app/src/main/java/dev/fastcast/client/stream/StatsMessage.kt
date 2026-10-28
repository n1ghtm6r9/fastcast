package dev.fastcast.client.stream

import kotlin.math.roundToLong
import org.json.JSONObject

class StatsMessage(
    val decoder: String,
    val mime: String,
    val width: Int,
    val height: Int,
    val fps: Double,
    val mbps: Double,
    val dropped: Long,
    val display: String,
    val receiveBuffer: Int? = null,
    val colorStandard: Int? = null,
    val colorRange: Int? = null,
    val colorTransfer: Int? = null,
    val network: String? = null,
    val error: String? = null,
    val delayMs: Double? = null,
    val tunneled: Boolean = false,
    val socketDrops: Int? = null,
    val inboxDrops: Int? = null,
    val linkPauseMs: Int? = null,
) {

    fun toJson(): String {
        val texts = linkedMapOf(DECODER to decoder, MIME to mime, DISPLAY to display)
        error?.let { texts[ERROR] = it }
        while (true) {
            val json = render(texts)
            val excess = json.toByteArray(Charsets.UTF_8).size - MAX_BYTES
            if (excess <= 0) return json
            val longest = texts.maxBy { it.value.length }
            if (longest.value.isEmpty()) return json
            val cut = (excess + MOST_BYTES_PER_CHAR - 1) / MOST_BYTES_PER_CHAR
            texts[longest.key] = withoutTheLast(cut, longest.value)
        }
    }

    private fun render(texts: Map<String, String>): String {
        val json = JSONObject()
            .put(TYPE, TYPE_STATS)
            .put(DECODER, texts.getValue(DECODER))
            .put(MIME, texts.getValue(MIME))
            .put(WIDTH, width)
            .put(HEIGHT, height)
            .put(FPS, roundedToTenths(fps))
            .put(MBPS, roundedToTenths(mbps))
            .put(DROPPED, dropped)
            .put(DISPLAY, texts.getValue(DISPLAY))
        receiveBuffer?.let { json.put(RECEIVE_BUFFER, it) }
        socketDrops?.let { json.put(SOCKET_DROPS, it) }
        inboxDrops?.let { json.put(INBOX_DROPS, it) }
        linkPauseMs?.let { json.put(LINK_PAUSE_MS, it) }
        colorStandard?.let { json.put(COLOR_STANDARD, it) }
        colorRange?.let { json.put(COLOR_RANGE, it) }
        colorTransfer?.let { json.put(COLOR_TRANSFER, it) }
        network?.let { json.put(NETWORK, it) }
        delayMs?.let { json.put(DELAY_MS, roundedToTenths(it)) }
        if (tunneled) json.put(TUNNELED, true)
        texts[ERROR]?.let { json.put(ERROR, it) }
        return json.toString()
    }

    private fun withoutTheLast(count: Int, text: String): String {
        val kept = (text.length - count).coerceAtLeast(0)
        val end = if (kept > 0 && text[kept - 1].isHighSurrogate()) kept - 1 else kept
        return text.substring(0, end)
    }

    private fun roundedToTenths(value: Double): Double = (value * TENTHS_PER_UNIT).roundToLong() / TENTHS_PER_UNIT

    companion object {
        const val MAX_BYTES = 1388
        private const val MOST_BYTES_PER_CHAR = 6
        private const val TYPE = "type"
        private const val TYPE_STATS = "stats"
        private const val DECODER = "decoder"
        private const val MIME = "mime"
        private const val WIDTH = "width"
        private const val HEIGHT = "height"
        private const val FPS = "fps"
        private const val MBPS = "mbps"
        private const val DROPPED = "dropped"
        private const val DISPLAY = "display"
        private const val RECEIVE_BUFFER = "receiveBuffer"
        private const val SOCKET_DROPS = "socketDrops"
        private const val INBOX_DROPS = "inboxDrops"
        private const val LINK_PAUSE_MS = "linkPauseMs"
        private const val COLOR_STANDARD = "colorStandard"
        private const val COLOR_RANGE = "colorRange"
        private const val COLOR_TRANSFER = "colorTransfer"
        private const val NETWORK = "network"
        private const val ERROR = "error"
        private const val DELAY_MS = "delayMs"
        private const val TUNNELED = "tunneled"
        private const val TENTHS_PER_UNIT = 10.0
    }
}
