package dev.fastcast.client.network

object LinkSpeed {

    fun megabitsFrom(text: String?): Int? = text?.trim()?.toIntOrNull()?.takeIf { it > 0 }
}
