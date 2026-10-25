package dev.fastcast.client.update

import dev.fastcast.client.testing.aRejectedVersions
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test

class ARejectedVersionIsRememberedTest {

    private val rejected = aRejectedVersions()

    @Test
    fun `nothing is rejected at first`() {
        assertFalse(rejected.isRejected(101))
    }

    @Test
    fun `a rejected version is reported as rejected`() {
        rejected.reject(101)

        assertTrue(rejected.isRejected(101))
    }

    @Test
    fun `the versions around a rejected one are not rejected`() {
        rejected.reject(101)

        assertEquals(listOf(false, false), listOf(100, 102).map(rejected::isRejected))
    }

    @Test
    fun `several versions can be rejected`() {
        rejected.reject(101)
        rejected.reject(103)

        assertEquals(listOf(true, false, true), listOf(101, 102, 103).map(rejected::isRejected))
    }

    @Test
    fun `rejecting a version twice keeps it rejected`() {
        rejected.reject(101)
        rejected.reject(101)

        assertTrue(rejected.isRejected(101))
    }
}
