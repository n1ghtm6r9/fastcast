package dev.fastcast.client.testing

import dev.fastcast.client.update.RejectedVersions
import java.util.concurrent.atomic.AtomicReference

fun aRejectedVersions(vararg versionCodes: Int): RejectedVersions {
    val saved = AtomicReference(versionCodes.toSet())
    return RejectedVersions(read = saved::get, write = saved::set)
}
