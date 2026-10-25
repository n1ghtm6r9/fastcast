package dev.fastcast.client.update

import android.app.PendingIntent
import android.content.Context
import android.content.Intent
import android.content.IntentSender
import android.content.pm.PackageInstaller
import android.os.Build
import java.io.File

class UpdateInstaller(private val context: Context) {

    fun install(apk: File, manifest: UpdateManifest) {
        val installer = context.packageManager.packageInstaller
        val sessionId = installer.createSession(sessionParams(manifest))
        try {
            installer.openSession(sessionId).use { session ->
                copyInto(session, apk)
                session.commit(statusSender(sessionId))
            }
        } catch (e: Exception) {
            runCatching { installer.abandonSession(sessionId) }
            throw e
        }
    }

    private fun sessionParams(manifest: UpdateManifest): PackageInstaller.SessionParams =
        PackageInstaller.SessionParams(PackageInstaller.SessionParams.MODE_FULL_INSTALL).apply {
            setAppPackageName(context.packageName)
            setSize(manifest.size)
            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
                setRequireUserAction(PackageInstaller.SessionParams.USER_ACTION_NOT_REQUIRED)
            }
        }

    private fun copyInto(session: PackageInstaller.Session, apk: File) {
        apk.inputStream().use { input ->
            session.openWrite(ENTRY_NAME, 0, apk.length()).use { output ->
                input.copyTo(output)
                session.fsync(output)
            }
        }
    }

    private fun statusSender(sessionId: Int): IntentSender {
        val statusIntent = Intent(context, InstallStatusReceiver::class.java)
        val flags = PendingIntent.FLAG_UPDATE_CURRENT or mutabilityFlag()
        return PendingIntent.getBroadcast(context, sessionId, statusIntent, flags).intentSender
    }

    private fun mutabilityFlag(): Int =
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) PendingIntent.FLAG_MUTABLE else 0

    private companion object {
        const val ENTRY_NAME = "fastcast-tv.apk"
    }
}
