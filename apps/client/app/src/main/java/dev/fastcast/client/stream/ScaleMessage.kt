package dev.fastcast.client.stream

class ScaleMessage(
    val percent: Int,
    val min: Int,
    val max: Int,
) {

    companion object {
        private const val TYPE = "type"
        private const val TYPE_SCALE = "scale"
        private const val PERCENT = "percent"
        private const val MIN = "min"
        private const val MAX = "max"

        fun parse(text: String): ScaleMessage? {
            val json = JsonObjects.parse(text) ?: return null
            if (json.optString(TYPE) != TYPE_SCALE) return null
            val percent = json.optInt(PERCENT).takeIf { it > 0 } ?: return null
            val min = json.optInt(MIN).takeIf { it > 0 } ?: return null
            val max = json.optInt(MAX).takeIf { it > 0 } ?: return null
            return ScaleMessage(percent, min, max)
        }
    }
}
