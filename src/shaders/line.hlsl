struct vOut
{
    float4 position : SV_POSITION;
    float4 color : COLOR;
};

cbuffer viewMatrixBuffer
{
	matrix viewMatrix;
	float3 sun_direction;
};

#ifdef VS

struct vInput
{
	float4 position1 : POSITION;
	float4 Position2 : POSITIONNEXT;
	float4 color : COLOR;
	float thickness : THICKNESS;
};

vOut VSMain(const vInput input)
{
	vOut output;

	float4 clipPos = mul(input.position1, transpose(viewMatrix));
	float4 clipNextPos = mul(input.Position2, transpose(viewMatrix));

	float3 tangent = clipNextPos - clipPos;
	float3 normal = normalize(cross(clipPos, tangent));

	float4 offset = float4(normal.x, normal.y, normal.z, 0.0f);

	output.position = clipPos + offset * input.thickness;
	output.color = input.color;

	return output;
}

#else

float4 PSMain(const vOut input) : SV_TARGET
{
    return input.color;
}

#endif