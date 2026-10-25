package dev.fastcast.client.update

import dev.fastcast.client.discovery.DirectExecutor
import java.io.IOException
import java.util.concurrent.CountDownLatch
import java.util.concurrent.LinkedBlockingQueue
import java.util.concurrent.TimeUnit
import org.junit.After
import org.junit.Assert.assertNull
import org.junit.Assert.assertTrue
import org.junit.Test

class ALookupWithoutANewerVersionStaysSilentTest {

    private val callbacks = LinkedBlockingQueue<String>()
    private val asked = CountDownLatch(1)
    private var lookup: UpdateLookup? = null

    @After
    fun tearDown() {
        lookup?.stop()
    }

    @Test
    fun `no newer version leaves the lookup silent`() {
        start(fetch = {
            asked.countDown()
            null
        })

        assertTrue(asked.await(TIMEOUT_SECONDS, TimeUnit.SECONDS))
        assertNull(callbacks.poll(300L, TimeUnit.MILLISECONDS))
    }

    @Test
    fun `a failing check leaves the lookup silent`() {
        start(fetch = {
            asked.countDown()
            throw IOException("offline")
        })

        assertTrue(asked.await(TIMEOUT_SECONDS, TimeUnit.SECONDS))
        assertNull(callbacks.poll(300L, TimeUnit.MILLISECONDS))
    }

    @Test
    fun `a failing check is not remembered by the shared cache`() {
        val cache = UpdateCheckCache { 0L }
        start(cache = cache, fetch = {
            asked.countDown()
            throw IOException("offline")
        })
        asked.await(TIMEOUT_SECONDS, TimeUnit.SECONDS)
        callbacks.poll(300L, TimeUnit.MILLISECONDS)

        var fetchedAgain = false
        cache.resolve {
            fetchedAgain = true
            null
        }

        assertTrue(fetchedAgain)
    }

    private fun start(cache: UpdateCheckCache = UpdateCheckCache { 0L }, fetch: () -> UpdateManifest?) {
        lookup = UpdateLookup(
            cache = cache,
            fetch = fetch,
            deliver = DirectExecutor,
            onUpdateAvailable = { callbacks.add("update available") },
        ).also(UpdateLookup::start)
    }

    private companion object {
        const val TIMEOUT_SECONDS = 5L
    }
}
