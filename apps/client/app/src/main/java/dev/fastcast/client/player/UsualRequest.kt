package dev.fastcast.client.player

import dev.fastcast.client.decoder.SupportedCodecs
import dev.fastcast.client.display.DisplayModeOption
import dev.fastcast.client.stream.StreamRequest

object UsualRequest {

    private val TUNNEL_CODECS = listOf(SupportedCodecs.HEVC)

    fun of(
        mode: DisplayModeOption,
        codecsAt: (DisplayModeOption) -> List<String>,
        tenBitAt: (DisplayModeOption) -> Set<String>,
    ): StreamRequest {
        val codecs = if (mode.tunneled) TUNNEL_CODECS else codecsAt(mode)
        val tenBit = tenBitAt(mode).filter { it in codecs }.toSet()
        return StreamRequest(mode.width, mode.height, mode.fps, codecs, tenBit)
    }
}
