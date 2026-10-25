package dev.fastcast.client.player

import java.io.File
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test

class ThePlayerNeverChecksForUpdatesTest {

    private val sources = File(PLAYER_SOURCES).listFiles { file -> file.extension == "kt" }.orEmpty().toList()

    @Test
    fun `the player sources are found`() {
        assertTrue(sources.isNotEmpty())
    }

    @Test
    fun `no player source refers to the update package`() {
        val offenders = sources.filter { it.readText().contains(UPDATE_PACKAGE) }.map(File::getName)

        assertEquals(emptyList<String>(), offenders)
    }

    private companion object {
        const val PLAYER_SOURCES = "src/main/java/dev/fastcast/client/player"
        const val UPDATE_PACKAGE = "dev.fastcast.client.update"
    }
}
