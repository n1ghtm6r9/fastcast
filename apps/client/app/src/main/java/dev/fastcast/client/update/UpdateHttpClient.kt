package dev.fastcast.client.update

import java.util.concurrent.TimeUnit
import okhttp3.OkHttpClient

object UpdateHttpClient {

    private const val CONNECT_TIMEOUT_SECONDS = 5L
    private const val READ_TIMEOUT_SECONDS = 10L

    val shared: OkHttpClient by lazy(::create)

    fun create(): OkHttpClient = OkHttpClient.Builder()
        .connectTimeout(CONNECT_TIMEOUT_SECONDS, TimeUnit.SECONDS)
        .readTimeout(READ_TIMEOUT_SECONDS, TimeUnit.SECONDS)
        .followRedirects(true)
        .followSslRedirects(false)
        .build()
}
