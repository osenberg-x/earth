#import bevy_pbr::forward_io::VertexOutput

@group(2) @binding(0) var day_texture: texture_2d<f32>;
@group(2) @binding(1) var day_sampler: sampler;
@group(2) @binding(2) var night_texture: texture_2d<f32>;
@group(2) @binding(3) var night_sampler: sampler;
@group(2) @binding(4) var ocean_mask: texture_2d<f32>;
@group(2) @binding(5) var ocean_mask_sampler: sampler;
@group(2) @binding(6) var specular_map: texture_2d<f32>;
@group(2) @binding(7) var specular_map_sampler: sampler;
@group(2) @binding(8) var normal_map: texture_2d<f32>;
@group(2) @binding(9) var normal_map_sampler: sampler;
@group(2) @binding(10) 
