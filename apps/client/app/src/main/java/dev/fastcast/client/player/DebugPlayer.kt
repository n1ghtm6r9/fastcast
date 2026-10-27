package dev.fastcast.client.player

import android.view.Surface
import dev.fastcast.client.debug.StreamTrial

interface DebugPlayer {

    fun currentSession(): PlayerSession?

    fun isPlaying(): Boolean

    fun outputSurface(): Surface?

    fun usualRefreshRate(): Float?

    fun streamTrial(trial: StreamTrial): String

    fun lastError(): String?

    fun streamUsual()
}
