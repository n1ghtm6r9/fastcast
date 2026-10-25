package dev.fastcast.client.player

import android.content.Context
import android.util.AttributeSet
import android.view.SurfaceView
import kotlin.math.min
import kotlin.math.roundToInt

class VideoSurfaceView @JvmOverloads constructor(
    context: Context,
    attrs: AttributeSet? = null,
) : SurfaceView(context, attrs) {

    private var videoWidth = 0
    private var videoHeight = 0

    fun setVideoSize(width: Int, height: Int) {
        if (width <= 0 || height <= 0) return
        if (width == videoWidth && height == videoHeight) return
        videoWidth = width
        videoHeight = height
        requestLayout()
    }

    override fun onMeasure(widthMeasureSpec: Int, heightMeasureSpec: Int) {
        if (videoWidth == 0 || videoHeight == 0) {
            super.onMeasure(widthMeasureSpec, heightMeasureSpec)
            return
        }
        val availableWidth = MeasureSpec.getSize(widthMeasureSpec)
        val availableHeight = MeasureSpec.getSize(heightMeasureSpec)
        val scale = min(availableWidth.toFloat() / videoWidth, availableHeight.toFloat() / videoHeight)
        setMeasuredDimension((videoWidth * scale).roundToInt(), (videoHeight * scale).roundToInt())
    }
}
