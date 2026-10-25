package dev.fastcast.client.update

import com.sun.net.httpserver.HttpContext
import com.sun.net.httpserver.HttpExchange
import com.sun.net.httpserver.HttpServer
import java.net.InetAddress
import java.net.InetSocketAddress
import java.util.concurrent.ConcurrentHashMap
import java.util.concurrent.CountDownLatch
import java.util.concurrent.ExecutorService
import java.util.concurrent.Executors
import java.util.concurrent.TimeUnit
import java.util.concurrent.atomic.AtomicInteger

class FakeUpdateServer : AutoCloseable {

    private val server = HttpServer.create(InetSocketAddress(InetAddress.getLoopbackAddress(), 0), BACKLOG)
    private val workers: ExecutorService = Executors.newCachedThreadPool { Thread(it).apply { isDaemon = true } }
    private val requests = ConcurrentHashMap<String, AtomicInteger>()
    private val contexts = ConcurrentHashMap<String, HttpContext>()
    private val shutdown = CountDownLatch(1)

    private val baseUrl = "http://127.0.0.1:${server.address.port}"

    init {
        server.executor = workers
        server.start()
    }

    fun urlOf(path: String): String = baseUrl + path

    fun serve(path: String, body: ByteArray, status: Int = 200) = respond(path) { exchange ->
        exchange.sendResponseHeaders(status, if (body.isEmpty()) NO_BODY else body.size.toLong())
        if (body.isNotEmpty()) exchange.responseBody.write(body)
    }

    fun redirect(path: String, toPath: String, status: Int = 302) = redirectToUrl(path, urlOf(toPath), status)

    fun redirectToUrl(path: String, location: String, status: Int = 302) = respond(path) { exchange ->
        exchange.responseHeaders.add("Location", location)
        exchange.sendResponseHeaders(status, NO_BODY)
    }

    fun cutShort(path: String, body: ByteArray, announcedSize: Long) = respond(path) { exchange ->
        exchange.sendResponseHeaders(200, announcedSize)
        exchange.responseBody.write(body)
        exchange.responseBody.flush()
    }

    fun neverAnswer(path: String) = respond(path) { shutdown.await(HANG_LIMIT_SECONDS, TimeUnit.SECONDS) }

    fun requestsTo(path: String): Int = requests[path]?.get() ?: 0

    override fun close() {
        shutdown.countDown()
        server.stop(0)
        workers.shutdownNow()
    }

    private fun respond(path: String, handler: (HttpExchange) -> Unit) {
        contexts.remove(path)?.let(server::removeContext)
        contexts[path] = server.createContext(path) { exchange ->
            requests.computeIfAbsent(path) { AtomicInteger() }.incrementAndGet()
            try {
                handler(exchange)
            } finally {
                exchange.close()
            }
        }
    }

    private companion object {
        const val BACKLOG = 0
        const val NO_BODY = -1L
        const val HANG_LIMIT_SECONDS = 30L
    }
}
