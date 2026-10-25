package dev.fastcast.client.update

import dev.fastcast.client.testing.anUpdateManifest
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Test

class ANoUpdateResultIsReusedTooTest {

    private var nowMs = 0L
    private val cache = UpdateCheckCache { nowMs }
    private var fetches = 0

    @Test
    fun `none is reused within the window`() {
        cache.resolve(::fetchNothing)
        nowMs += 59_999L

        assertNull(cache.resolve(::fetchNothing))
        assertEquals(1, fetches)
    }

    @Test
    fun `none is fetched again after the window`() {
        cache.resolve(::fetchNothing)
        nowMs += 60_000L

        cache.resolve(::fetchNothing)

        assertEquals(2, fetches)
    }

    @Test
    fun `an update that appears after the window is picked up`() {
        val manifest = anUpdateManifest()
        cache.resolve(::fetchNothing)
        nowMs += 60_000L

        assertEquals(manifest, cache.resolve { manifest })
    }

    @Test
    fun `an update that appears within the window is not seen yet`() {
        cache.resolve(::fetchNothing)
        nowMs += 1_000L

        assertNull(cache.resolve { anUpdateManifest() })
    }

    private fun fetchNothing(): UpdateManifest? {
        fetches++
        return null
    }
}
