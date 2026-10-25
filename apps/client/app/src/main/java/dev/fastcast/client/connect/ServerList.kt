package dev.fastcast.client.connect

import android.view.LayoutInflater
import android.view.View
import android.widget.LinearLayout
import android.widget.TextView
import dev.fastcast.client.R
import dev.fastcast.client.discovery.DiscoveredServer

class ServerList(
    private val container: LinearLayout,
    private val onChosen: (DiscoveredServer) -> Unit,
) {

    private val inflater = LayoutInflater.from(container.context)

    fun show(servers: List<DiscoveredServer>) {
        val excess = container.childCount - servers.size
        if (excess > 0) container.removeViews(servers.size, excess)
        servers.forEachIndexed { index, server -> bind(rowAt(index), server) }
    }

    fun focusFirst() {
        container.getChildAt(0)?.requestFocus()
    }

    private fun rowAt(index: Int): View =
        container.getChildAt(index) ?: inflater.inflate(R.layout.item_server, container, false).also { row ->
            row.setOnClickListener { onChosen(row.tag as DiscoveredServer) }
            container.addView(row)
        }

    private fun bind(row: View, server: DiscoveredServer) {
        row.tag = server
        row.findViewById<TextView>(R.id.server_name).text = server.name
        row.findViewById<TextView>(R.id.server_address).text = server.address
    }
}
