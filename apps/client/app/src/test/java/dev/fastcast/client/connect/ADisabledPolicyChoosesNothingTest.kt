package dev.fastcast.client.connect

import dev.fastcast.client.testing.aServer
import org.junit.Assert.assertFalse
import org.junit.Assert.assertNull
import org.junit.Assert.assertTrue
import org.junit.Test

class ADisabledPolicyChoosesNothingTest {

    private var nowMs = 0L
    private val policy = AutoConnectPolicy { nowMs }
    private val server = aServer(name = "Gaming PC")

    @Test
    fun `a new policy is enabled`() {
        assertTrue(policy.isEnabled)
    }

    @Test
    fun `disabling turns the policy off`() {
        policy.disable()

        assertFalse(policy.isEnabled)
    }

    @Test
    fun `a disabled policy ignores a remembered server`() {
        policy.disable()

        assertNull(policy.choose(listOf(server), "Gaming PC"))
    }

    @Test
    fun `a disabled policy ignores a lone server even after the wait`() {
        policy.choose(listOf(server), null)
        policy.disable()

        nowMs = 10 * WAIT_MS

        assertNull(policy.choose(listOf(server), null))
    }

    @Test
    fun `a policy disabled before it ever saw a server stays silent`() {
        policy.disable()

        nowMs = 10 * WAIT_MS

        assertNull(policy.choose(listOf(server), null))
        assertNull(policy.choose(listOf(server), "Gaming PC"))
    }

    @Test
    fun `an empty list does not turn a disabled policy back on`() {
        policy.disable()

        policy.choose(emptyList(), null)

        assertFalse(policy.isEnabled)
        assertNull(policy.choose(listOf(server), "Gaming PC"))
    }

    @Test
    fun `disabling twice changes nothing`() {
        policy.disable()
        policy.disable()

        assertFalse(policy.isEnabled)
    }

    private companion object {
        const val WAIT_MS = 1_500L
    }
}
