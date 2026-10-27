package dev.fastcast.client.decoder

import android.annotation.TargetApi
import android.media.MediaCodec
import android.media.MediaFormat
import android.os.Build

@TargetApi(Build.VERSION_CODES.S)
object CodecParameterSnapshot {

    private const val UNREAD = "?"
    private const val BYTES = "bytes"
    private const val NOTHING = "null"
    private val TYPE_NAMES = mapOf(
        MediaFormat.TYPE_INTEGER to "int",
        MediaFormat.TYPE_LONG to "long",
        MediaFormat.TYPE_FLOAT to "float",
        MediaFormat.TYPE_STRING to "string",
        MediaFormat.TYPE_BYTE_BUFFER to BYTES,
    )

    fun subscribe(codec: MediaCodec): List<String> {
        val names = WatchedVendorParameters.among(codec.supportedVendorParameters)
        if (names.isNotEmpty()) codec.subscribeToVendorParameters(names)
        return names
    }

    fun read(codec: MediaCodec, vendorNames: List<String>): CodecParameters {
        val format = codec.outputFormat
        val vendor = vendorNames.associateWith { vendorValueOf(codec, format, it) }
        val rest = format.keys.filterNot(vendor::containsKey).associateWith { valueOf(format, it) }
        return CodecParameters(vendor, rest)
    }

    private fun vendorValueOf(codec: MediaCodec, format: MediaFormat, name: String): String =
        if (format.containsKey(name)) valueOf(format, name) else UNREAD + typeNameOf(codec.getParameterDescriptor(name)?.type)

    private fun valueOf(format: MediaFormat, key: String): String = when (format.getValueTypeForKey(key)) {
        MediaFormat.TYPE_INTEGER -> format.getInteger(key).toString()
        MediaFormat.TYPE_LONG -> format.getLong(key).toString()
        MediaFormat.TYPE_FLOAT -> format.getFloat(key).toString()
        MediaFormat.TYPE_STRING -> format.getString(key).orEmpty()
        MediaFormat.TYPE_BYTE_BUFFER -> BYTES
        else -> NOTHING
    }

    private fun typeNameOf(type: Int?): String = TYPE_NAMES[type] ?: NOTHING
}
