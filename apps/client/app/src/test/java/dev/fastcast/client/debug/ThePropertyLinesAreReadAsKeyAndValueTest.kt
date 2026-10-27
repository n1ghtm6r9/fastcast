package dev.fastcast.client.debug

import org.junit.Assert.assertEquals
import org.junit.Test

class ThePropertyLinesAreReadAsKeyAndValueTest {

    @Test
    fun `an interesting property becomes key=value`() {
        assertEquals(listOf("ro.vendor.panel.refresh=120"), PropertyLines.matching("[ro.vendor.panel.refresh]: [120]"))
    }

    @Test
    fun `uninteresting properties are left out`() {
        assertEquals(emptyList<String>(), PropertyLines.matching("[ro.build.version.sdk]: [34]"))
    }

    @Test
    fun `lines that are not properties are skipped`() {
        val output = "garbage\n\n[vendor.hwc.fps]: [60]\nwarning: refresh\n"

        assertEquals(listOf("vendor.hwc.fps=60"), PropertyLines.matching(output))
    }

    @Test
    fun `a value may hold brackets or be empty`() {
        val output = "[vendor.display.modes]: [[60, 120]]\n[persist.vendor.memc]: []"

        assertEquals(listOf("vendor.display.modes=[60, 120]", "persist.vendor.memc="), PropertyLines.matching(output))
    }

    @Test
    fun `windows line endings are read too`() {
        assertEquals(listOf("vendor.hwc.fps=60"), PropertyLines.matching("[vendor.hwc.fps]: [60]\r\n"))
    }
}
