package dev.fastcast.client.update

interface UpdateListener {

    fun onNoUpdate()

    fun onUpdateFound(manifest: UpdateManifest)

    fun onUpdateProgress(percent: Int)

    fun onUpdateFailed(cause: Exception)
}
