package dev.fastcast.client.discovery

import java.net.InetAddress
import java.util.concurrent.Executor

fun aFinderFor(
    responder: FakeDiscoveryResponder,
    listener: ServerFinderListener,
    deliver: Executor = DirectExecutor,
    probeIntervalMs: Long = 40L,
    staleAfterMs: Long = 5_000L,
    targets: () -> List<InetAddress> = { listOf(responder.address) },
): ServerFinder = ServerFinder(
    listener = listener,
    deliver = deliver,
    targets = targets,
    port = responder.port,
    probeIntervalMs = probeIntervalMs,
    staleAfterMs = staleAfterMs,
    bindAddress = responder.address,
)
