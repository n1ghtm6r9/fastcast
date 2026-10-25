package dev.fastcast.client.discovery

import java.net.InetAddress
import java.net.NetworkInterface
import java.net.SocketException

object BroadcastTargets {
    private val LIMITED_BROADCAST: InetAddress = InetAddress.getByAddress(byteArrayOf(-1, -1, -1, -1))

    fun find(): List<InetAddress> = (listOf(LIMITED_BROADCAST) + directedBroadcasts()).distinct()

    private fun directedBroadcasts(): List<InetAddress> =
        interfaces()
            .filterNot(::isDownOrLoopback)
            .flatMap { it.interfaceAddresses }
            .mapNotNull { it.broadcast }

    private fun interfaces(): List<NetworkInterface> = try {
        NetworkInterface.getNetworkInterfaces()?.toList().orEmpty()
    } catch (e: SocketException) {
        emptyList()
    }

    private fun isDownOrLoopback(networkInterface: NetworkInterface): Boolean = try {
        !networkInterface.isUp || networkInterface.isLoopback
    } catch (e: SocketException) {
        false
    }
}
