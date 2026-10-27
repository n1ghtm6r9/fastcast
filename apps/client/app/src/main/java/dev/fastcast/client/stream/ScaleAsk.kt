package dev.fastcast.client.stream

import org.json.JSONObject

class ScaleAsk(private val step: Int) {

    fun toJson(): String = JSONObject().put(TYPE, TYPE_SCALE).put(STEP, step).toString()

    private companion object {
        const val TYPE = "type"
        const val TYPE_SCALE = "scale"
        const val STEP = "step"
    }
}
