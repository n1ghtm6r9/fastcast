package dev.fastcast.client.display

import android.content.Context
import android.hardware.display.DisplayManager
import android.view.Display

object DisplayModes {

    fun defaultDisplay(context: Context): Display? =
        context.getSystemService(DisplayManager::class.java)?.getDisplay(Display.DEFAULT_DISPLAY)

    fun supported(display: Display): List<DisplayModeOption> = display.supportedModes.map(::optionOf)

    fun active(display: Display): DisplayModeOption = optionOf(display.mode)

    private fun optionOf(mode: Display.Mode): DisplayModeOption =
        DisplayModeOption(mode.modeId, mode.physicalWidth, mode.physicalHeight, mode.refreshRate)
}
