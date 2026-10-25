package dev.fastcast.client.connect

import dev.fastcast.client.testing.aServer
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Test

class AnEmptyListChoosesNothingAndRestartsTheWaitTest {

    private var nowMs = 0L
    private val policy = AutoConnectPolicy { nowMs }
    private val server = aServer(name = "Gaming PC")

    @Test
    fun `an empty list chooses nothing`() {
        assertNull(policy.choose(emptyList(), null))
    }

    @Test
    fun `an empty list chooses nothing even with a remembered name`() {
        assertNull(policy.choose(emptyList(), "Gaming PC"))
    }

    @Test
    fun `an empty list keeps the policy enabled`() {
        policy.choose(emptyList(), "Gaming PC")

        assertEquals(server, policy.choose(listOf(server), "Gaming PC"))
    }

    @Test
    fun `an empty list in between makes the wait start over`() {
        policy.choose(listOf(server), null)
        nowMs = 1_000L
        policy.choose(emptyList(), null)

        nowMs = 1_200L
        assertNull(policy.choose(listOf(server), null))

        nowMs = 1_200L + WAIT_MS - 1
        assertNull(policy.choose(listOf(server), null))

        nowMs = 1_200L + WAIT_MS
        assertEquals(server, policy.choose(listOf(server), null))
    }

    @Test
    fun `the time spent with an empty list does not count towards the wait`() {
        policy.choose(emptyList(), null)

        nowMs = 10 * WAIT_MS

        assertNull(policy.choose(listOf(server), null))
    }

    private companion object {
        const val WAIT_MS = 1_500L
    }
}
