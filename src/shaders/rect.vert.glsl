struct VSInput
{
    float3 a_pos : POSITION;
    float4 a_clr : COLOR0;
};

struct VSOutput
{
    float4 position : SV_Position;
    float4 v_clr    : COLOR0;
};

VSOutput main(VSInput input)
{
    VSOutput output;
    output.position = float4(input.a_pos, 1.0);
    output.v_clr = input.a_clr;
    return output;
}
