struct PSInput
{
    float4 v_clr : COLOR0;
};

float4 main(PSInput input) : SV_Target0
{
    return input.v_clr;
}
