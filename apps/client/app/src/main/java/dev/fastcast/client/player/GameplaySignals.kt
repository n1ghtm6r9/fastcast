package dev.fastcast.client.player

import android.app.GameManager
import android.app.GameState
import android.content.Context
import android.os.Build
import dev.fastcast.client.decoder.runQuietly

object GameplaySignals {

    fun announce(context: Context) {
        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.TIRAMISU) return
        runQuietly {
            context.getSystemService(GameManager::class.java)
                ?.setGameState(GameState(false, GameState.MODE_GAMEPLAY_UNINTERRUPTIBLE))
        }
    }

    fun gameMode(context: Context): Int? {
        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.S) return null
        return runCatching { context.getSystemService(GameManager::class.java)?.gameMode }.getOrNull()
    }
}
