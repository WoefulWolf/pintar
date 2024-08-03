struct vOut
{
    float4 position : SV_POSITION;
    float4 normal : NORMAL;
    float4 color : COLOR;
    float4 bary_coords : BARY_COORDS;
};

cbuffer MatrixBuffer
{
    matrix viewMatrix;
    float3 sun_direction;
};

#ifdef VS

struct vInput
{
    float4 position : POSITION;
    float4 normal : NORMAL;
    float4 color : COLOR;
    float4 bary_coords : BARY_COORDS;
};

vOut VSMain(const vInput input)
{
    float4 pos = input.position;
    vOut output;
    output.position = mul(pos, transpose(viewMatrix));
    output.normal = input.normal;
    output.color = input.color;
    output.bary_coords = input.bary_coords;

    return output;
}

#else

float map(float value, float min1, float max1, float min2, float max2) {
    float perc = (value - min1) / (max1 - min1);
    return perc * (max2 - min2) + min2;
}

float4 PSMain(const vOut input) : SV_TARGET
{
    clip(input.color.a < 0.1f ? -1 : 1);

    float lighting = dot(input.normal.xyz, sun_direction);
    float4 final_col = input.color;
    final_col.xyz *= (1.0 - lighting);

    float cam_X = viewMatrix._m30;
    float cam_Y = viewMatrix._m31;
    float cam_Z = viewMatrix._m32;

    float3 cam = normalize(float3(cam_X, cam_Y, cam_Z));

    float fresnel = dot(-cam, input.normal.xyz);
    fresnel = saturate(fresnel);
    fresnel = pow(fresnel, 2);

    float b_fresnel = dot(-cam, -input.normal.xyz);
    b_fresnel = saturate(b_fresnel);
    b_fresnel = pow(b_fresnel, 2);

    float final_fresnel = (fresnel + b_fresnel);
    final_fresnel = map(final_fresnel, 0.0, 1.0, 0.3, 1.0);

    final_col.xyz *= final_fresnel;

    float4 outCol = final_col;

    float minBary = min(input.bary_coords.x, min(input.bary_coords.y, input.bary_coords.z));
    minBary = round(smoothstep(0, 0.075f, minBary));

    outCol.a *= (1.25f - minBary);

    return outCol;
}

#endif