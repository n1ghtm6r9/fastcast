package dev.fastcast.client.decoder

interface DecoderEvents {

    fun onConfigured()

    fun onVideoSize(width: Int, height: Int)

    fun onFirstFrame()

    fun onKeyframeNeeded()

    fun onFailure(message: String)
}
