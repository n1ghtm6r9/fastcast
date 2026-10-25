package dev.fastcast.client.testing

import dev.fastcast.client.stream.ConfigMessage

fun aConfigMessage(width: Int = 1280, height: Int = 720): ConfigMessage =
    ConfigMessage("avc1.64002a", width, height, 60, RealAvcConfigurationRecord.BYTES)
