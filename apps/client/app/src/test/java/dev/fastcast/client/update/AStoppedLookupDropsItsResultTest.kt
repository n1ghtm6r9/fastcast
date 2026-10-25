package dev.fastcast.client.update

import dev.fastcast.client.discovery.DirectExecutor
import dev.fastcast.client.discovery.QueueingExecutor
import dev.fastcast.client.testing.anUpdateManifest
import java.util.concurrent.CountDownLatch
import java.util.concurrent.Executor
import java.util.concurrent.LinkedBlockingQueue
import java.util.concurrent.TimeUnit
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Assert.assertTrue
import org.junit.Test

class AStoppedLookupDropsItsResultTest {

    private val callbacks = LinkedBlockingQueue<String>()
    private var lookup: UpdateLookup? = null

    @After
    fun tearDown() {
        lookup?.stop()
    }

    @Test
    fun `a lookup stopped during the check never calls back`() {
        val checking = CountDownLatch(1)
        val release = CountDownLatch(1)
        val stoppable = create(DirectExecutor) {
            checking.countDown()
            release.await(TIMEOUT_SECONDS, TimeUnit.SECONDS)
            anUpdateManifest()
        }
        stoppable.start()
        assertTrue(checking.await(TIMEOUT_SECONDS, TimeUnit.SECONDS))

        stoppable.stop()
        release.countDown()

        assertNull(callbacks.poll(300L, TimeUnit.MILLISECONDS))
    }

    @Test
    fun `a result queued before the stop is dropped`() {
        val executor = QueueingExecutor()
        val stoppable = create(executor) { anUpdateManifest() }
        stoppable.start()
        val queued = executor.next()

        stoppable.stop()
        queued.run()

        assertNull(callbacks.poll(300L, TimeUnit.MILLISECONDS))
    }

    @Test
    fun `a lookup that is started again after a stop calls back`() {
        val executor = QueueingExecutor()
        val restartable = create(executor) { anUpdateManifest() }
        restartable.start()
        executor.next()
        restartable.stop()

        restartable.start()
        executor.next().run()

        assertEquals(CALLBACK, callbacks.poll(TIMEOUT_SECONDS, TimeUnit.SECONDS))
    }

    private fun create(deliver: Executor, fetch: () -> UpdateManifest?): UpdateLookup =
        UpdateLookup(
            cache = UpdateCheckCache { 0L },
            fetch = fetch,
            deliver = deliver,
            onUpdateAvailable = { callbacks.add(CALLBACK) },
        ).also { lookup = it }

    private companion object {
        const val CALLBACK = "update available"
        const val TIMEOUT_SECONDS = 5L
    }
}
