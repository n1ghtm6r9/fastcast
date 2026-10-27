package dev.fastcast.client.network

import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Test

class TheLinkSpeedIsReadFromTheKernelTextTest {

    @Test
    fun `a positive number of megabits is the speed`() {
        assertEquals(1000, LinkSpeed.megabitsFrom("1000\n"))
        assertEquals(100, LinkSpeed.megabitsFrom("100"))
    }

    @Test
    fun `a speed the kernel does not know is no speed`() {
        assertNull(LinkSpeed.megabitsFrom("-1\n"))
        assertNull(LinkSpeed.megabitsFrom("0"))
        assertNull(LinkSpeed.megabitsFrom(""))
        assertNull(LinkSpeed.megabitsFrom("fast"))
        assertNull(LinkSpeed.megabitsFrom(null))
    }
}
