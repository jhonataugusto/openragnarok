#language slang 2026

import globals;

[[vk::binding(0, 0)]] var global_uniforms: ConstantBuffer<GlobalUniforms>;
[[vk::binding(2, 0)]] var linear_sampler: SamplerState;
[[vk::binding(0, 2)]] var skybox_texture: Texture2D;

static const var PI_HALF: float = 1.57079632679;
static const var TWO_PI: float = 6.28318530718;
static const var SKYDOME_LONGITUDE_SEGMENTS: uint = 64;
static const var SKYDOME_LATITUDE_SEGMENTS: uint = 24;
static const var SKYDOME_VERTEX_COUNT: uint = SKYDOME_LONGITUDE_SEGMENTS * SKYDOME_LATITUDE_SEGMENTS * 6;
static const var SKYDOME_RADIUS: float = 512.0;
static const var SKYDOME_LOW_ANGLE: float = -0.35;
static const var SKYDOME_TEXTURE_EDGE_PADDING: float = 0.001;

struct DomeVertex {
    var direction: float3;
    var uv: float2;
};

struct SkyboxVertex {
    float4 position : SV_Position;
    [[vk::location(0)]] var uv: float2;
};

[ForceInline]
func quad_uv(corner_index: uint) -> float2 {
    if (corner_index == 0 || corner_index == 3) {
        return float2(0.0, 1.0);
    }

    if (corner_index == 1) {
        return float2(0.0, 0.0);
    }

    if (corner_index == 2 || corner_index == 4) {
        return float2(1.0, 0.0);
    }

    return float2(1.0, 1.0);
}

[ForceInline]
func skydome_vertex(vertex_index: uint) -> DomeVertex {
    let quad_index = vertex_index / 6;
    let corner_index = vertex_index % 6;
    let longitude_index = quad_index % SKYDOME_LONGITUDE_SEGMENTS;
    let latitude_index = quad_index / SKYDOME_LONGITUDE_SEGMENTS;
    let corner_uv = quad_uv(corner_index);
    let dome_uv =
        (float2(float(longitude_index), float(latitude_index)) + corner_uv) /
        float2(float(SKYDOME_LONGITUDE_SEGMENTS), float(SKYDOME_LATITUDE_SEGMENTS));
    let longitude = dome_uv.x * TWO_PI;
    let latitude = PI_HALF + (SKYDOME_LOW_ANGLE - PI_HALF) * dome_uv.y;
    let latitude_radius = cos(latitude);

    var vertex: DomeVertex;
    vertex.direction = normalize(float3(sin(longitude) * latitude_radius, sin(latitude), cos(longitude) * latitude_radius));
    vertex.uv = float2(dome_uv.x, clamp(dome_uv.y, SKYDOME_TEXTURE_EDGE_PADDING, 1.0 - SKYDOME_TEXTURE_EDGE_PADDING));
    return vertex;
}

[[shader("vertex")]]
func vs_main(uint vertex_index : SV_VulkanVertexID) -> SkyboxVertex {
    let vertex = skydome_vertex(vertex_index);
    let world_position = global_uniforms.camera_position.xyz + vertex.direction * SKYDOME_RADIUS;

    var output: SkyboxVertex;
    output.position = mul(global_uniforms.view_projection, float4(world_position, 1.0));
    output.position.z = 0.0;
    output.uv = vertex.uv;
    return output;
}

[[shader("pixel")]]
func fs_main(input: SkyboxVertex) -> float4 {
    let color = skybox_texture.Sample(linear_sampler, input.uv).rgb;
    return float4(color, 1.0);
}
