cbuffer Noise : register(b0)
{
    uint frame;
    uint levels;
    float output_scale;
    uint padding;
};

uint scrambled(uint value)
{
    uint state = value * 747796405u + 2891336453u;
    uint word = ((state >> ((state >> 28u) + 4u)) ^ state) * 277803737u;

    return (word >> 22u) ^ word;
}

float4 main(float4 position : SV_Position) : SV_Target
{
    uint2 pixel = uint2(position.xy);
    uint seed = scrambled(pixel.x + scrambled(pixel.y + scrambled(frame)));

    return float4(seed % levels, (seed >> 16) % levels, 0, 0) * output_scale;
}
