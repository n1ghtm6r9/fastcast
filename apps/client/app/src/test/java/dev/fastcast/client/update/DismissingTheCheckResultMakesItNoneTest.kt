package dev.fastcast.client.update

import dev.fastcast.client.testing.anUpdateManifest
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Test

class DismissingTheCheckResultMakesItNoneTest {

    private var nowMs = 0L
    private val cache = UpdateCheckCache { nowMs }
    private val manifest = anUpdateManifest()
    private var fetches = 0

    @Test
    fun `a dismissed manifest is reported as none without a fetch`() {
        cache.resolve(::fetchManifest)

        cache.dismiss()

        assertNull(cache.resolve(::fetchManifest))
        assertEquals(1, fetches)
    }

    @Test
    fun `dismissing restarts the window`() {
        cache.resolve(::fetchManifest)
        nowMs += 30_000L
        cache.dismiss()

        nowMs += 59_999L
        assertNull(cache.resolve(::fetchManifest))
        assertEquals(1, fetches)

        nowMs += 1L
        assertEquals(manifest, cache.resolve(::fetchManifest))
        assertEquals(2, fetches)
    }

    @Test
    fun `dismissing before any fetch makes the next request reuse none`() {
        cache.dismiss()

        assertNull(cache.resolve(::fetchManifest))
        assertEquals(0, fetches)
    }

    @Test
    fun `dismissing none changes nothing visible`() {
        cache.resolve { null }

        cache.dismiss()

        assertNull(cache.resolve(::fetchManifest))
        assertEquals(0, fetches)
    }

    private fun fetchManifest(): UpdateManifest? {
        fetches++
        return manifest
    }
}
