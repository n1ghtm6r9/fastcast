package dev.fastcast.client.connect

import dev.fastcast.client.testing.aServer
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Test

class AHeldPolicyChoosesNothingUntilTheHoldEndsTest {

    private var nowMs = 0L
    private val policy = AutoConnectPolicy { nowMs }
    private val server = aServer(name = "Gaming PC")

    @Test
    fun `a remembered server is not chosen the moment a hold starts`() {
        policy.holdFor(HOLD_MS)

        assertNull(policy.choose(listOf(server), "Gaming PC"))
    }

    @Test
    fun `a remembered server is not chosen just short of the hold`() {
        policy.holdFor(HOLD_MS)

        nowMs = HOLD_MS - 1

        assertNull(policy.choose(listOf(server), "Gaming PC"))
    }

    @Test
    fun `a remembered server is chosen the moment the hold ends`() {
        policy.holdFor(HOLD_MS)

        nowMs = HOLD_MS

        assertEquals(server, policy.choose(listOf(server), "Gaming PC"))
    }

    @Test
    fun `a lone server is not chosen during the hold even after the wait`() {
        policy.holdFor(HOLD_MS)
        policy.choose(listOf(server), null)

        nowMs = HOLD_MS - 1

        assertNull(policy.choose(listOf(server), null))
    }

    @Test
    fun `a lone server that waited during the hold is chosen the moment it ends`() {
        policy.holdFor(HOLD_MS)
        policy.choose(listOf(server), null)

        nowMs = HOLD_MS

        assertEquals(server, policy.choose(listOf(server), null))
    }

    @Test
    fun `a lone server that appears just before the hold ends still waits its own time`() {
        policy.holdFor(HOLD_MS)
        nowMs = HOLD_MS - 1
        policy.choose(listOf(server), null)

        nowMs = HOLD_MS
        assertNull(policy.choose(listOf(server), null))

        nowMs = HOLD_MS - 1 + WAIT_MS
        assertEquals(server, policy.choose(listOf(server), null))
    }

    @Test
    fun `the hold counts from the moment it was placed`() {
        nowMs = 10_000L
        policy.holdFor(HOLD_MS)

        nowMs = 10_000L + HOLD_MS - 1
        assertNull(policy.choose(listOf(server), "Gaming PC"))

        nowMs = 10_000L + HOLD_MS
        assertEquals(server, policy.choose(listOf(server), "Gaming PC"))
    }

    @Test
    fun `several servers without a remembered one are never chosen even after the hold`() {
        policy.holdFor(HOLD_MS)
        val servers = listOf(aServer(name = "A", host = "192.168.0.11"), aServer(name = "B", host = "192.168.0.12"))

        nowMs = HOLD_MS * 10

        assertNull(policy.choose(servers, null))
    }

    private companion object {
        const val HOLD_MS = 3_000L
        const val WAIT_MS = 1_500L
    }
}
