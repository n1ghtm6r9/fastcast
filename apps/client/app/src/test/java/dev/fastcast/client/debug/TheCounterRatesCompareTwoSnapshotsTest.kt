package dev.fastcast.client.debug

import org.junit.Assert.assertEquals
import org.junit.Test

class TheCounterRatesCompareTwoSnapshotsTest {

    private val from = DebugCounters(nanos = 1_000_000_000L, bytes = 0L, decoded = 0L, rendered = 0L, dropped = 0L)
    private val to = DebugCounters(
        nanos = 3_000_000_000L,
        bytes = 25_000_000L,
        decoded = 240L,
        rendered = 230L,
        dropped = 10L,
    )

    @Test
    fun `the rates are the changes per second`() {
        val rates = CounterRates.between(from, to)

        assertEquals(120.0, rates.decodedPerSecond, 1e-9)
        assertEquals(115.0, rates.renderedPerSecond, 1e-9)
        assertEquals(5.0, rates.droppedPerSecond, 1e-9)
        assertEquals(100.0, rates.megabitsPerSecond, 1e-9)
    }

    @Test
    fun `the description lists every rate`() {
        assertEquals(
            "decoded 120.0/s, rendered 115.0/s, dropped 5.0/s, 100.0 Mbit/s",
            CounterRates.between(from, to).describe(),
        )
    }

    @Test
    fun `no elapsed time means no rates`() {
        val rates = CounterRates.between(to, to)

        assertEquals(0.0, rates.decodedPerSecond, 0.0)
        assertEquals(0.0, rates.renderedPerSecond, 0.0)
        assertEquals(0.0, rates.droppedPerSecond, 0.0)
        assertEquals(0.0, rates.megabitsPerSecond, 0.0)
    }
}
