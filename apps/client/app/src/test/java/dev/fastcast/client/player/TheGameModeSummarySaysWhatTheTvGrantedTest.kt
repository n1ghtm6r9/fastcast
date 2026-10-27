package dev.fastcast.client.player

import android.app.GameManager
import org.junit.Assert.assertEquals
import org.junit.Test

class TheGameModeSummarySaysWhatTheTvGrantedTest {

    @Test
    fun `the summary names the minimal post-processing and the game mode`() {
        assertEquals(
            "minimal post-processing yes, game mode standard",
            GameModeSummary.of(true, GameManager.GAME_MODE_STANDARD),
        )
        assertEquals(
            "minimal post-processing no, game mode unsupported",
            GameModeSummary.of(false, GameManager.GAME_MODE_UNSUPPORTED),
        )
        assertEquals(
            "minimal post-processing yes, game mode performance",
            GameModeSummary.of(true, GameManager.GAME_MODE_PERFORMANCE),
        )
    }

    @Test
    fun `a game mode without a name is shown by its number`() {
        assertEquals("minimal post-processing yes, game mode 7", GameModeSummary.of(true, 7))
    }

    @Test
    fun `without a game manager only the post-processing is named`() {
        assertEquals("minimal post-processing yes", GameModeSummary.of(true, null))
    }
}
