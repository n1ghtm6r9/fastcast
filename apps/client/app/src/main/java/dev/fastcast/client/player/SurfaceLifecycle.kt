package dev.fastcast.client.player

import android.view.Surface
import android.view.SurfaceHolder

class SurfaceLifecycle(
    private val onCreated: (Surface) -> Unit,
    private val onDestroyed: () -> Unit,
) : SurfaceHolder.Callback {

    override fun surfaceCreated(holder: SurfaceHolder) = onCreated(holder.surface)

    override fun surfaceChanged(holder: SurfaceHolder, format: Int, width: Int, height: Int) = Unit

    override fun surfaceDestroyed(holder: SurfaceHolder) = onDestroyed()
}
