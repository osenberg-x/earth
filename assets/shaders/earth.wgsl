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
@group(2) @binding(10)  var<uniform> sun_uniform: SunUniform;

struct SunUniform {
    sun_uniform: vec3<f32>,
    // 16-byte alignment
    _padding: f32,
}

const PI: f32 = 3.14159265;

// desaturate a color
fn desaturate(color: vec3<f32>, factor: f32) -> vec3<f32> {
    let gray = dot(color, vec3<f32>(0.299, 0.587, 0.114));
    return mix(color, vec3<f32>(gray), factor);
}

// calculate tangent space vectors for spherical surfaces
fn calculate_sphere_tangent_space(world_pos: vec3<f32>, uv: vec2<f32>) -> mat3x3<f32> {
    // normalize position to get point on unit sphere
    let point_on_sphere = normalize(world_pos);

    let longitude = (uv.x * 2.0 - 1.0) * PI;
    let latitude = (0.5 - uv.y) * PI;

    // calculate tangent
    // direction of increasing longitude
    let tangent = vec3<f32>(
        -sin(longitude),
        0.0,
        cos(longitude),
    );

    // calculate bitangent
    // direction of increasing latitude
    let bitangent = vec3<f32>(
        -cos(longitude) * sin(latitude),
        cos(latitude),
        -sin(longitude) * cos(latitude)
    );

    let normal = point_on_sphere;

    return mat3x3<f32>(
        normalize(tangent),
        normalize(bitangent),
        normal,
    );
}

// sample and decode normal
fn sample_normal_map_shpere(uv: vec2<f32>, world_pos: vec3<f32>) -> vec3<f32> {
    // get the mesh normal
    let mesh_normal = normalize(world_pos);

    // sample normal map
    let normal_sample = textureSample(normal_map, normal_map_sampler, uv).rgb;
}
