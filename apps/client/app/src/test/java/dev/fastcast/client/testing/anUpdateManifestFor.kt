package dev.fastcast.client.testing

import dev.fastcast.client.update.UpdateManifest

fun anUpdateManifestFor(payload: ByteArray, url: String, versionCode: Int = 101): UpdateManifest =
    anUpdateManifest(versionCode = versionCode, url = url, sha256 = sha256Hex(payload), size = payload.size.toLong())
