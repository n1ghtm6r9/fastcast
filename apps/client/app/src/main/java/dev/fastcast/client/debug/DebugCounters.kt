package dev.fastcast.client.debug

class DebugCounters(
    val nanos: Long,
    val bytes: Long,
    val queued: Long,
    val decoded: Long,
    val rendered: Long,
    val dropped: Long,
)
