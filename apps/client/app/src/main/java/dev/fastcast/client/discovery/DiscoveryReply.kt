package dev.fastcast.client.discovery

import org.json.JSONException
import org.json.JSONObject

object DiscoveryReply {
    private const val TYPE = "type"
    private const val TYPE_SERVER = "server"
    private const val NAME = "name"
    private const val PORT = "port"
    private const val MAX_PORT = 65_535L

    fun parse(data: ByteArray, host: String): DiscoveredServer? {
        if (data.size > DiscoveryProtocol.MAX_REPLY_BYTES) return null
        val json = parseJson(String(data, Charsets.UTF_8)) ?: return null
        if (json.optString(TYPE) != TYPE_SERVER) return null
        val name = (json.opt(NAME) as? String)?.takeIf { it.isNotBlank() } ?: return null
        val port = parsePort(json.opt(PORT)) ?: return null
        return DiscoveredServer(name, host, port)
    }

    private fun parseJson(text: String): JSONObject? = try {
        JSONObject(text)
    } catch (e: JSONException) {
        null
    }

    private fun parsePort(value: Any?): Int? =
        (value as? Number)?.toLong()?.takeIf { it in 1..MAX_PORT }?.toInt()
}
