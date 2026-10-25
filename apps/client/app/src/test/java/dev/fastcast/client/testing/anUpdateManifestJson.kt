package dev.fastcast.client.testing

import org.json.JSONObject

fun anUpdateManifestJson(vararg changes: Pair<String, Any?>): String {
    val json = JSONObject()
        .put("versionCode", 101)
        .put("versionName", "0.1.101")
        .put("url", "https://github.com/n1ghtm6r9/fastcast/releases/download/v0.1.101/fastcast-tv.apk")
        .put("sha256", "a".repeat(64))
        .put("size", 4_321_000L)
    changes.forEach { (key, value) -> if (value == null) json.remove(key) else json.put(key, value) }
    return json.toString()
}
