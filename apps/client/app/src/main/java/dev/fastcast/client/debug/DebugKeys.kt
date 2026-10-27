package dev.fastcast.client.debug

object DebugKeys {

    private val INTERESTING = Regex(
        "fps|hz|refresh|frame|vsync|game|allm|vrr|memc|mjc|frc|dlg|panel|hdmi|disp|hwc|surface|sf\\.|timing|pq|" +
            "hdr|dolby|freesync|latency|picture|motion|resolution|tcl|120|144|240|288",
        RegexOption.IGNORE_CASE,
    )

    fun interesting(key: String): Boolean = INTERESTING.containsMatchIn(key)
}
