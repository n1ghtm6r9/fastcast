float4 main(uint vertex : SV_VertexID) : SV_Position
{
    float2 corner = float2((vertex << 1) & 2, vertex & 2);

    return float4(corner * float2(2.0, -2.0) + float2(-1.0, 1.0), 0.0, 1.0);
}
