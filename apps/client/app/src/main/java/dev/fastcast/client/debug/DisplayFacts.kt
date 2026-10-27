package dev.fastcast.client.debug

import android.content.Context
import android.hardware.display.DisplayManager
import android.os.Build
import android.view.Display

object DisplayFacts {

    private const val STEP = "display"
    private const val NO_DISPLAY_MANAGER = "no display manager"

    fun of(context: Context): FactSection {
        val manager = context.getSystemService(DisplayManager::class.java)
            ?: return FactSection(STEP, listOf(NO_DISPLAY_MANAGER))
        return FactSection(STEP, listOfNotNull(matchContentFrameRate(manager)) + manager.displays.flatMap(::factsOf))
    }

    private fun matchContentFrameRate(manager: DisplayManager): String? =
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
            "match content frame rate ${manager.matchContentFrameRateUserPreference}"
        } else {
            null
        }

    private fun factsOf(display: Display): List<String> = listOfNotNull(
        "display ${display.displayId} ${display.name}, state ${display.state}, flags ${display.flags}, " +
            "refresh ${display.refreshRate}",
        "mode ${modeOf(display.mode)}",
        "modes ${display.supportedModes.joinToString(" ") { modeOf(it) }}",
        "vsync offset ${display.appVsyncOffsetNanos} ns, deadline ${display.presentationDeadlineNanos} ns",
        hdrOf(display),
        minimalPostProcessingOf(display),
        productOf(display),
    )

    private fun modeOf(mode: Display.Mode): String {
        val base = "${mode.modeId}:${mode.physicalWidth}x${mode.physicalHeight}@${mode.refreshRate}"
        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.S) return base
        val alternatives = mode.alternativeRefreshRates
        return if (alternatives.isEmpty()) base else "$base(alt ${alternatives.joinToString(",")})"
    }

    private fun hdrOf(display: Display): String {
        val capabilities = display.hdrCapabilities
        return "hdr ${display.isHdr} types [${hdrTypesOf(display).joinToString(",")}] " +
            "max ${capabilities?.desiredMaxLuminance} nits, wide gamut ${display.isWideColorGamut}"
    }

    @Suppress("DEPRECATION")
    private fun hdrTypesOf(display: Display): IntArray =
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.UPSIDE_DOWN_CAKE) {
            display.mode.supportedHdrTypes
        } else {
            display.hdrCapabilities?.supportedHdrTypes ?: IntArray(0)
        }

    private fun minimalPostProcessingOf(display: Display): String? =
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.R) {
            "minimal post processing ${display.isMinimalPostProcessingSupported}"
        } else {
            null
        }

    private fun productOf(display: Display): String? {
        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.S) return null
        val product = display.deviceProductInfo ?: return "product unknown"
        return "product ${product.name} pnp ${product.manufacturerPnpId} id ${product.productId} " +
            "year ${product.modelYear} sink ${product.connectionToSinkType}"
    }
}
