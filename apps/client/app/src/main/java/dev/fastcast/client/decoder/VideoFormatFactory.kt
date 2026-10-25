package dev.fastcast.client.decoder

import android.media.MediaFormat
import android.os.Build
import dev.fastcast.client.stream.ConfigMessage
import java.nio.ByteBuffer

object VideoFormatFactory {

    private const val MIN_INPUT_SIZE = 2 * 1024 * 1024
    private const val REALTIME_PRIORITY = 0
    private const val LOW_LATENCY_ENABLED = 1
    private const val FIRST_CSD = "csd-0"
    private const val SECOND_CSD = "csd-1"
    private const val QUALCOMM_LOW_LATENCY = "vendor.qti-ext-dec-low-latency.enable"

    fun probe(mime: String, config: ConfigMessage): MediaFormat =
        MediaFormat.createVideoFormat(mime, config.width, config.height).apply {
            setInteger(MediaFormat.KEY_FRAME_RATE, config.fps)
        }

    fun create(
        mime: String,
        config: ConfigMessage,
        parameters: AvcConfigurationRecord,
        choice: DecoderChoice,
    ): MediaFormat = probe(mime, config).apply {
        setByteBuffer(FIRST_CSD, ByteBuffer.wrap(parameters.csd0))
        setByteBuffer(SECOND_CSD, ByteBuffer.wrap(parameters.csd1))
        setInteger(MediaFormat.KEY_MAX_INPUT_SIZE, maxInputSize(config))
        setInteger(MediaFormat.KEY_PRIORITY, REALTIME_PRIORITY)
        setInteger(MediaFormat.KEY_OPERATING_RATE, config.fps)
        applyLowLatency(choice)
    }

    private fun maxInputSize(config: ConfigMessage): Int = maxOf(config.width * config.height, MIN_INPUT_SIZE)

    private fun MediaFormat.applyLowLatency(choice: DecoderChoice) {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.R && choice.supportsLowLatency) {
            setInteger(MediaFormat.KEY_LOW_LATENCY, LOW_LATENCY_ENABLED)
        }
        if (choice.isQualcomm) setInteger(QUALCOMM_LOW_LATENCY, LOW_LATENCY_ENABLED)
    }
}
