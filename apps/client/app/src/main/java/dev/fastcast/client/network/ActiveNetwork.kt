package dev.fastcast.client.network

import android.content.Context
import android.net.ConnectivityManager
import android.net.NetworkCapabilities
import java.io.File

object ActiveNetwork {

    private const val ETHERNET = "ethernet"
    private const val WIFI = "wifi"
    private const val OTHER = "other"
    private const val SPEED_FILE = "/sys/class/net/%s/speed"

    fun describe(context: Context): String? = runCatching { query(context) }.getOrNull()

    private fun query(context: Context): String? {
        val connectivity = context.getSystemService(ConnectivityManager::class.java) ?: return null
        val network = connectivity.activeNetwork ?: return null
        val capabilities = connectivity.getNetworkCapabilities(network) ?: return null
        val speed = connectivity.getLinkProperties(network)?.interfaceName?.let(::speedOf)
        return NetworkLabel.of(transportOf(capabilities), speed)
    }

    private fun transportOf(capabilities: NetworkCapabilities): String = when {
        capabilities.hasTransport(NetworkCapabilities.TRANSPORT_ETHERNET) -> ETHERNET
        capabilities.hasTransport(NetworkCapabilities.TRANSPORT_WIFI) -> WIFI
        else -> OTHER
    }

    private fun speedOf(interfaceName: String): Int? =
        LinkSpeed.megabitsFrom(runCatching { File(SPEED_FILE.format(interfaceName)).readText() }.getOrNull())
}
