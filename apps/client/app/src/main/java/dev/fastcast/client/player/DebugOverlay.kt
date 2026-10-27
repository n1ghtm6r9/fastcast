package dev.fastcast.client.player

import android.os.Handler
import android.view.View
import android.widget.TextView
import dev.fastcast.client.R
import dev.fastcast.client.debug.DebugStep
import dev.fastcast.client.debug.FastStreamResult

class DebugOverlay(
    private val view: TextView,
    private val handler: Handler,
) {

    private val hide = Runnable { view.visibility = View.GONE }

    fun showProgress(step: DebugStep) {
        val context = view.context
        show(context.getString(R.string.debug_step, step.ordinal + 1, DebugStep.entries.size, context.getString(labelOf(step))))
    }

    fun showDone(result: FastStreamResult?) {
        val context = view.context
        show(
            result?.let { context.getString(R.string.debug_done_rates, it.decodedPerSecond, it.presentedPerSecond) }
                ?: context.getString(R.string.debug_done),
        )
        handler.postDelayed(hide, DONE_SHOWN_MS)
    }

    private fun show(text: String) {
        handler.removeCallbacks(hide)
        view.text = text
        view.visibility = View.VISIBLE
    }

    private fun labelOf(step: DebugStep): Int = when (step) {
        DebugStep.FACTS -> R.string.debug_step_facts
        DebugStep.VSYNC -> R.string.debug_step_vsync
        DebugStep.ASK -> R.string.debug_step_ask
        DebugStep.STREAM -> R.string.debug_step_stream
        DebugStep.FAST -> R.string.debug_step_fast
        DebugStep.RESTORE -> R.string.debug_step_restore
    }

    private companion object {
        const val DONE_SHOWN_MS = 15_000L
    }
}
