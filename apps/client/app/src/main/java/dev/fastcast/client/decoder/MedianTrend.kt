package dev.fastcast.client.decoder

class MedianTrend(private val groupSize: Int, private val groups: Int) {

    private val filling = ArrayList<Long>(groupSize)
    private val finished = ArrayDeque<Long>()

    @Synchronized
    fun add(value: Long) {
        filling.add(value)
        if (filling.size < groupSize) return
        finished.addLast(filling.sorted()[groupSize / 2])
        filling.clear()
        if (finished.size > groups) finished.removeFirst()
    }

    @Synchronized
    fun medians(): List<Long> = finished.toList()
}
