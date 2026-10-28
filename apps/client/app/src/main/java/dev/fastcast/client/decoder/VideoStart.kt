package dev.fastcast.client.decoder

enum class VideoStart(val clockWaitNanos: Long) {
    ON_RUNNING_CLOCK(1_000_000_000L),
    AT_ONCE(0L),
}
