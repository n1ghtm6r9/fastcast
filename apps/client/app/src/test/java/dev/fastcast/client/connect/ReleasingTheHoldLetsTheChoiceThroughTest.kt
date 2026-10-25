package dev.fastcast.client.connect

import dev.fastcast.client.testing.aServer
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Test

class ReleasingTheHoldLetsTheChoiceThroughTest {

    private var nowMs = 0L
    private val policy = AutoConnectPolicy { nowMs }
    private val server = aServer(name = "Gaming PC")

    @Test
    fun `a hold until released never runs out`() {
        policy.holdUntilReleased()

        nowMs = Long.MAX_VALUE / 2

        assertNull(policy.choose(listOf(server), "Gaming PC"))
    }

    @Test
    fun `releasing lets a remembered server through at once`() {
        policy.holdUntilReleased()
        assertNull(policy.choose(listOf(server), "Gaming PC"))

        policy.release()

        assertEquals(server, policy.choose(listOf(server), "Gaming PC"))
    }

    @Test
    fun `releasing ends a timed hold early`() {
        policy.holdFor(3_000L)
        nowMs = 100L

        policy.release()

        assertEquals(server, policy.choose(listOf(server), "Gaming PC"))
    }

    @Test
    fun `releasing a lone server that already waited lets it through`() {
        policy.holdUntilReleased()
        policy.choose(listOf(server), null)
        nowMs = 5_000L
        assertNull(policy.choose(listOf(server), null))

        policy.release()

        assertEquals(server, policy.choose(listOf(server), null))
    }

    @Test
    fun `releasing without a hold changes nothing`() {
        policy.release()

        assertEquals(server, policy.choose(listOf(server), "Gaming PC"))
    }

    @Test
    fun `a hold placed again after a release holds again`() {
        policy.holdUntilReleased()
        policy.release()

        policy.holdUntilReleased()

        assertNull(policy.choose(listOf(server), "Gaming PC"))
    }

    @Test
    fun `a timed hold replaces a hold until released`() {
        policy.holdUntilReleased()

        policy.holdFor(3_000L)
        nowMs = 3_000L

        assertEquals(server, policy.choose(listOf(server), "Gaming PC"))
    }

    @Test
    fun `a disabled policy stays silent after the release`() {
        policy.holdUntilReleased()
        policy.disable()

        policy.release()

        assertNull(policy.choose(listOf(server), "Gaming PC"))
    }

    @Test
    fun `a hold does not re-enable a disabled policy`() {
        policy.disable()

        policy.holdFor(1L)
        nowMs = 10L

        assertNull(policy.choose(listOf(server), "Gaming PC"))
    }
}
