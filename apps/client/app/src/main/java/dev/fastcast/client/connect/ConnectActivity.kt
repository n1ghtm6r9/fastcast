package dev.fastcast.client.connect

import android.app.Activity
import android.content.Intent
import android.os.Bundle
import android.os.Handler
import android.os.Looper
import android.util.Log
import android.view.KeyEvent
import android.view.View
import android.widget.ProgressBar
import android.widget.TextView
import dev.fastcast.client.BuildConfig
import dev.fastcast.client.R
import dev.fastcast.client.discovery.DiscoveredServer
import dev.fastcast.client.discovery.ServerFinder
import dev.fastcast.client.discovery.ServerFinderListener
import dev.fastcast.client.player.PlayerActivity
import dev.fastcast.client.update.InstallFailureSignal
import dev.fastcast.client.update.UpdateCheckCache
import dev.fastcast.client.update.UpdateChecker
import dev.fastcast.client.update.UpdateDownloader
import dev.fastcast.client.update.UpdateInstaller
import dev.fastcast.client.update.UpdateListener
import dev.fastcast.client.update.UpdateManifest
import dev.fastcast.client.update.UpdateRunner
import java.io.File
import java.util.concurrent.Executor

class ConnectActivity : Activity(), ServerFinderListener, UpdateListener {

    private lateinit var serverStore: RememberedServerStore
    private lateinit var serverList: ServerList
    private lateinit var searchingPanel: View
    private lateinit var updateProgress: ProgressBar
    private lateinit var statusText: TextView
    private lateinit var hintText: View
    private lateinit var errorText: TextView
    private lateinit var controlsHint: View
    private lateinit var updateRunner: UpdateRunner

    private var focus = ServerFocus()
    private var servers = emptyList<DiscoveredServer>()
    private var rememberedName: String? = null
    private var updateVersion: String? = null
    private var updatePercent = 0
    private var playerStarting = false

    private val handler = Handler(Looper.getMainLooper())
    private val deliverOnMain = Executor { handler.post(it) }
    private val finder = ServerFinder(this, deliverOnMain)
    private val installFailure = Runnable { stopUpdating() }
    private val hintTimer = Runnable { hintText.visibility = View.VISIBLE }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_connect)
        serverStore = RememberedServerStore(this)
        serverList = ServerList(findViewById(R.id.server_grid), ::connectTo) { server, index -> focus.focusedOn(server, index) }
        searchingPanel = findViewById(R.id.searching_panel)
        updateProgress = findViewById(R.id.update_progress)
        statusText = findViewById(R.id.status_text)
        hintText = findViewById(R.id.hint_text)
        errorText = findViewById(R.id.error_text)
        controlsHint = findViewById(R.id.controls_hint)
        findViewById<TextView>(R.id.version_text).text = getString(R.string.version_label, BuildConfig.VERSION_NAME)
        updateRunner = UpdateRunner(
            listener = this,
            deliver = deliverOnMain,
            cache = UpdateCheckCache.shared,
            fetch = UpdateChecker()::check,
            download = UpdateDownloader(File(cacheDir, UPDATES_DIRECTORY))::download,
            install = UpdateInstaller(applicationContext)::install,
        )
    }

    override fun onStart() {
        super.onStart()
        playerStarting = false
        focus = ServerFocus()
        hintText.visibility = View.INVISIBLE
        updateVersion = null
        rememberedName = serverStore.loadName()
        render(emptyList())
        InstallFailureSignal.listener = installFailure
        updateRunner.start()
        finder.start()
        handler.postDelayed(hintTimer, HINT_DELAY_MS)
    }

    override fun onStop() {
        handler.removeCallbacksAndMessages(null)
        InstallFailureSignal.listener = null
        updateRunner.stop()
        finder.stop()
        super.onStop()
    }

    override fun dispatchKeyEvent(event: KeyEvent): Boolean {
        if (event.action == KeyEvent.ACTION_DOWN && event.keyCode in ARROW_KEYS && servers.isNotEmpty()) focus.userMoved()
        return super.dispatchKeyEvent(event)
    }

    override fun onActivityResult(requestCode: Int, resultCode: Int, data: Intent?) {
        super.onActivityResult(requestCode, resultCode, data)
        if (requestCode == REQUEST_PLAYER) showError(data?.getStringExtra(PlayerActivity.EXTRA_ERROR))
    }

    override fun onServersChanged(servers: List<DiscoveredServer>) {
        render(servers)
        focusServer()
    }

    override fun onNoUpdate() = Unit

    override fun onUpdateFound(manifest: UpdateManifest) {
        updateVersion = manifest.versionName
        updatePercent = 0
        render(servers)
    }

    override fun onUpdateProgress(percent: Int) {
        updatePercent = percent
        render(servers)
    }

    override fun onUpdateFailed(cause: Exception) {
        Log.w(TAG, "Update failed", cause)
        stopUpdating()
    }

    private fun stopUpdating() {
        updateVersion = null
        render(servers)
        focusServer()
    }

    private fun render(servers: List<DiscoveredServer>) {
        this.servers = servers
        val version = updateVersion
        serverList.show(if (version == null) servers else emptyList(), rememberedName)
        controlsHint.visibility = if (version == null && servers.isNotEmpty()) View.VISIBLE else View.INVISIBLE
        searchingPanel.visibility = if (version == null && servers.isEmpty()) View.VISIBLE else View.GONE
        updateProgress.visibility = if (version != null) View.VISIBLE else View.GONE
        updateProgress.progress = updatePercent
        statusText.text = when {
            version != null -> getString(R.string.update_progress, version, updatePercent)
            servers.isEmpty() -> getString(R.string.discovery_searching)
            else -> getString(R.string.discovery_found)
        }
    }

    private fun focusServer() {
        if (updateVersion != null) return
        focus.indexToFocus(servers, rememberedName)?.let(serverList::focus)
    }

    private fun connectTo(server: DiscoveredServer) {
        if (playerStarting) return
        playerStarting = true
        serverStore.saveName(server.name)
        showError(null)
        startActivityForResult(PlayerActivity.createIntent(this, server), REQUEST_PLAYER)
    }

    private fun showError(message: String?) {
        errorText.text = message
        errorText.visibility = if (message.isNullOrEmpty()) View.GONE else View.VISIBLE
    }

    private companion object {
        const val TAG = "fastcast-connect"
        const val UPDATES_DIRECTORY = "updates"
        const val REQUEST_PLAYER = 1
        const val HINT_DELAY_MS = 5_000L
        val ARROW_KEYS = setOf(
            KeyEvent.KEYCODE_DPAD_UP,
            KeyEvent.KEYCODE_DPAD_DOWN,
            KeyEvent.KEYCODE_DPAD_LEFT,
            KeyEvent.KEYCODE_DPAD_RIGHT,
        )
    }
}
