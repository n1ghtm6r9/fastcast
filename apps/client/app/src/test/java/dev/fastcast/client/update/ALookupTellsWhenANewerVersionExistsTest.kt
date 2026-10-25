package dev.fastcast.client.update

import dev.fastcast.client.discovery.DirectExecutor
import dev.fastcast.client.discovery.QueueingExecutor
import dev.fastcast.client.testing.anUpdateManifest
import java.util.concurrent.Executor
import java.util.concurrent.LinkedBlockingQueue
import java.util.concurrent.TimeUnit
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Test

class ALookupTellsWhenANewerVersionExistsTest {

    private val callbacks = LinkedBlockingQueue<String>()
    private var lookup: UpdateLookup? = null

    @After
    fun tearDown() {
        lookup?.stop()
    }

    @Test
    fun `a newer version triggers the callback`() {
        lookupWith(DirectExecutor).start()

        assertEquals(CALLBACK, callbacks.poll(TIMEOUT_SECONDS, TimeUnit.SECONDS))
    }

    @Test
    fun `the callback runs once for a single start`() {
        lookupWith(DirectExecutor).start()
        callbacks.poll(TIMEOUT_SECONDS, TimeUnit.SECONDS)

        assertNull(callbacks.poll(200L, TimeUnit.MILLISECONDS))
    }

    @Test
    fun `the callback waits for the executor`() {
        val executor = QueueingExecutor()
        lookupWith(executor).start()

        val queued = executor.next()

        assertNull(callbacks.poll(200L, TimeUnit.MILLISECONDS))
        queued.run()
        assertEquals(CALLBACK, callbacks.poll(TIMEOUT_SECONDS, TimeUnit.SECONDS))
    }

    @Test
    fun `starting twice asks once`() {
        var fetches = 0
        val twice = UpdateLookup(
            cache = UpdateCheckCache { 0L },
            fetch = {
                fetches++
                anUpdateManifest()
            },
            deliver = DirectExecutor,
            onUpdateAvailable = { callbacks.add(CALLBACK) },
        ).also { lookup = it }

        twice.start()
        twice.start()
        callbacks.poll(TIMEOUT_SECONDS, TimeUnit.SECONDS)

        assertNull(callbacks.poll(200L, TimeUnit.MILLISECONDS))
        assertEquals(1, fetches)
    }

    private fun lookupWith(deliver: Executor): UpdateLookup =
        UpdateLookup(
            cache = UpdateCheckCache { 0L },
            fetch = { anUpdateManifest() },
            deliver = deliver,
            onUpdateAvailable = { callbacks.add(CALLBACK) },
        ).also { lookup = it }

    private companion object {
        const val CALLBACK = "update available"
        const val TIMEOUT_SECONDS = 5L
    }
}
