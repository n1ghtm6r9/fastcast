package dev.fastcast.client.update

import dev.fastcast.client.testing.anUpdateManifest
import org.junit.Assert.assertEquals
import org.junit.Test

class ACheckResultIsReusedForSixtySecondsTest {

    private var nowMs = 5_000L
    private val cache = UpdateCheckCache { nowMs }
    private val manifest = anUpdateManifest(versionCode = 101)
    private var fetches = 0

    @Test
    fun `the first request fetches`() {
        assertEquals(manifest, cache.resolve(::fetchManifest))
        assertEquals(1, fetches)
    }

    @Test
    fun `a request right after the first reuses the result`() {
        cache.resolve(::fetchManifest)

        assertEquals(manifest, cache.resolve(::fetchManifest))
        assertEquals(1, fetches)
    }

    @Test
    fun `a request just short of sixty seconds reuses the result`() {
        cache.resolve(::fetchManifest)
        nowMs += 59_999L

        assertEquals(manifest, cache.resolve(::fetchManifest))
        assertEquals(1, fetches)
    }

    @Test
    fun `a request after sixty seconds fetches again`() {
        cache.resolve(::fetchManifest)
        nowMs += 60_000L

        cache.resolve(::fetchManifest)

        assertEquals(2, fetches)
    }

    @Test
    fun `reusing the result does not extend the window`() {
        cache.resolve(::fetchManifest)
        nowMs += 30_000L
        cache.resolve(::fetchManifest)
        nowMs += 30_000L

        cache.resolve(::fetchManifest)

        assertEquals(2, fetches)
    }

    @Test
    fun `every fetch starts a new window`() {
        cache.resolve(::fetchManifest)
        nowMs += 60_000L
        cache.resolve(::fetchManifest)
        nowMs += 59_999L
        cache.resolve(::fetchManifest)
        assertEquals(2, fetches)

        nowMs += 1L
        cache.resolve(::fetchManifest)

        assertEquals(3, fetches)
    }

    @Test
    fun `a newer answer replaces the stale one after the window`() {
        val newer = anUpdateManifest(versionCode = 102)
        cache.resolve(::fetchManifest)
        nowMs += 60_000L

        assertEquals(newer, cache.resolve { newer })
        assertEquals(newer, cache.resolve(::fetchManifest))
    }

    private fun fetchManifest(): UpdateManifest? {
        fetches++
        return manifest
    }
}
