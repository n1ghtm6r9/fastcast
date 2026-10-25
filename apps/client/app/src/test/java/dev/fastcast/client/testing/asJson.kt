package dev.fastcast.client.testing

import dev.fastcast.client.update.UpdateManifest

fun UpdateManifest.asJson(): String = anUpdateManifestJson(
    "versionCode" to versionCode,
    "versionName" to versionName,
    "url" to url,
    "sha256" to sha256,
    "size" to size,
)
