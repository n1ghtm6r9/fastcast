package dev.fastcast.client.decoder

class DecodeOnlyProbe(private val frames: Int) {

    private var dueNanos: Long? = null
    private var taken = 0

    fun decodesOnly(nowNanos: Long): Boolean {
        val due = dueNanos ?: (nowNanos + FIRST_NANOS).also { dueNanos = it }
        if (taken >= frames || nowNanos - due < 0) return false
        taken++
        dueNanos = nowNanos + EVERY_NANOS
        return true
    }

    fun restart() {
        dueNanos = null
        taken = 0
    }

    private companion object {
        const val FIRST_NANOS = 2_000_000_000L
        const val EVERY_NANOS = 1_000_000_000L
    }
}
