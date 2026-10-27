package dev.fastcast.client.stream

class DiagnoseMessage(val run: Long) {

    companion object {
        private const val TYPE = "type"
        private const val TYPE_DIAGNOSE = "diagnose"
        private const val RUN = "run"

        fun parse(text: String): DiagnoseMessage? {
            val json = JsonObjects.parse(text) ?: return null
            if (json.optString(TYPE) != TYPE_DIAGNOSE) return null
            val run = json.optLong(RUN).takeIf { it > 0 } ?: return null
            return DiagnoseMessage(run)
        }
    }
}
