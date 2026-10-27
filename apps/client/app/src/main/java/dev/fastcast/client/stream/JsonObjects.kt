package dev.fastcast.client.stream

import org.json.JSONException
import org.json.JSONObject

object JsonObjects {

    fun parse(text: String): JSONObject? = try {
        JSONObject(text)
    } catch (e: JSONException) {
        null
    }
}
