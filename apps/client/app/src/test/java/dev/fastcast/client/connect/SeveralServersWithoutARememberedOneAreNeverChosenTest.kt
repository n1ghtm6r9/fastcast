package dev.fastcast.client.connect

import dev.fastcast.client.testing.aServer
import org.junit.Assert.assertNull
import org.junit.Test

class SeveralServersWithoutARememberedOneAreNeverChosenTest {

    private var nowMs = 0L
    private val policy = AutoConnectPolicy { nowMs }
    private val laptop = aServer(name = "Laptop", host = "192.168.0.11")
    private val gamingPc = aServer(name = "Gaming PC", host = "192.168.0.12")

    @Test
    fun `two servers that appear together are not chosen however long we wait`() {
        val servers = listOf(laptop, gamingPc)

        assertNull(policy.choose(servers, null))
        nowMs = WAIT_MS
        assertNull(policy.choose(servers, null))
        nowMs = 10 * WAIT_MS
        assertNull(policy.choose(servers, null))
    }

    @Test
    fun `a second server arriving within the wait cancels the automatic choice`() {
        policy.choose(listOf(laptop), null)

        nowMs = WAIT_MS - 1
        policy.choose(listOf(laptop, gamingPc), null)

        nowMs = WAIT_MS
        assertNull(policy.choose(listOf(laptop, gamingPc), null))
    }

    @Test
    fun `a second server arriving after the wait is never chosen either`() {
        policy.choose(listOf(laptop), null)

        nowMs = 10 * WAIT_MS

        assertNull(policy.choose(listOf(laptop, gamingPc), null))
    }

    @Test
    fun `several servers none of which is remembered are not chosen`() {
        val servers = listOf(laptop, gamingPc)

        assertNull(policy.choose(servers, "Office"))
        nowMs = 10 * WAIT_MS
        assertNull(policy.choose(servers, "Office"))
    }

    private companion object {
        const val WAIT_MS = 1_500L
    }
}
