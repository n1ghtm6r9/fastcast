cbuffer Parameters : register(b0)
{
    int cursor_left;
    int cursor_top;
    int cursor_width;
    int cursor_height;
    uint cursor_visible;
    uint cursor_kind;
    int source_width;
    float output_scale;
    float luma_black;
    float luma_range;
    float chroma_centre;
    float chroma_range;
};

Texture2D<float4> desktop_image : register(t0);
Texture2D<float4> cursor_image : register(t1);

static const uint CURSOR_MASKED = 1;
static const uint CHANNEL_MAX = 255;
static const float3 BT709_WEIGHTS = float3(0.2126, 0.7152, 0.0722);

uint3 composited_sample(int2 position)
{
    uint3 screen = (uint3)round(desktop_image.Load(int3(position, 0)).rgb * 255.0);
    int2 offset = position - int2(cursor_left, cursor_top);

    if (cursor_visible == 0 || any(offset < 0) || any(offset >= int2(cursor_width, cursor_height)))
    {
        return screen;
    }

    float4 shape = cursor_image.Load(int3(offset, 0));
    uint3 colour = (uint3)round(shape.rgb * 255.0);
    uint alpha = (uint)round(shape.a * 255.0);

    if (cursor_kind == CURSOR_MASKED)
    {
        uint kept = alpha == CHANNEL_MAX ? CHANNEL_MAX : 0u;

        return (screen & kept) ^ colour;
    }

    return (colour * alpha + screen * (CHANNEL_MAX - alpha) + CHANNEL_MAX / 2) / CHANNEL_MAX;
}

float3 unit_rgb(int2 position)
{
    return composited_sample(position) / 255.0;
}
