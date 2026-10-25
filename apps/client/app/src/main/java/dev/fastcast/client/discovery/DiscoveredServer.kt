package dev.fastcast.client.discovery

data class DiscoveredServer(
    val name: String,
    val host: String,
    val port: Int,
) {

    val address: String
        get() = "$host:$port"
}
