package dev.fastcast.client.testing

fun aServerReply(name: String = "Gaming PC", port: Int = 3278): String =
    """{"type":"server","version":1,"name":"$name","port":$port}"""
