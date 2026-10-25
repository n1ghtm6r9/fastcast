float3 filtered_row(int row, int chroma_column)
{
    int centre = chroma_column * 2;
    int left = max(centre - 1, 0);
    int right = min(centre + 1, source_width - 1);

    return 0.25 * unit_rgb(int2(left, row))
        + 0.5 * unit_rgb(int2(centre, row))
        + 0.25 * unit_rgb(int2(right, row));
}

float2 main(float4 position : SV_Position) : SV_Target
{
    int2 chroma = int2(position.xy);
    float3 rgb = (filtered_row(chroma.y * 2, chroma.x) + filtered_row(chroma.y * 2 + 1, chroma.x)) / 2.0;
    float luma = dot(rgb, BT709_WEIGHTS);
    float2 difference = float2(rgb.b - luma, rgb.r - luma) / float2(1.8556, 1.5748);
    float2 code = floor(chroma_centre + chroma_range * difference + 0.5);

    return code * output_scale;
}
