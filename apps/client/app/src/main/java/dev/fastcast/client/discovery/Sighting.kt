package dev.fastcast.client.discovery

data class Sighting(
    val server: DiscoveredServer,
    val seenAtNs: Long,
)
