package dev.fastcast.client.testing

import dev.fastcast.client.discovery.DirectExecutor
import dev.fastcast.client.update.UpdateCheckCache
import dev.fastcast.client.update.UpdateListener
import dev.fastcast.client.update.UpdateManifest
import dev.fastcast.client.update.UpdateRunner
import java.io.File
import java.util.concurrent.Executor

fun anUpdateRunner(
    listener: UpdateListener,
    deliver: Executor = DirectExecutor,
    cache: UpdateCheckCache = UpdateCheckCache(),
    fetch: () -> UpdateManifest? = { null },
    download: (UpdateManifest, (Long, Long) -> Unit, () -> Boolean) -> File = { _, _, _ -> error("unexpected download") },
    install: (File, UpdateManifest) -> Unit = { _, _ -> error("unexpected install") },
): UpdateRunner = UpdateRunner(listener, deliver, cache, fetch, download, install)
