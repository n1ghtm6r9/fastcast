package dev.fastcast.client.player

interface PlayerSessionListener {

    fun onStatus(status: PlayerStatus)

    fun onVideoSize(width: Int, height: Int)
}
