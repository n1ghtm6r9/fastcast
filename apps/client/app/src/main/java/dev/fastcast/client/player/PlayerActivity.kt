package dev.fastcast.client.player

import android.app.Activity
import android.content.Context
import android.content.Intent
import android.media.AudioManager
import android.os.Bundle
import android.os.Handler
import android.os.Looper
import android.os.SystemClock
import android.view.Display
import android.view.KeyEvent
import android.view.Surface
import android.view.View
import android.view.WindowManager
import android.widget.TextView
import dev.fastcast.client.R
import dev.fastcast.client.debug.DebugRun
import dev.fastcast.client.debug.HandlerClock
import dev.fastcast.client.debug.StreamTrial
import dev.fastcast.client.decoder.DecoderPace
import dev.fastcast.client.decoder.DecoderSelector
import dev.fastcast.client.decoder.SupportedCodecs
import dev.fastcast.client.decoder.TunnelAudio
import dev.fastcast.client.decoder.VideoMime
import dev.fastcast.client.discovery.DiscoveredServer
import dev.fastcast.client.display.DisplayModeChooser
import dev.fastcast.client.display.DisplayModeOption
import dev.fastcast.client.display.DisplayModes
import dev.fastcast.client.display.FrameRateHint
import dev.fastcast.client.display.MinimalPostProcessing
import dev.fastcast.client.display.PreferredDisplayMode
import dev.fastcast.client.display.ScreenModeStore
import dev.fastcast.client.display.ScreenModes
import dev.fastcast.client.network.ActiveNetwork
import dev.fastcast.client.stream.ScaleMessage
import dev.fastcast.client.stream.StreamRequest
import kotlin.math.ceil

class PlayerActivity : Activity(), PlayerSessionListener, DebugPlayer {

    private lateinit var address: String
    private lateinit var serverName: String
    private lateinit var offer: StreamOffer
    private lateinit var videoSurface: VideoSurfaceView
    private lateinit var statusPanel: View
    private lateinit var statusSpinner: View
    private lateinit var statusText: TextView
    private lateinit var statsText: TextView
    private lateinit var controlsHint: View
    private lateinit var menuSubtitle: TextView
    private lateinit var screenValue: TextView
    private lateinit var screenPrevious: View
    private lateinit var screenNext: View
    private lateinit var scaleRow: View
    private lateinit var scaleValue: TextView
    private lateinit var scaleSmaller: View
    private lateinit var scaleLarger: View
    private lateinit var scaleToast: TextView
    private lateinit var screenModeStore: ScreenModeStore
    private lateinit var menu: PlayerMenu
    private lateinit var debugOverlay: DebugOverlay

    private val handler = Handler(Looper.getMainLooper())
    private val statsTicker = object : Runnable {
        override fun run() {
            renderStats()
            handler.postDelayed(this, STATS_INTERVAL_MS)
        }
    }
    private val hideControlsHint = Runnable { controlsHint.visibility = View.GONE }
    private val hideScaleToast = Runnable { scaleToast.visibility = View.GONE }

