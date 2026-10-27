package dev.fastcast.client.debug

import org.junit.Assert.assertEquals
import org.junit.Test

class ThePropertyChangesListWhatDifferedTest {

    @Test
    fun `a changed value is listed with the old and the new value`() {
        assertEquals(
            listOf("vendor.panel.fps: 60 -> 120"),
            PropertyChanges.between(mapOf("vendor.panel.fps" to "60"), mapOf("vendor.panel.fps" to "120")),
        )
    }

    @Test
    fun `an added or removed property shows a dash for the missing side`() {
        assertEquals(
            listOf("vendor.memc: on -> -", "vendor.vdp.fps: - -> 120"),
            PropertyChanges.between(mapOf("vendor.memc" to "on"), mapOf("vendor.vdp.fps" to "120")),
        )
    }

    @Test
    fun `unchanged properties are left out and the rest is sorted by name`() {
        val before = mapOf("b" to "1", "same" to "x", "a" to "1")
        val after = mapOf("same" to "x", "b" to "2", "a" to "2")

        assertEquals(listOf("a: 1 -> 2", "b: 1 -> 2"), PropertyChanges.between(before, after))
    }

    @Test
    fun `nothing changed is said once`() {
        assertEquals(listOf("props unchanged"), PropertyChanges.between(mapOf("a" to "1"), mapOf("a" to "1")))
    }

    @Test
    fun `at most forty changes are listed and the rest is counted`() {
        val before = (1..45).associate { "key%02d".format(it) to "old" }
        val after = before.mapValues { "new" }

        val changes = PropertyChanges.between(before, after)

        assertEquals(41, changes.size)
        assertEquals("key40: old -> new", changes[39])
        assertEquals("and 5 more changed props", changes.last())
    }
}
