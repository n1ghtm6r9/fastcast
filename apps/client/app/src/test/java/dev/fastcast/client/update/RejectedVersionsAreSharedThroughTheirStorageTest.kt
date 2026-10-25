package dev.fastcast.client.update

import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test

class RejectedVersionsAreSharedThroughTheirStorageTest {

    private var saved = emptySet<Int>()

    @Test
    fun `a version rejected earlier is still rejected by a new instance`() {
        rejectedVersions().reject(101)

        assertTrue(rejectedVersions().isRejected(101))
    }

    @Test
    fun `a version rejected elsewhere is seen by an instance created before`() {
        val early = rejectedVersions()

        rejectedVersions().reject(101)

        assertTrue(early.isRejected(101))
    }

    @Test
    fun `rejecting a version keeps the versions saved before`() {
        saved = setOf(100)

        rejectedVersions().reject(101)

        assertEquals(setOf(100, 101), saved)
    }

    private fun rejectedVersions() = RejectedVersions(read = { saved }, write = { saved = it })
}
