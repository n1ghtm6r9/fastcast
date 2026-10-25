package dev.fastcast.client.decoder

fun runQuietly(block: () -> Unit) {
    try {
        block()
    } catch (ignored: Exception) {
        return
    }
}
