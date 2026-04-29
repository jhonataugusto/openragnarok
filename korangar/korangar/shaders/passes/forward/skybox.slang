#language slang 2026

import globals;
import screen_space;

[[vk::binding(0, 0)]] var global_uniforms: ConstantBuffer<GlobalUniforms>;
[[vk::binding(2, 0)]] var linear_sampler: SamplerState;
[[vk::binding(0, 2)]] var skybox_texture: Texture2D;

static const var PI: float = 3.14159265359;
static const var TWO_PI: float = 6.28318530718;

struct SkyboxVertex {
    float4 position : SV_Position;
    [[vk::location(0)]] var direction: float3;
};

[[shader("vertex")]]
func vs_main(uint vertex_index : SV_VulkanVertexID) -> SkyboxVertex {
    let fullscreen = FullscreenVertex.new(vertex_index);
    let clip_position = float4(fullscreen.position.xy, 0.0, 1.0);
    let world_position = mul(global_uniforms.inverse_view_projection, clip_position);
    let world = world_position.xyz / world_position.w;

    var output: SkyboxVertex;
    output.position = fullscreen.position;
    output.direction = normalize(world - global_uniforms.camera_position.xyz);
    return output;
}

[[shader("pixel")]]
func fs_main(input: SkyboxVertex) -> float4 {
    let direction = normalize(input.direction);
    let u = 0.5 + atan2(direction.x, direction.z) / TWO_PI;
    let v = 0.5 - asin(clamp(direction.y, -1.0, 1.0)) / PI;
    let color = skybox_texture.Sample(linear_sampler, float2(u, v)).rgb;
    return float4(color, 1.0);
}
