package dev.fastcast.client.debug

import android.content.Context
import android.content.pm.FeatureInfo

object FeatureFacts {

    private const val STEP = "features"

    fun of(context: Context): FactSection =
        FactSection(STEP, context.packageManager.systemAvailableFeatures.map(::describe).sorted())

    private fun describe(feature: FeatureInfo): String {
        val name = feature.name ?: return "opengl es ${feature.glEsVersion}"
        return if (feature.version == 0) name else "$name=${feature.version}"
    }
}
