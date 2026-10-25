package dev.fastcast.client.update

import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.content.pm.PackageInstaller
import android.os.Build
import android.util.Log

class InstallStatusReceiver : BroadcastReceiver() {

    override fun onReceive(context: Context, intent: Intent) {
        val status = intent.getIntExtra(PackageInstaller.EXTRA_STATUS, PackageInstaller.STATUS_FAILURE)
        val message = intent.getStringExtra(PackageInstaller.EXTRA_STATUS_MESSAGE)
        when (status) {
            PackageInstaller.STATUS_PENDING_USER_ACTION -> askUser(context, intent)
            PackageInstaller.STATUS_SUCCESS -> Log.i(TAG, "Update installed")
            else -> onFailure(status, message)
        }
    }

    private fun askUser(context: Context, intent: Intent) {
        val confirmation = confirmationIntent(intent) ?: return
        try {
            context.startActivity(confirmation.addFlags(Intent.FLAG_ACTIVITY_NEW_TASK))
        } catch (e: RuntimeException) {
            Log.w(TAG, "Could not show the install confirmation", e)
        }
    }

    private fun onFailure(status: Int, message: String?) {
        Log.w(TAG, "Update install failed with status $status: $message")
        UpdateCheckCache.shared.dismiss()
        InstallFailureSignal.listener?.run()
    }

    private fun confirmationIntent(intent: Intent): Intent? =
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
            intent.getParcelableExtra(Intent.EXTRA_INTENT, Intent::class.java)
        } else {
            @Suppress("DEPRECATION")
            intent.getParcelableExtra(Intent.EXTRA_INTENT)
        }

    private companion object {
        const val TAG = "fastcast-update"
    }
}
