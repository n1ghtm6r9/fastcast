package dev.fastcast.client.update

object InstallFailureSignal {

    @Volatile
    var listener: Runnable? = null
}
