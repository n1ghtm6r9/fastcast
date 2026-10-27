package dev.fastcast.client.decoder

import android.media.MediaCodecInfo.CodecCapabilities
import android.media.MediaFormat
import android.os.Build
import dev.fastcast.client.stream.ConfigMessage
import java.nio.ByteBuffer

object VideoFormatFactory {

    private const val MIN_INPUT_SIZE = 2 * 1024 * 1024
    private const val LOW_LATENCY_ENABLED = 1
    private const val CSD_KEY_PREFIX = "csd-"
    private const val QUALCOMM_LOW_LATENCY = "vendor.qti-ext-dec-low-latency.enable"

    fun probe(mime: String, config: ConfigMessage): MediaFormat =
        MediaFormat.createVideoFormat(mime, config.width, config.height).apply {
            setInteger(MediaFormat.KEY_FRAME_RATE, config.fps)
        }

    fun create(
        mime: String,
        config: ConfigMessage,
        parameters: CodecConfiguration,
        choice: DecoderChoice,
        pace: DecoderPace,
        tweak: DecoderTweak,
        audioSessionId: Int? = null,
    ): MediaFormat = MediaFormat.createVideoFormat(mime, config.width, config.height).apply {
        pace.frameRate(config.fps)?.let { setInteger(MediaFormat.KEY_FRAME_RATE, it) }
        parameters.csdBuffers.forEachIndexed { index, csd ->
            setByteBuffer("$CSD_KEY_PREFIX$index", ByteBuffer.wrap(csd))
        }
        setInteger(MediaFormat.KEY_MAX_INPUT_SIZE, maxInputSize(config))
        setInteger(MediaFormat.KEY_PRIORITY, pace.priority)
        val operatingRate = tweak.operatingRate(config.fps) ?: pace.operatingRate(config.fps)
        operatingRate?.let { setInteger(MediaFormat.KEY_OPERATING_RATE, it) }
        if (audioSessionId == null) applySurface(choice, tweak) else applyTunnel(audioSessionId, choice, tweak)
        tweak.vendorValues(config.fps).forEach { (key, value) -> setVendorValue(key, value) }
    }

    private fun maxInputSize(config: ConfigMessage): Int = maxOf(config.width * config.height, MIN_INPUT_SIZE)

    private fun MediaFormat.applySurface(choice: DecoderChoice, tweak: DecoderTweak) {
        if (tweak.usesStandardLowLatency) applyLowLatency(choice)
    }

    private fun MediaFormat.setVendorValue(key: String, value: Number) {
        if (value is Float) setFloat(key, value) else setInteger(key, value.toInt())
    }

    private fun MediaFormat.applyTunnel(audioSessionId: Int, choice: DecoderChoice, tweak: DecoderTweak) {
        setFeatureEnabled(CodecCapabilities.FEATURE_TunneledPlayback, true)
        setInteger(MediaFormat.KEY_AUDIO_SESSION_ID, audioSessionId)
        if (tweak.lowLatency) applyLowLatency(choice)
    }

    private fun MediaFormat.applyLowLatency(choice: DecoderChoice) {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.R && choice.supportsLowLatency) {
            setInteger(MediaFormat.KEY_LOW_LATENCY, LOW_LATENCY_ENABLED)
        }
        if (choice.isQualcomm) setInteger(QUALCOMM_LOW_LATENCY, LOW_LATENCY_ENABLED)
    }
}
