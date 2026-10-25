package dev.fastcast.client.connect

import android.view.LayoutInflater
import android.view.View
import android.widget.GridLayout
import android.widget.TextView
import dev.fastcast.client.R
import dev.fastcast.client.discovery.DiscoveredServer

class ServerList(
    private val grid: GridLayout,
    private val onChosen: (DiscoveredServer) -> Unit,
    private val onFocused: (DiscoveredServer, Int) -> Unit,
) {

    private val inflater = LayoutInflater.from(grid.context)
    private var rebinding = false

    fun show(servers: List<DiscoveredServer>, rememberedName: String?) {
        rebinding = true
        val excess = grid.childCount - servers.size
        if (excess > 0) grid.removeViews(servers.size, excess)
        servers.forEachIndexed { index, server ->
            val card = grid.getChildAt(index) ?: newCard().also(grid::addView)
            bind(card, server, server.name == rememberedName)
        }
        rebinding = false
    }

    fun focus(index: Int) {
        grid.getChildAt(index)?.requestFocus()
    }

    private fun newCard(): View =
        inflater.inflate(R.layout.item_server, grid, false).also { card ->
            card.setOnClickListener { onChosen(card.tag as DiscoveredServer) }
            card.setOnFocusChangeListener { _, hasFocus -> if (hasFocus) reportFocus(card) }
        }

    private fun reportFocus(card: View) {
        val server = card.tag as? DiscoveredServer ?: return
        if (!rebinding) onFocused(server, grid.indexOfChild(card))
    }

    private fun bind(card: View, server: DiscoveredServer, lastUsed: Boolean) {
        card.tag = server
        card.findViewById<TextView>(R.id.server_name).text = server.name
        card.findViewById<TextView>(R.id.server_address).text = server.host
        card.findViewById<View>(R.id.server_badge).visibility = if (lastUsed) View.VISIBLE else View.GONE
    }
}
