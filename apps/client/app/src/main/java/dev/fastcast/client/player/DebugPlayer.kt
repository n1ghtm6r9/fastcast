package dev.fastcast.client.player

import android.view.Surface

interface DebugPlayer {

    fun currentSession(): PlayerSession?

    fun isPlaying(): Boolean

    fun outputSurface(): Surface?

    fun usualRefreshRate(): Float?

    fun streamFast(): String

    fun streamUsual()
}
