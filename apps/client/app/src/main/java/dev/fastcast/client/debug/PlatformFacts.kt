package dev.fastcast.client.debug

import android.app.ActivityManager
import android.content.Context
import android.os.Build
import android.os.PowerManager
import dev.fastcast.client.BuildConfig
import dev.fastcast.client.player.GameplaySignals

object PlatformFacts {

    private const val STEP = "device"
    private const val BYTES_PER_MEGABYTE = 1_048_576L

    fun of(context: Context): FactSection = FactSection(
        STEP,
        listOfNotNull(
            "model ${Build.MANUFACTURER} ${Build.MODEL}, device ${Build.DEVICE}, product ${Build.PRODUCT}, " +
                "board ${Build.BOARD}, hardware ${Build.HARDWARE}",
            soc(),
            "android ${Build.VERSION.RELEASE}, sdk ${Build.VERSION.SDK_INT}, patch ${Build.VERSION.SECURITY_PATCH}",
            "build ${Build.DISPLAY}, fingerprint ${Build.FINGERPRINT}",
            "app ${BuildConfig.VERSION_NAME} (${BuildConfig.VERSION_CODE})",
            performanceClass(),
            memory(context),
            power(context),
            "game mode ${GameplaySignals.gameMode(context)}",
        ),
    )

    private fun soc(): String? =
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) "soc ${Build.SOC_MANUFACTURER} ${Build.SOC_MODEL}" else null

    private fun performanceClass(): String? =
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
            "media performance class ${Build.VERSION.MEDIA_PERFORMANCE_CLASS}"
        } else {
            null
        }

    private fun memory(context: Context): String? {
        val manager = context.getSystemService(ActivityManager::class.java) ?: return null
        val info = ActivityManager.MemoryInfo().also(manager::getMemoryInfo)
        return "memory ${info.totalMem / BYTES_PER_MEGABYTE} MB, available ${info.availMem / BYTES_PER_MEGABYTE} MB, " +
            "low ram ${manager.isLowRamDevice}, class ${manager.memoryClass} MB"
    }

    private fun power(context: Context): String? {
        val manager = context.getSystemService(PowerManager::class.java) ?: return null
        val thermal = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.Q) manager.currentThermalStatus else null
        return "thermal status $thermal, sustained performance ${manager.isSustainedPerformanceModeSupported}, " +
            "power save ${manager.isPowerSaveMode}"
    }
}
