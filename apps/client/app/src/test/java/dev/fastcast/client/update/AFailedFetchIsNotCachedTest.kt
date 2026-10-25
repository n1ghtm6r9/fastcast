package dev.fastcast.client.update

import dev.fastcast.client.testing.anUpdateManifest
import org.junit.Assert.assertEquals
import org.junit.Assert.assertThrows
import org.junit.Test

class AFailedFetchIsNotCachedTest {

    private val cache = UpdateCheckCache { 0L }
    private val manifest = anUpdateManifest()
    private var fetches = 0

    @Test
    fun `a failing fetch passes its exception on`() {
        val failure = assertThrows(IllegalStateException::class.java) { cache.resolve(::failToFetch) }

        assertEquals("offline", failure.message)
    }

    @Test
    fun `the next request fetches again after a failure`() {
        runCatching { cache.resolve(::failToFetch) }

        assertEquals(manifest, cache.resolve(::fetchManifest))
        assertEquals(2, fetches)
    }

    @Test
    fun `a failure does not disturb the cached result`() {
        cache.resolve(::fetchManifest)

        assertEquals(manifest, cache.resolve(::failToFetch))
        assertEquals(1, fetches)
    }

    private fun fetchManifest(): UpdateManifest? {
        fetches++
        return manifest
    }

    private fun failToFetch(): UpdateManifest? {
        fetches++
        throw IllegalStateException("offline")
    }
}
