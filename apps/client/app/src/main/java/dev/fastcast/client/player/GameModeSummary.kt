package dev.fastcast.client.player

import android.app.GameManager

object GameModeSummary {

    private const val YES = "yes"
    private const val NO = "no"
    private const val SEPARATOR = ", "
    private const val UNSUPPORTED = "unsupported"
    private const val STANDARD = "standard"
    private const val PERFORMANCE = "performance"
    private const val BATTERY = "battery"
    private const val CUSTOM = "custom"

    fun of(minimalPostProcessing: Boolean, gameMode: Int?): String = listOfNotNull(
        "minimal post-processing ${if (minimalPostProcessing) YES else NO}",
        gameMode?.let { "game mode ${nameOf(it)}" },
    ).joinToString(SEPARATOR)

    private fun nameOf(gameMode: Int): String = when (gameMode) {
        GameManager.GAME_MODE_UNSUPPORTED -> UNSUPPORTED
        GameManager.GAME_MODE_STANDARD -> STANDARD
        GameManager.GAME_MODE_PERFORMANCE -> PERFORMANCE
        GameManager.GAME_MODE_BATTERY -> BATTERY
        GameManager.GAME_MODE_CUSTOM -> CUSTOM
        else -> gameMode.toString()
    }
}
