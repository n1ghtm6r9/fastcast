package dev.fastcast.client.discovery

interface ServerFinderListener {

    fun onServersChanged(servers: List<DiscoveredServer>)
}
