package dev.fastcast.client.player

import android.content.Context
import android.os.Handler
import android.view.View
import android.widget.TextView
import dev.fastcast.client.R
import dev.fastcast.client.debug.DebugStep
import dev.fastcast.client.debug.TrialResult

class DebugOverlay(
    private val view: TextView,
    private val handler: Handler,
) {

    private val hide = Runnable { view.visibility = View.GONE }

    fun showProgress(step: DebugStep) {
        val context = view.context
        show(context.getString(R.string.debug_step, step.ordinal + 1, DebugStep.entries.size, context.getString(labelOf(step))))
    }

    fun showDone(results: List<TrialResult>) {
        val context = view.context
        val lines = listOf(context.getString(R.string.debug_done)) + results.map { lineOf(context, it) }
        show(lines.joinToString(LINE_BREAK))
        handler.postDelayed(hide, DONE_SHOWN_MS)
    }

    private fun lineOf(context: Context, result: TrialResult): String {
        val trial = result.trial
        val path = if (trial.tunneled) R.string.debug_trial_tunnel else R.string.debug_trial_surface
        val label = context.getString(path, trial.height, trial.fps)
        if (result.presentedPerSecond <= 0.0) {
            return context.getString(R.string.debug_trial_unreported, label, result.queuedPerSecond)
        }
        return context.getString(R.string.debug_trial_shown, label, result.presentedPerSecond, result.queuedPerSecond)
    }

    fun hideNow() {
        handler.removeCallbacks(hide)
        hide.run()
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
        DebugStep.TUNNEL -> R.string.debug_step_tunnel
        DebugStep.RESTORE -> R.string.debug_step_restore
    }

    private companion object {
        const val DONE_SHOWN_MS = 15_000L
        const val LINE_BREAK = "\n"
    }
}
