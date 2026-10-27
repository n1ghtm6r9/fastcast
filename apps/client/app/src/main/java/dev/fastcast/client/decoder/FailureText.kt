package dev.fastcast.client.decoder

import android.media.MediaCodec

object FailureText {

    fun of(error: Throwable): String {
        val told = error.message?.takeIf { it.isNotBlank() } ?: error.javaClass.simpleName
        val details = (error as? MediaCodec.CodecException)?.diagnosticInfo
        return if (details.isNullOrBlank()) told else "$told ($details)"
    }
}
