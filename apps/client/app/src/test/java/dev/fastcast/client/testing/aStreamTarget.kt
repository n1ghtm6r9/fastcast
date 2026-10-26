package dev.fastcast.client.testing

import dev.fastcast.client.stream.StreamRequest
import dev.fastcast.client.stream.StreamTarget

fun aStreamTarget(port: Int = 3278, request: StreamRequest? = null): StreamTarget =
    StreamTarget("127.0.0.1", port, request)
