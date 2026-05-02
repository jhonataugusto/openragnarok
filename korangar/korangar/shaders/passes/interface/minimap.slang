#language slang 2026

import globals;
import interface;
import transform;

[[vk::binding(0, 0)]] var global_uniforms: ConstantBuffer<GlobalUniforms>;
[[vk::binding(2, 0)]] var linear_sampler: SamplerState;
[[vk::binding(0, 1)]] var minimap_uniforms: ConstantBuffer<MinimapUniforms>;
[[vk::binding(1, 1)]] var minimap_texture: Texture2D;

struct MinimapUniforms {
    var tint: float4;
    var screen_position: float2;
    var screen_size: float2;
    var player_uv: float2;
    var map_size_tiles: float2;
    var visible_tiles: float;
    var rotation_radians: float;
};

struct MinimapVertexInput {
    var vertex_index: uint : SV_VulkanVertexID;
};

struct MinimapVertexOutput {
    var position: float4 : SV_Position;
    [[vk::location(0)]] var fragment_position: float2;
    [[vk::location(1)]] var disc_uv: float2;
};

[ForceInline]
func minimap_vertex_data(vertex_index: uint) -> float4 {
    let index = 1 << vertex_index;
    let x = float((index & 0x0E) != 0);
    let y = float((index & 0x1C) != 0);
    return float4(x, -y, x, y);
}

[[shader("vertex")]]
func vs_main(input: MinimapVertexInput) -> MinimapVertexOutput {
    let vertex = minimap_vertex_data(input.vertex_index);
    let clip_size = minimap_uniforms.screen_size * 2.0;
    let position = coordinate_space::screen_to_clip_space(minimap_uniforms.screen_position) + vertex.xy * clip_size;

    var output: MinimapVertexOutput;
    output.position = float4(position, 0.0, 1.0);
    output.fragment_position = coordinate_space::clip_to_screen_space(position);
    output.disc_uv = vertex.zw;
    return output;
}

[[shader("pixel")]]
func fs_main(input: MinimapVertexOutput) -> float4 {
    let centered = input.disc_uv - float2(0.5);
    let radius = length(centered);

    if (radius > 0.5) {
        return float4(0.0);
    }

    let angle = minimap_uniforms.rotation_radians;
    let rotation = float2x2(
        cos(angle), -sin(angle),
        sin(angle), cos(angle)
    );

    let visible_tiles = max(minimap_uniforms.visible_tiles, 1.0);
    let map_size_tiles = max(minimap_uniforms.map_size_tiles, float2(1.0));
    let tile_offset = mul(rotation, centered) * visible_tiles;
    let texture_uv = minimap_uniforms.player_uv + tile_offset / map_size_tiles;

    if (any(texture_uv < float2(0.0)) || any(texture_uv > float2(1.0))) {
        return float4(0.0);
    }

    let edge_alpha = saturate((0.5 - radius) / max(fwidth(radius), 0.0001));
    return minimap_texture.Sample(linear_sampler, texture_uv) * minimap_uniforms.tint * edge_alpha;
}
