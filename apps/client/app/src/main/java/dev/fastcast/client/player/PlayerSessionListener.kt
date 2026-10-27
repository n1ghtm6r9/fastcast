package dev.fastcast.client.player

import dev.fastcast.client.stream.ScaleMessage

interface PlayerSessionListener {

    fun onStatus(status: PlayerStatus)

    fun onVideoSize(width: Int, height: Int)

    fun onScale(scale: ScaleMessage)

    fun onDiagnose(run: Long)
}
