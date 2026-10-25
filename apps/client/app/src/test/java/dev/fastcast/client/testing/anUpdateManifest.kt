package dev.fastcast.client.testing

import dev.fastcast.client.update.UpdateManifest

fun anUpdateManifest(
    versionCode: Int = 101,
    url: String = "https://github.com/n1ghtm6r9/fastcast/releases/download/v0.1.101/fastcast-tv.apk",
    sha256: String = "a".repeat(64),
    size: Long = 4_321_000L,
): UpdateManifest = UpdateManifest(versionCode, "0.1.$versionCode", url, sha256, size)
