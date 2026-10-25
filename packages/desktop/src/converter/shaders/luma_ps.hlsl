float main(float4 position : SV_Position) : SV_Target
{
    float luma = dot(unit_rgb(int2(position.xy)), BT709_WEIGHTS);

    return floor(luma_black + luma_range * luma + 0.5) * output_scale;
}
