package dev.fastcast.client.testing

import java.math.BigInteger
import java.security.MessageDigest

fun sha256Hex(bytes: ByteArray): String =
    BigInteger(1, MessageDigest.getInstance("SHA-256").digest(bytes)).toString(16).padStart(64, '0')
