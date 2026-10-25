package dev.fastcast.client.update

class RejectedVersions(
    private val read: () -> Set<Int>,
    private val write: (Set<Int>) -> Unit,
) {

    fun isRejected(versionCode: Int): Boolean = versionCode in read()

    fun reject(versionCode: Int) {
        write(read() + versionCode)
    }
}
