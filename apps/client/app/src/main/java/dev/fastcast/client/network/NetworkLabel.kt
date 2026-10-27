package dev.fastcast.client.network

object NetworkLabel {

    fun of(transport: String, megabits: Int?): String = if (megabits == null) transport else "$transport $megabits Mbit/s"
}
