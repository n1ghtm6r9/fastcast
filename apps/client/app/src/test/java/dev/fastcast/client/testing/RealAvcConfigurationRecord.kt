package dev.fastcast.client.testing

object RealAvcConfigurationRecord {

    val BYTES: ByteArray = hexBytes(
        "0164002affe1001a6764002aacd9402800b5b011000003000100000300780f18319601000668eae3cb22c0fdf8f800",
    )

    val SEQUENCE_PARAMETER_SET: ByteArray = hexBytes(
        "6764002aacd9402800b5b011000003000100000300780f183196",
    )

    val PICTURE_PARAMETER_SET: ByteArray = hexBytes("68eae3cb22c0")
}
