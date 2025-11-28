use bevy::prelude::*;

pub mod materials;
pub mod mesh;
pub mod normal;
pub mod uv;

use crate::{Sun, config::*};
use materials::{EarthMaterial, SunUniform};
use mesh::generate_face;
use normal::{generate_normal_map, save_image_as_png};

pub struct EarthPlugin;

impl Plugin for EarthPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<EarthMaterial>::default())
            .add_systems(Startup, setup);
    }
}

/// Earth tag
#[derive(Component)]
pub struct Earth;

/// holds everything needed for earth generation including normal map
#[derive(Resource)]
struct EarthData {
    displacement_handle: Handle<Image>,
    normal_map_handle: Option<Handle<Image>>,  // generated normal map
    earth_entity: Entity,
    earth_material: Option<Handle<EarthMaterial>>,  // created after normal map generation
}
