package dev.fastcast.client.update

import dev.fastcast.client.testing.anUpdateManifest
import java.util.concurrent.Callable
import java.util.concurrent.CountDownLatch
import java.util.concurrent.Executors
import java.util.concurrent.TimeUnit
import java.util.concurrent.atomic.AtomicInteger
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Assert.assertTrue
import org.junit.Test

class ConcurrentChecksShareOneFetchTest {

    private val cache = UpdateCheckCache { 0L }
    private val manifest = anUpdateManifest()
    private val fetches = AtomicInteger()
    private val pool = Executors.newFixedThreadPool(CALLERS)

    @After
    fun tearDown() {
        pool.shutdownNow()
    }

    @Test
    fun `simultaneous requests wait for one fetch and share its manifest`() {
        val fetching = CountDownLatch(1)
        val release = CountDownLatch(1)
        val fetch = {
            fetches.incrementAndGet()
            fetching.countDown()
            release.await(TIMEOUT_SECONDS, TimeUnit.SECONDS)
            manifest
        }

        val results = List(CALLERS) { pool.submit(Callable { cache.resolve(fetch) }) }
        assertTrue(fetching.await(TIMEOUT_SECONDS, TimeUnit.SECONDS))
        release.countDown()

        assertEquals(List(CALLERS) { manifest }, results.map { it.get(TIMEOUT_SECONDS, TimeUnit.SECONDS) })
        assertEquals(1, fetches.get())
    }

    @Test
    fun `simultaneous requests share a missing update too`() {
        val fetching = CountDownLatch(1)
        val release = CountDownLatch(1)
        val fetch = {
            fetches.incrementAndGet()
            fetching.countDown()
            release.await(TIMEOUT_SECONDS, TimeUnit.SECONDS)
            null
        }

        val results = List(CALLERS) { pool.submit(Callable { cache.resolve(fetch) }) }
        assertTrue(fetching.await(TIMEOUT_SECONDS, TimeUnit.SECONDS))
        release.countDown()

        results.forEach { assertNull(it.get(TIMEOUT_SECONDS, TimeUnit.SECONDS)) }
        assertEquals(1, fetches.get())
    }

    @Test
    fun `dismissing never waits for a fetch in progress`() {
        val fetching = CountDownLatch(1)
        val release = CountDownLatch(1)
        val running = pool.submit(Callable {
            cache.resolve {
                fetching.countDown()
                release.await(TIMEOUT_SECONDS, TimeUnit.SECONDS)
                manifest
            }
        })
        assertTrue(fetching.await(TIMEOUT_SECONDS, TimeUnit.SECONDS))

        val dismissal = pool.submit(Runnable { cache.dismiss() })

        dismissal.get(TIMEOUT_SECONDS, TimeUnit.SECONDS)
        release.countDown()
        running.get(TIMEOUT_SECONDS, TimeUnit.SECONDS)
    }

    private companion object {
        const val CALLERS = 4
        const val TIMEOUT_SECONDS = 5L
    }
}
