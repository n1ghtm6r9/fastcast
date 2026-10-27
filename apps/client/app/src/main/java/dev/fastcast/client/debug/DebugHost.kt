package dev.fastcast.client.debug

interface DebugHost {

    fun factSections(): List<FactSection>

    fun measureVsync(durationMs: Long, done: (LongArray) -> Unit)

    fun askRefreshRate(rate: Float)

    fun clearRefreshRate()

    fun displayNow(): String

    fun isPlaying(): Boolean

    fun send(message: String): Boolean

    fun counters(): DebugCounters?

    fun presentedSince(startNanos: Long): LongArray

    fun streamTrial(trial: StreamTrial): String

    fun trialFacts(): List<String>

    fun lastError(): String?

    fun streamUsual()

    fun showProgress(step: DebugStep)

    fun hideProgress()

    fun showDone(results: List<TrialResult>)
}
