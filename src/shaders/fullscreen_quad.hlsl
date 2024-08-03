struct vOut
{
    float4 pos : SV_POSITION;
    float2 uv : TEXCOORD;
};

#ifdef VS

vOut VSMain(uint vI: SV_VERTEXID)
{
    vOut Out;
    Out.uv = float2(vI%2,vI%4/2);
    Out.pos = float4((Out.uv.x-0.5f)*2,-(Out.uv.y-0.5f)*2,0,1);
    return Out;
}

#else

sampler sampler0;
Texture2D texture0;

float4 PSMain(vOut input) : SV_TARGET {
  return texture0.Sample(sampler0, input.uv);
}
#endif