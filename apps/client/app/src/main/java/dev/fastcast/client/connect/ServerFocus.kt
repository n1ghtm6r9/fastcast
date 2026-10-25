package dev.fastcast.client.connect

import dev.fastcast.client.discovery.DiscoveredServer

class ServerFocus {

    private var movedByUser = false
    private var focusedAddress: String? = null
    private var focusedIndex = 0

    fun userMoved() {
        movedByUser = true
    }

    fun focusedOn(server: DiscoveredServer, index: Int) {
        focusedAddress = server.address
        focusedIndex = index
    }

    fun indexToFocus(servers: List<DiscoveredServer>, rememberedName: String?): Int? {
        if (servers.isEmpty()) return null
        val index = chooseIndex(servers, rememberedName)
        focusedOn(servers[index], index)
        return index
    }

    private fun chooseIndex(servers: List<DiscoveredServer>, rememberedName: String?): Int {
        val remembered = servers.indexOfFirst { it.name == rememberedName }
        if (!movedByUser && remembered >= 0) return remembered
        val kept = servers.indexOfFirst { it.address == focusedAddress }
        return if (kept >= 0) kept else focusedIndex.coerceAtMost(servers.lastIndex)
    }
}