    private var screen: Display? = null
    private var session: PlayerSession? = null
    private var lastError: String? = null
    private var statsVisible = false
    private var controlsHintShown = false
    private var modeSummary = ""
    private var screenModes: List<DisplayModeOption> = emptyList()
    private var requestedMode: DisplayModeOption? = null
    private var screenScale: ScaleMessage? = null
    private var announceScaleUntilMs = 0L
    private var playing = false
    private var debugRun: DebugRun? = null
    private var debugRunId = NO_DEBUG_RUN
    private var tunneled = false
    private var pace = DecoderPace.REALTIME
    private var rebuildingSurface = false

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        window.addFlags(WindowManager.LayoutParams.FLAG_KEEP_SCREEN_ON)
        MinimalPostProcessing.request(window)
        GameplaySignals.announce(this)
        setContentView(R.layout.activity_player)
        ImmersiveMode.enter(window)
        val requestedAddress = intent.getStringExtra(EXTRA_ADDRESS)
        if (requestedAddress == null) {
            finish()
            return
        }
        address = requestedAddress
        serverName = intent.getStringExtra(EXTRA_NAME) ?: requestedAddress
        screen = DisplayModes.defaultDisplay(this)
        screenModeStore = ScreenModeStore(this)
        offer = StreamOffer(address, requestStream())
        tunneled = requestedMode?.tunneled == true
        pace = chosenPace()
        videoSurface = findViewById(R.id.video_surface)
        statusPanel = findViewById(R.id.status_panel)
        statusSpinner = findViewById(R.id.status_spinner)
        statusText = findViewById(R.id.status_text)
        statsText = findViewById(R.id.stats_text)
        controlsHint = findViewById(R.id.controls_hint)
        menuSubtitle = findViewById(R.id.menu_subtitle)
        screenValue = findViewById(R.id.menu_screen_value)
        screenPrevious = findViewById(R.id.menu_screen_previous)
        screenNext = findViewById(R.id.menu_screen_next)
        scaleRow = findViewById(R.id.menu_scale_row)
        scaleValue = findViewById(R.id.menu_scale_value)
        scaleSmaller = findViewById(R.id.menu_scale_smaller)
        scaleLarger = findViewById(R.id.menu_scale_larger)
        scaleToast = findViewById(R.id.scale_toast)
        debugOverlay = DebugOverlay(findViewById(R.id.debug_status), handler)
        findViewById<TextView>(R.id.menu_title).text = serverName
        menu = PlayerMenu(findViewById(R.id.menu_panel), findViewById(R.id.menu_resume), handler)
        findViewById<View>(R.id.menu_resume).setOnClickListener { menu.close() }
        findViewById<View>(R.id.menu_stats).setOnClickListener {
            menu.close()
            toggleStats()
        }
        findViewById<View>(R.id.menu_disconnect).setOnClickListener { finish() }
        screenPrevious.setOnClickListener { stepScreenMode(-1) }
        screenNext.setOnClickListener { stepScreenMode(1) }
        scaleSmaller.setOnClickListener { askScale(-1) }
        scaleLarger.setOnClickListener { askScale(1) }
        findViewById<View>(R.id.player_root).setOnClickListener { openMenu() }
        videoSurface.holder.addCallback(SurfaceLifecycle(::startSession, ::onSurfaceDestroyed))
        showStatus(PlayerStatus.Connecting)
    }

    override fun onWindowFocusChanged(hasFocus: Boolean) {
        super.onWindowFocusChanged(hasFocus)
        if (hasFocus) ImmersiveMode.enter(window)
    }

    override fun dispatchKeyEvent(event: KeyEvent): Boolean {
        if (menu.isOpen) menu.keepOpen()
        return super.dispatchKeyEvent(event)
    }

    override fun onKeyDown(keyCode: Int, event: KeyEvent): Boolean = when (RemoteKeys.actionFor(keyCode)) {
        RemoteAction.Leave -> {
            if (event.repeatCount == 0) leave()
            true
        }
        RemoteAction.ToggleStats -> {
            if (event.repeatCount == 0) toggleStats()
            true
        }
        RemoteAction.ShowMenu -> if (menu.isOpen) {
            super.onKeyDown(keyCode, event)
        } else {
            openMenu()
            true
        }
        RemoteAction.ScaleUp -> scaleByKey(keyCode, event, 1)
        RemoteAction.ScaleDown -> scaleByKey(keyCode, event, -1)
        null -> super.onKeyDown(keyCode, event)
    }

    override fun finish() {
        setResult(RESULT_OK, Intent().putExtra(EXTRA_ERROR, lastError))
        super.finish()
    }

    override fun onDestroy() {
        debugRun?.cancel()
        handler.removeCallbacksAndMessages(null)
        stopSession()
        super.onDestroy()
    }

    override fun onStatus(status: PlayerStatus) {
        trackError(status)
        showStatus(status)
    }

    override fun onVideoSize(width: Int, height: Int) {
        videoSurface.setVideoSize(width, height)
    }

    override fun onScale(scale: ScaleMessage) {
        screenScale = scale
        renderScaleRow()
        if (SystemClock.uptimeMillis() < announceScaleUntilMs) showScaleToast(scale)
    }

    override fun onDiagnose(run: Long) {
        if (run == debugRunId || debugRun?.isRunning == true) return
        debugRunId = run
        debugRun = DebugRun(PlayerDebugHost(this, this, debugOverlay), HandlerClock(handler)).also { it.start() }
    }

    override fun currentSession(): PlayerSession? = session

    override fun isPlaying(): Boolean = playing

    override fun outputSurface(): Surface? = videoSurface.holder.surface.takeIf { it.isValid }

    override fun usualRefreshRate(): Float? = requestedMode?.refreshRate

    override fun streamTrial(trial: StreamTrial): String {
        lastError = null
        switchStream(StreamRequest(trial.width, trial.height, trial.fps, TUNNEL_CODECS), trial.tunneled, trial.pace)
        val path = if (trial.tunneled) TUNNELED_PATH else SURFACE_PATH
        val paceName = trial.pace.name.lowercase()
        return "asked ${trial.width}x${trial.height}@${trial.fps} $path $paceName codecs ${TUNNEL_CODECS.joinToString(",")}"
    }

    override fun lastError(): String? = lastError

    override fun streamUsual() = streamChosenMode()

    private fun streamChosenMode() {
        val request = requestStream()
        switchStream(request, requestedMode?.tunneled == true, chosenPace())
    }

    private fun chosenPace(): DecoderPace = DecoderPace.of(requestedMode?.bestEffort == true)

    private fun switchStream(request: StreamRequest?, throughTunnel: Boolean, nextPace: DecoderPace) {
        val leavingTunnel = tunneled && !throughTunnel
        tunneled = throughTunnel
        pace = nextPace
        offer = StreamOffer(address, request)
        val surface = videoSurface.holder.surface
        if (!surface.isValid) return
        if (leavingTunnel) rebuildSurface() else startSession(surface)
    }

    private fun rebuildSurface() {
        rebuildingSurface = true
        videoSurface.visibility = View.GONE
    }

    private fun onSurfaceDestroyed() {
        stopSession()
        if (!rebuildingSurface) return
        rebuildingSurface = false
        handler.post { videoSurface.visibility = View.VISIBLE }
    }

    private fun leave() {
        if (menu.isOpen) menu.close() else finish()
    }

    private fun openMenu() {
        handler.removeCallbacks(hideControlsHint)
        controlsHint.visibility = View.GONE
        menuSubtitle.text = getString(R.string.menu_subtitle, address, activeDisplayLabel())
        renderScreenRow()
        askScale(NO_STEP)
        menu.open()
    }

    private fun stepScreenMode(delta: Int) {
        val next = ScreenModes.step(screenModes, requestedMode, delta) ?: return
        screenModeStore.save(next)
        streamChosenMode()
        renderScreenRow()
    }

    private fun renderScreenRow() {
        val mode = requestedMode ?: return
        val format = if (mode.tunneled) R.string.menu_screen_mode_tunnel else R.string.menu_screen_mode
        screenValue.text = getString(format, mode.width, mode.height, mode.fps)
        showStep(screenPrevious, ScreenModes.step(screenModes, mode, -1) != null)
        showStep(screenNext, ScreenModes.step(screenModes, mode, 1) != null)
    }

    private fun showStep(button: View, possible: Boolean) {
        button.isEnabled = possible
        button.alpha = if (possible) 1f else UNAVAILABLE_STEP_ALPHA
    }

    private fun scaleByKey(keyCode: Int, event: KeyEvent, step: Int): Boolean {
        if (menu.isOpen) return super.onKeyDown(keyCode, event)
        if (event.repeatCount == 0 && askScale(step)) {
            announceScaleUntilMs = SystemClock.uptimeMillis() + SCALE_ANSWER_MS
        }
        return true
    }

    private fun askScale(step: Int): Boolean = session?.askScale(step) ?: false

    private fun renderScaleRow() {
        val scale = screenScale ?: return
        scaleRow.visibility = View.VISIBLE
        scaleValue.text = getString(R.string.menu_scale_value, scale.percent)
        showStep(scaleSmaller, scale.percent > scale.min)
        showStep(scaleLarger, scale.percent < scale.max)
    }

    private fun showScaleToast(scale: ScaleMessage) {
        scaleToast.text = getString(R.string.scale_toast, scale.percent)
        scaleToast.visibility = View.VISIBLE
        handler.removeCallbacks(hideScaleToast)
        handler.postDelayed(hideScaleToast, SCALE_TOAST_MS)
    }

    private fun requestStream(): StreamRequest? {
        val display = screen ?: return null
        val modes = DisplayModes.supported(display)
        val sustains = { mode: DisplayModeOption -> hardwareCodecsAt(mode).isNotEmpty() }
        screenModes = ScreenModes.withTunnels(ScreenModes.offer(modes, sustains), ::canTunnel, ::allowsTunnel)
        val chosen = ScreenModes.saved(screenModes, screenModeStore.load()) ?: DisplayModeChooser.choose(modes, sustains)
        chosen?.let { PreferredDisplayMode.apply(window, it) }
        val mode = chosen ?: DisplayModes.active(display)
        requestedMode = mode
        val codecs = if (mode.tunneled) TUNNEL_CODECS else hardwareCodecsAt(mode)
        val tenBit = if (mode.tunneled) emptySet() else tenBitCodecsAt(mode)
        modeSummary = ModeSummary.of(modes, codecs, tenBit, screenModes.filter { it.tunneled })
        return StreamRequest(mode.width, mode.height, mode.fps, codecs, tenBit)
    }

    private fun canTunnel(width: Int, height: Int, fps: Int): Boolean =
        DecoderSelector.canTunnelInHardware(VideoMime.HEVC, width, height, fps)

    private fun allowsTunnel(width: Int, height: Int, fps: Int): Boolean =
        DecoderSelector.allowsTunnelInHardware(VideoMime.HEVC, width, height, fps)

    private fun hardwareCodecsAt(mode: DisplayModeOption): List<String> = SupportedCodecs.among { mime ->
        DecoderSelector.canDecodeInHardware(mime, mode.width, mode.height, mode.fps)
    }

    private fun tenBitCodecsAt(mode: DisplayModeOption): Set<String> = SupportedCodecs.among { mime ->
        DecoderSelector.canDecodeTenBitInHardware(mime, mode.width, mode.height, mode.fps)
    }.toSet()

    private fun activeDisplayLabel(): String = screen?.let { DisplayModes.active(it).label }.orEmpty()

    private fun reportedDisplayLabel(): String =
        listOf(activeDisplayLabel(), modeSummary, gameSummary()).filter { it.isNotEmpty() }.joinToString(REPORT_SEPARATOR)

    private fun gameSummary(): String =
        GameModeSummary.of(screen?.let(MinimalPostProcessing::supported) ?: false, GameplaySignals.gameMode(this))

    private fun networkLabel(): String? = ActiveNetwork.describe(this)

    private fun startSession(surface: Surface) {
        stopSession()
        showStatus(PlayerStatus.Connecting)
        requestedMode?.let { FrameRateHint.apply(surface, it.refreshRate) }
        session = PlayerSession(offer, surface, this, ::reportedDisplayLabel, ::networkLabel, tunnelAudio(), pace)
            .also { it.start() }
    }

    private fun tunnelAudio(): TunnelAudio? =
        if (tunneled) TunnelAudio(getSystemService(AudioManager::class.java).generateAudioSessionId()) else null

    private fun stopSession() {
        session?.stop()
        session = null
    }

    private fun trackError(status: PlayerStatus) {
        lastError = when (status) {
            is PlayerStatus.Reconnecting -> getString(R.string.error_connection, status.reason)
            is PlayerStatus.DecoderError -> getString(R.string.status_decoder_error, status.message)
            PlayerStatus.Playing -> null
            else -> lastError
        }
    }

    private fun showStatus(status: PlayerStatus) {
        playing = status == PlayerStatus.Playing
        val message = statusMessage(status)
        statusText.text = message
        statusPanel.visibility = if (message == null) View.GONE else View.VISIBLE
        statusSpinner.visibility = if (status is PlayerStatus.DecoderError) View.GONE else View.VISIBLE
        if (status == PlayerStatus.Playing) showControlsHintOnce()
    }

    private fun showControlsHintOnce() {
        if (controlsHintShown || menu.isOpen) return
        controlsHintShown = true
        controlsHint.visibility = View.VISIBLE
        handler.postDelayed(hideControlsHint, CONTROLS_HINT_MS)
    }

    private fun statusMessage(status: PlayerStatus): String? = when (status) {
        PlayerStatus.Connecting -> getString(R.string.status_connecting, serverName)
        PlayerStatus.WaitingForVideo -> getString(R.string.status_waiting_for_video)
        PlayerStatus.Playing -> null
        is PlayerStatus.Reconnecting -> getString(R.string.status_reconnecting, secondsUntil(status.retryInMs))
        is PlayerStatus.DecoderError -> getString(R.string.status_decoder_error, status.message)
    }

    private fun secondsUntil(delayMs: Long): Int = ceil(delayMs / MILLISECONDS_PER_SECOND).toInt()

    private fun toggleStats() {
        statsVisible = !statsVisible
        statsText.visibility = if (statsVisible) View.VISIBLE else View.GONE
        handler.removeCallbacks(statsTicker)
        if (statsVisible) statsTicker.run()
    }

    private fun renderStats() {
        val snapshot = session?.snapshot(SystemClock.elapsedRealtime()) ?: return
        statsText.text = getString(
            R.string.stats_format,
            snapshot.width,
            snapshot.height,
            snapshot.framesPerSecond,
            snapshot.megabitsPerSecond,
            snapshot.decoderName.ifEmpty { NO_DECODER },
            snapshot.droppedFrames,
            snapshot.delayMs?.let { getString(R.string.stats_delay_ms, it) } ?: NO_DELAY,
        )
    }

    companion object {
        const val EXTRA_ERROR = "dev.fastcast.client.player.ERROR"

        private const val EXTRA_ADDRESS = "dev.fastcast.client.player.ADDRESS"
        private const val EXTRA_NAME = "dev.fastcast.client.player.NAME"
        private const val STATS_INTERVAL_MS = 1_000L
        private const val CONTROLS_HINT_MS = 4_000L
        private const val MILLISECONDS_PER_SECOND = 1_000.0
        private const val NO_DECODER = "-"
        private const val NO_DELAY = "-"
        private const val REPORT_SEPARATOR = "; "
        private const val UNAVAILABLE_STEP_ALPHA = 0.35f
        private const val NO_STEP = 0
        private const val SCALE_ANSWER_MS = 2_000L
        private const val SCALE_TOAST_MS = 1_500L
        private const val NO_DEBUG_RUN = 0L
        private const val TUNNELED_PATH = "tunneled"
        private const val SURFACE_PATH = "surface"
        private val TUNNEL_CODECS = listOf(SupportedCodecs.HEVC)

        fun createIntent(context: Context, server: DiscoveredServer): Intent =
            Intent(context, PlayerActivity::class.java)
                .putExtra(EXTRA_ADDRESS, server.address)
                .putExtra(EXTRA_NAME, server.name)
    }
}
