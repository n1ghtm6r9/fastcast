package dev.fastcast.client.testing

object RealAv1ConfigurationRecord {

    val SEQUENCE_HEADER_OBU: ByteArray = hexBytes("0a0f00000072efbfe1bc0219d020201040")

    val BYTES: ByteArray = hexBytes("810e4c00") + SEQUENCE_HEADER_OBU
}
