package dev.fastcast.client.decoder

interface DecoderEvents {

    fun onVideoSize(width: Int, height: Int)

    fun onFirstFrame()

    fun onFailure(message: String)
}
