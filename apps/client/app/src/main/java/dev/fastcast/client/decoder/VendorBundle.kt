package dev.fastcast.client.decoder

import android.os.Bundle

object VendorBundle {

    fun of(values: Map<String, Number>): Bundle = Bundle().apply {
        values.forEach { (key, value) -> if (value is Float) putFloat(key, value) else putInt(key, value.toInt()) }
    }
}
