package dev.fastcast.client.connect

import dev.fastcast.client.testing.aServer
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Test

class ALoneServerIsChosenOnlyAfterTheWaitTest {

    private var nowMs = 0L
    private val policy = AutoConnectPolicy { nowMs }
    private val server = aServer(name = "Gaming PC")

    @Test
    fun `a lone server is not chosen the moment it appears`() {
        assertNull(policy.choose(listOf(server), null))
    }

    @Test
    fun `a lone server is not chosen just short of the wait`() {
        policy.choose(listOf(server), null)

        nowMs = WAIT_MS - 1

        assertNull(policy.choose(listOf(server), null))
    }

    @Test
    fun `a lone server is chosen once the wait has passed`() {
        policy.choose(listOf(server), null)

        nowMs = WAIT_MS

        assertEquals(server, policy.choose(listOf(server), null))
    }

    @Test
    fun `a lone server stays chosen when asked again later`() {
        policy.choose(listOf(server), null)

        nowMs = 10 * WAIT_MS

        assertEquals(server, policy.choose(listOf(server), null))
    }

    @Test
    fun `the wait counts from the first reply and not from the last question`() {
        nowMs = 1_000L
        policy.choose(listOf(server), null)

        nowMs = 1_000L + WAIT_MS - 1
        assertNull(policy.choose(listOf(server), null))

        nowMs = 1_000L + WAIT_MS
        assertEquals(server, policy.choose(listOf(server), null))
    }

    @Test
    fun `asking often in between does not stretch the wait`() {
        policy.choose(listOf(server), null)
        nowMs = 500L
        policy.choose(listOf(server), null)
        nowMs = 1_000L
        policy.choose(listOf(server), null)

        nowMs = WAIT_MS

        assertEquals(server, policy.choose(listOf(server), null))
    }

    @Test
    fun `a lone server is still chosen when the remembered name is another one`() {
        policy.choose(listOf(server), "Office")

        nowMs = WAIT_MS

        assertEquals(server, policy.choose(listOf(server), "Office"))
    }

    @Test
    fun `a renamed lone server keeps the wait running`() {
        policy.choose(listOf(aServer(name = "Old name")), null)
        nowMs = 1_000L
        policy.choose(listOf(aServer(name = "New name")), null)

        nowMs = WAIT_MS

        assertEquals(aServer(name = "New name"), policy.choose(listOf(aServer(name = "New name")), null))
    }

    private companion object {
        const val WAIT_MS = 1_500L
    }
}
