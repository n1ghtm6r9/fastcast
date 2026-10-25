package dev.fastcast.client.testing

fun anApkPayload(size: Int): ByteArray = ByteArray(size) { (it * 31 + 7).toByte() }
