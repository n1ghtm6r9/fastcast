package dev.fastcast.client.decoder

import android.media.AudioAttributes
import android.media.AudioFormat
import android.media.AudioTimestamp
import android.media.AudioTrack
import java.nio.ByteBuffer
import java.util.Locale
import java.util.concurrent.atomic.AtomicLong

class TunnelAudio(val sessionId: Int) {

    private val chunksWritten = AtomicLong()
    private val timestamp = AudioTimestamp()
    private var writer: Thread? = null

    @Volatile
    private var track: AudioTrack? = null

    @Volatile
    private var sampler: AudioClockSampler? = null

    @Volatile
    private var writing = false

    @Volatile
    private var failure: String? = null

    fun start() {
        if (track != null) return
        val created = try {
            createTrack()
        } catch (e: Exception) {
            failure = FailureText.of(e)
            return
        }
        track = created
        writing = true
        chunksWritten.set(0L)
        sampler = AudioClockSampler(System.nanoTime())
        created.play()
        writer = Thread({ writeSilence(created) }, THREAD_NAME).apply { start() }
    }

    fun stop() {
        val active = track ?: return
        writing = false
        runQuietly(active::stop)
        writer?.join(JOIN_TIMEOUT_MS)
        writer = null
        runQuietly(active::release)
        track = null
        sampler = null
    }

    @Synchronized
    fun clockUs(nowNanos: Long): Long? {
        val active = track ?: return null
        val clock = sampler ?: return null
        return clock.clockUs(nowNanos) { played(active) }
    }

    fun describe(): String {
        val active = track ?: return failure?.let { "$AUDIO failed: $it" } ?: "$AUDIO not started"
        return try {
            DESCRIPTION.format(
                Locale.ROOT,
                AUDIO,
                sessionId,
                active.playState,
                active.playbackHeadPosition.toDouble() / TunnelClock.SAMPLE_RATE,
                chunksWritten.get() * TunnelClock.CHUNK_MS / MILLISECONDS_PER_SECOND,
                active.underrunCount,
            ) + clockFacts() + failure?.let { ", failed: $it" }.orEmpty()
        } catch (e: Exception) {
            "$AUDIO: ${FailureText.of(e)}"
        }
    }

    private fun played(active: AudioTrack): PlayedAudio? = try {
        if (active.getTimestamp(timestamp)) {
            PlayedAudio(timestamp.framePosition, timestamp.nanoTime)
        } else {
            PlayedAudio(active.playbackHeadPosition.toLong() and UNSIGNED_INT, System.nanoTime())
        }
    } catch (e: Exception) {
        null
    }

    private fun clockFacts(): String {
        val clock = sampler ?: return ""
        val movedNanos = clock.movedAfterNanos ?: return CLOCK_STILL
        val rate = clock.rate()?.let { RATE.format(Locale.ROOT, it) } ?: UNKNOWN_RATE
        return CLOCK_MOVED.format(Locale.ROOT, movedNanos / NANOS_PER_MILLISECOND, rate)
    }

    private fun createTrack(): AudioTrack = AudioTrack.Builder()
        .setAudioAttributes(
            AudioAttributes.Builder()
                .setUsage(AudioAttributes.USAGE_MEDIA)
                .setContentType(AudioAttributes.CONTENT_TYPE_MOVIE)
                .setFlags(AudioAttributes.FLAG_HW_AV_SYNC)
                .build(),
        )
        .setAudioFormat(
            AudioFormat.Builder()
                .setEncoding(AudioFormat.ENCODING_PCM_16BIT)
                .setSampleRate(TunnelClock.SAMPLE_RATE)
                .setChannelMask(AudioFormat.CHANNEL_OUT_STEREO)
                .build(),
        )
        .setTransferMode(AudioTrack.MODE_STREAM)
        .setBufferSizeInBytes(bufferBytes())
        .setSessionId(sessionId)
        .build()

    private fun bufferBytes(): Int {
        val minimum = AudioTrack.getMinBufferSize(
            TunnelClock.SAMPLE_RATE,
            AudioFormat.CHANNEL_OUT_STEREO,
            AudioFormat.ENCODING_PCM_16BIT,
        )
        return maxOf(minimum * BUFFER_MULTIPLE, TunnelClock.CHUNK_BYTES * MIN_BUFFERED_CHUNKS)
    }

    private fun writeSilence(target: AudioTrack) {
        val silence = ByteBuffer.allocateDirect(TunnelClock.CHUNK_BYTES)
        var index = 0L
        try {
            while (writing) {
                silence.clear()
                val written =
                    target.write(silence, TunnelClock.CHUNK_BYTES, AudioTrack.WRITE_BLOCKING, TunnelClock.chunkNanos(index))
                if (written < 0) {
                    failure = "write returned $written"
                    return
                }
                if (written == TunnelClock.CHUNK_BYTES) {
                    index++
                    chunksWritten.incrementAndGet()
                }
            }
        } catch (e: Exception) {
            failure = FailureText.of(e)
        }
    }

    private companion object {
        const val THREAD_NAME = "fastcast-tunnel-audio"
        const val JOIN_TIMEOUT_MS = 1_000L
        const val BUFFER_MULTIPLE = 2
        const val MIN_BUFFERED_CHUNKS = 4
        const val MILLISECONDS_PER_SECOND = 1_000.0
        const val NANOS_PER_MILLISECOND = 1_000_000L
        const val UNSIGNED_INT = 0xFFFF_FFFFL
        const val AUDIO = "audio clock"
        const val DESCRIPTION = "%s session %d: play state %d, played %.2f s, written %.2f s, underruns %d"
        const val CLOCK_STILL = ", clock not moving yet"
        const val CLOCK_MOVED = ", clock moved after %d ms, rate %s"
        const val RATE = "%.4f"
        const val UNKNOWN_RATE = "unknown"
    }
}
