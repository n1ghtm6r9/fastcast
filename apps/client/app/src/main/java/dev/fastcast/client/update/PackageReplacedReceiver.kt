package dev.fastcast.client.update

import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.util.Log
import dev.fastcast.client.connect.ConnectActivity

class PackageReplacedReceiver : BroadcastReceiver() {

    override fun onReceive(context: Context, intent: Intent) {
        if (intent.action != Intent.ACTION_MY_PACKAGE_REPLACED) return
        val launch = Intent(context, ConnectActivity::class.java).addFlags(Intent.FLAG_ACTIVITY_NEW_TASK)
        runCatching { context.startActivity(launch) }
            .onFailure { Log.w(TAG, "Could not reopen the app after the update", it) }
    }

    private companion object {
        const val TAG = "fastcast-update"
    }
}
