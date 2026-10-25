package dev.fastcast.client.stream

interface StreamListener {

    fun onConnecting()

    fun onConnected()

    fun onConfig(config: ConfigMessage)

    fun onFrame(frame: FrameMessage)

    fun onDisconnected(retryInMs: Long, reason: String)
}
