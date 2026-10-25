package dev.fastcast.client.testing

import dev.fastcast.client.discovery.DiscoveredServer

fun aServer(name: String = "Gaming PC", host: String = "192.168.0.10", port: Int = 3278): DiscoveredServer =
    DiscoveredServer(name, host, port)
