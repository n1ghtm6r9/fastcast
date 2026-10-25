package dev.fastcast.client.discovery

import java.util.concurrent.Executor

object DirectExecutor : Executor {

    override fun execute(command: Runnable) = command.run()
}
