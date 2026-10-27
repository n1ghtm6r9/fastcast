package dev.fastcast.client.decoder

class PresentTimes(private val capacity: Int) {

    private val times = LongArray(capacity)
    private var next = 0
    private var count = 0

    @Synchronized
    fun add(nanos: Long) {
        times[next] = nanos
        next = (next + 1) % capacity
        if (count < capacity) count++
    }

    @Synchronized
    fun since(startNanos: Long): LongArray {
        val oldest = (next - count + capacity) % capacity
        return LongArray(count) { times[(oldest + it) % capacity] }.filter { it >= startNanos }.toLongArray()
    }
}
