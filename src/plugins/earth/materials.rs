use bevy::{
    prelude::*,
    render::render_resource::*,
    reflect::TypePath,
    asset::Asset,
};

#[derive(ShaderType, Clone, Copy, Debug)]
#[repr(C)]
pub struct SunUniform {
    pub direction: Vec3,
    pub _padding: f32,
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct EarthMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub day_texture: Handle<Image>,
    #[texture(2)]
    #[sampler(3)]
    pub night_texture: Handle<Image>,
    #[texture(4)]
    #[sampler(5)]
    pub ocean_mask: Handle<Image>,
    #[texture(6)]
    #[sampler(7)]
    pub specular_map: Handle<Image>,
    #[texture(8)]
    #[sampler(9)]
    pub normal_map: Handle<Image>,
    #[uniform(10)]
    pub sun_uniform: SunUniform,
}

impl Material for EarthMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/earth.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Opaque
    }
}