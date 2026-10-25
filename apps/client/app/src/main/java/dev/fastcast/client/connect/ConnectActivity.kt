package dev.fastcast.client.connect

import android.app.Activity
import android.content.Intent
import android.os.Bundle
import android.os.Handler
import android.os.Looper
import android.util.Log
import android.view.View
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
    private lateinit var serverScroll: View
    private lateinit var statusText: TextView
    private lateinit var hintText: TextView
    private lateinit var errorText: TextView
    private lateinit var updateRunner: UpdateRunner

    private var servers = emptyList<DiscoveredServer>()
    private var rememberedName: String? = null
    private var hintDue = false
    private var updateVersion: String? = null
    private var updatePercent = 0

    private val handler = Handler(Looper.getMainLooper())
    private val deliverOnMain = Executor { handler.post(it) }
    private val finder = ServerFinder(this, deliverOnMain)
    private var policy = AutoConnectPolicy()
    private val installFailure = Runnable { stopUpdating() }
    private val hintTimer = Runnable {
        hintDue = true
        render(servers)
    }
    private val autoConnectTicker = object : Runnable {
        override fun run() {
            connectIfDecided()
            if (policy.isEnabled) handler.postDelayed(this, AUTO_CONNECT_TICK_MS)
        }
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_connect)
        serverStore = RememberedServerStore(this)
        serverList = ServerList(findViewById(R.id.server_list), ::connectTo)
        serverScroll = findViewById(R.id.server_scroll)
        statusText = findViewById(R.id.status_text)
        hintText = findViewById(R.id.hint_text)
        errorText = findViewById(R.id.error_text)
        findViewById<TextView>(R.id.version_text).text = getString(R.string.version_label, BuildConfig.VERSION_NAME)
        updateRunner = UpdateRunner(
            listener = this,
            deliver = deliverOnMain,
            cache = UpdateCheckCache.shared,
            fetch = UpdateChecker()::check,
            download = UpdateDownloader(File(cacheDir, UPDATES_DIRECTORY))::download,
            install = UpdateInstaller(applicationContext)::install,
        )
        if (savedInstanceState != null) policy.disable()
    }

    override fun onStart() {
        super.onStart()
        hintDue = false
        updateVersion = null
        rememberedName = serverStore.loadName()
        render(emptyList())
        policy.holdFor(UPDATE_CHECK_WAIT_MS)
        InstallFailureSignal.listener = installFailure
        updateRunner.start()
        finder.start()
        handler.postDelayed(hintTimer, HINT_DELAY_MS)
        handler.postDelayed(autoConnectTicker, AUTO_CONNECT_TICK_MS)
    }

    override fun onStop() {
        handler.removeCallbacksAndMessages(null)
        InstallFailureSignal.listener = null
        updateRunner.stop()
        finder.stop()
        super.onStop()
    }

    override fun onActivityResult(requestCode: Int, resultCode: Int, data: Intent?) {
        super.onActivityResult(requestCode, resultCode, data)
        if (requestCode != REQUEST_PLAYER) return
        showError(data?.getStringExtra(PlayerActivity.EXTRA_ERROR))
        if (data?.getBooleanExtra(PlayerActivity.EXTRA_UPDATE_FOUND, false) == true) rearmAutoConnect()
    }

    override fun onServersChanged(servers: List<DiscoveredServer>) {
        val firstServerFound = this.servers.isEmpty() && servers.isNotEmpty()
        render(servers)
        if (firstServerFound) serverList.focusFirst()
        connectIfDecided()
    }

    override fun onNoUpdate() {
        policy.release()
    }

    override fun onUpdateFound(manifest: UpdateManifest) {
        policy.holdUntilReleased()
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

    private fun rearmAutoConnect() {
        policy = AutoConnectPolicy().also { it.holdFor(UPDATE_CHECK_WAIT_MS) }
    }

    private fun stopUpdating() {
        updateVersion = null
        policy.release()
        render(servers)
        serverList.focusFirst()
    }

    private fun render(servers: List<DiscoveredServer>) {
        this.servers = servers
        serverList.show(servers)
        val version = updateVersion
        if (version != null) {
            statusText.text = getString(R.string.update_progress, version, updatePercent)
            hintText.visibility = View.GONE
            serverScroll.visibility = View.GONE
        } else {
            statusText.setText(if (servers.isEmpty()) R.string.discovery_searching else R.string.discovery_found)
            hintText.visibility = if (servers.isEmpty() && hintDue) View.VISIBLE else View.GONE
            serverScroll.visibility = View.VISIBLE
        }
    }

    private fun connectIfDecided() {
        policy.choose(servers, rememberedName)?.let(::connectTo)
    }

    private fun connectTo(server: DiscoveredServer) {
        policy.disable()
        serverStore.saveName(server.name)
        showError(null)
        startActivityForResult(PlayerActivity.createIntent(this, server.address), REQUEST_PLAYER)
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
        const val UPDATE_CHECK_WAIT_MS = 3_000L
        const val AUTO_CONNECT_TICK_MS = 250L
    }
}
