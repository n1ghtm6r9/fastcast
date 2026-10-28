package dev.fastcast.client.debug

import dev.fastcast.client.stream.StreamRequest

object TrialRequest {

    fun of(trial: StreamTrial): StreamRequest = StreamRequest(
        trial.width,
        trial.height,
        trial.fps,
        listOf(trial.codec),
        if (trial.tenBit) setOf(trial.codec) else emptySet(),
    )
}
