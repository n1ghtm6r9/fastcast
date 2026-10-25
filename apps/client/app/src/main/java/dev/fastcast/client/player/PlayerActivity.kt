package dev.fastcast.client.player

import android.app.Activity
import android.content.Context
import android.content.Intent
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
import dev.fastcast.client.connect.ConnectActivity
import dev.fastcast.client.decoder.DecoderSelector
import dev.fastcast.client.decoder.SupportedCodecs
import dev.fastcast.client.discovery.DiscoveredServer
import dev.fastcast.client.display.DisplayModeChooser
import dev.fastcast.client.display.DisplayModes
import dev.fastcast.client.display.MinimalPostProcessing
import dev.fastcast.client.display.PreferredDisplayMode
import dev.fastcast.client.stream.StreamRequest
import dev.fastcast.client.stream.StreamUrl
import dev.fastcast.client.update.UpdateCheckCache
import dev.fastcast.client.update.UpdateChecker
import dev.fastcast.client.update.UpdateLookup
import java.util.concurrent.Executor
import kotlin.math.ceil

class PlayerActivity : Activity(), PlayerSessionListener {

    private lateinit var address: String
    private lateinit var serverName: String
    private lateinit var streamUrl: String
    private lateinit var videoSurface: VideoSurfaceView
    private lateinit var statusPanel: View
    private lateinit var statusSpinner: View
    private lateinit var statusText: TextView
    private lateinit var statsText: TextView
    private lateinit var controlsHint: View
    private lateinit var menuSubtitle: TextView
    private lateinit var menu: PlayerMenu

    private val handler = Handler(Looper.getMainLooper())
    private val updateLookup = UpdateLookup(
        cache = UpdateCheckCache.shared,
        fetch = UpdateChecker()::check,
        deliver = Executor { handler.post(it) },
        onUpdateAvailable = ::leaveForUpdate,
    )
    private val statsTicker = object : Runnable {
        override fun run() {
            renderStats()
            handler.postDelayed(this, STATS_INTERVAL_MS)
        }
    }
    private val hideControlsHint = Runnable { controlsHint.visibility = View.GONE }

    private var screen: Display? = null
    private var session: PlayerSession? = null
    private var lastError: String? = null
    private var statsVisible = false
    private var controlsHintShown = false

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        window.addFlags(WindowManager.LayoutParams.FLAG_KEEP_SCREEN_ON)
        MinimalPostProcessing.request(window)
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
        streamUrl = StreamUrl.build(address, requestStream())
        videoSurface = findViewById(R.id.video_surface)
        statusPanel = findViewById(R.id.status_panel)
        statusSpinner = findViewById(R.id.status_spinner)
        statusText = findViewById(R.id.status_text)
        statsText = findViewById(R.id.stats_text)
        controlsHint = findViewById(R.id.controls_hint)
        menuSubtitle = findViewById(R.id.menu_subtitle)
        findViewById<TextView>(R.id.menu_title).text = serverName
        menu = PlayerMenu(findViewById(R.id.menu_panel), findViewById(R.id.menu_resume), handler)
        findViewById<View>(R.id.menu_resume).setOnClickListener { menu.close() }
        findViewById<View>(R.id.menu_stats).setOnClickListener {
            menu.close()
            toggleStats()
        }
        findViewById<View>(R.id.menu_disconnect).setOnClickListener { finish() }
        findViewById<View>(R.id.player_root).setOnClickListener { openMenu() }
        videoSurface.holder.addCallback(SurfaceLifecycle(::startSession, ::stopSession))
        showStatus(PlayerStatus.Connecting)
    }

    override fun onStart() {
        super.onStart()
        updateLookup.start()
    }

    override fun onStop() {
        updateLookup.stop()
        super.onStop()
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
        null -> super.onKeyDown(keyCode, event)
    }

    override fun finish() {
        setResult(RESULT_OK, Intent().putExtra(EXTRA_ERROR, lastError))
        super.finish()
    }

    override fun onDestroy() {
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

    private fun leave() {
        if (menu.isOpen) menu.close() else finish()
    }

    private fun leaveForUpdate() {
        if (isTaskRoot) startActivity(Intent(this, ConnectActivity::class.java))
        finish()
    }

    private fun openMenu() {
        handler.removeCallbacks(hideControlsHint)
        controlsHint.visibility = View.GONE
        menuSubtitle.text = getString(R.string.menu_subtitle, address, activeDisplayLabel())
        menu.open()
    }

    private fun requestStream(): StreamRequest? {
        val display = screen ?: return null
        val chosen = DisplayModeChooser.choose(DisplayModes.supported(display))
        chosen?.let { PreferredDisplayMode.apply(window, it) }
        val mode = chosen ?: DisplayModes.active(display)
        val codecs = SupportedCodecs.among { mime ->
            DecoderSelector.canDecodeInHardware(mime, mode.width, mode.height, mode.fps)
        }
        return StreamRequest(mode.width, mode.height, mode.fps, codecs)
    }

    private fun activeDisplayLabel(): String = screen?.let { DisplayModes.active(it).label }.orEmpty()

    private fun startSession(surface: Surface) {
        stopSession()
        showStatus(PlayerStatus.Connecting)
        session = PlayerSession(streamUrl, surface, this, ::activeDisplayLabel).also { it.start() }
    }

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

        fun createIntent(context: Context, server: DiscoveredServer): Intent =
            Intent(context, PlayerActivity::class.java)
                .putExtra(EXTRA_ADDRESS, server.address)
                .putExtra(EXTRA_NAME, server.name)
    }
}
