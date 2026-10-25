package dev.fastcast.client.player

import android.app.Activity
import android.content.Context
import android.content.Intent
import android.os.Bundle
import android.os.Handler
import android.os.Looper
import android.os.SystemClock
import android.view.KeyEvent
import android.view.Surface
import android.view.View
import android.view.WindowManager
import android.widget.TextView
import dev.fastcast.client.R
import dev.fastcast.client.connect.ConnectActivity
import dev.fastcast.client.update.UpdateCheckCache
import dev.fastcast.client.update.UpdateChecker
import dev.fastcast.client.update.UpdateLookup
import java.util.concurrent.Executor
import kotlin.math.ceil

class PlayerActivity : Activity(), PlayerSessionListener {

    private lateinit var address: String
    private lateinit var videoSurface: VideoSurfaceView
    private lateinit var statusText: TextView
    private lateinit var statsText: TextView

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

    private var session: PlayerSession? = null
    private var lastError: String? = null
    private var statsVisible = false
    private var leavingForUpdate = false

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        window.addFlags(WindowManager.LayoutParams.FLAG_KEEP_SCREEN_ON)
        setContentView(R.layout.activity_player)
        ImmersiveMode.enter(window)
        val requestedAddress = intent.getStringExtra(EXTRA_ADDRESS)
        if (requestedAddress == null) {
            finish()
            return
        }
        address = requestedAddress
        videoSurface = findViewById(R.id.video_surface)
        statusText = findViewById(R.id.status_text)
        statsText = findViewById(R.id.stats_text)
        findViewById<View>(R.id.player_root).setOnClickListener { toggleStats() }
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

    override fun onKeyDown(keyCode: Int, event: KeyEvent): Boolean {
        if (keyCode in STATS_KEYS) {
            if (event.repeatCount == 0) toggleStats()
            return true
        }
        return super.onKeyDown(keyCode, event)
    }

    override fun finish() {
        setResult(RESULT_OK, Intent().putExtra(EXTRA_ERROR, lastError).putExtra(EXTRA_UPDATE_FOUND, leavingForUpdate))
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

    private fun leaveForUpdate() {
        leavingForUpdate = true
        if (isTaskRoot) startActivity(Intent(this, ConnectActivity::class.java))
        finish()
    }

    private fun startSession(surface: Surface) {
        stopSession()
        showStatus(PlayerStatus.Connecting)
        session = PlayerSession(address, surface, this).also { it.start() }
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
        statusText.visibility = if (message == null) View.GONE else View.VISIBLE
    }

    private fun statusMessage(status: PlayerStatus): String? = when (status) {
        PlayerStatus.Connecting -> getString(R.string.status_connecting)
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
        const val EXTRA_ADDRESS = "dev.fastcast.client.player.ADDRESS"
        const val EXTRA_ERROR = "dev.fastcast.client.player.ERROR"
        const val EXTRA_UPDATE_FOUND = "dev.fastcast.client.player.UPDATE_FOUND"

        private const val STATS_INTERVAL_MS = 1_000L
        private const val MILLISECONDS_PER_SECOND = 1_000.0
        private const val NO_DECODER = "-"
        private val STATS_KEYS = setOf(
            KeyEvent.KEYCODE_DPAD_CENTER,
            KeyEvent.KEYCODE_ENTER,
            KeyEvent.KEYCODE_NUMPAD_ENTER,
            KeyEvent.KEYCODE_INFO,
            KeyEvent.KEYCODE_MENU,
        )

        fun createIntent(context: Context, address: String): Intent =
            Intent(context, PlayerActivity::class.java).putExtra(EXTRA_ADDRESS, address)
    }
}
