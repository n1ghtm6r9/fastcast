package dev.fastcast.client.testing

fun aServerReplyOfSize(size: Int, name: String = "Gaming PC"): ByteArray {
    val head = aServerReply(name).removeSuffix("}") + ",\"padding\":\""
    val tail = "\"}"
    val padding = "x".repeat(size - head.length - tail.length)
    return (head + padding + tail).toByteArray(Charsets.UTF_8)
}
