package dev.fastcast.client.player

sealed interface PlayerStatus {

    data object Connecting : PlayerStatus

    data object WaitingForVideo : PlayerStatus

    data object Playing : PlayerStatus

    data class Reconnecting(val retryInMs: Long, val reason: String) : PlayerStatus

    data class DecoderError(val message: String) : PlayerStatus
}
