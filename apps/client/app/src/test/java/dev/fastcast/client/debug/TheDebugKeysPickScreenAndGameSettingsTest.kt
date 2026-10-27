package dev.fastcast.client.debug

import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test

class TheDebugKeysPickScreenAndGameSettingsTest {

    @Test
    fun `screen, frame rate and game keys are interesting`() {
        listOf(
            "ro.vendor.panel.refresh",
            "vendor.hwc.fps",
            "persist.sys.game_mode",
            "debug.sf.frame_rate_multiple_threshold",
            "persist.vendor.memc.level",
            "ro.vendor.hdmi.allm",
            "vendor.tcl.vrr",
            "peak_refresh_rate",
            "user_preferred_resolution_height",
            "match_content_frame_rate",
        ).forEach { assertTrue(it, DebugKeys.interesting(it)) }
    }

    @Test
    fun `unrelated keys are not interesting`() {
        listOf("ro.build.version.sdk", "persist.sys.timezone", "wifi_on", "bluetooth_name", "adb_enabled")
            .forEach { assertFalse(it, DebugKeys.interesting(it)) }
    }

    @Test
    fun `the case of a key does not matter`() {
        assertTrue(DebugKeys.interesting("Vendor.Display.FPS"))
    }
}
