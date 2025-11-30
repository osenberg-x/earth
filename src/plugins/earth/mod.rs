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
    normal_map_handle: Option<Handle<Image>>, // generated normal map
    earth_entity: Entity,
    earth_material: Option<Handle<EarthMaterial>>, // created after normal map generation
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, asset_server: Res<AssetServer>) {
    // sun direction
    let sun_direction = Vec3::new(1.0, 1.0, 1.0).normalize();

    // load textures
    let displacement_handle = asset_server.load(EARTH_DISPLACEMENT_TEXTURE);

    // create earth entity
    let earth_entity = commands
        .spawn((Earth, Transform::default(), GlobalTransform::default()))
        .id();

    commands.insert_resource(EarthData {
        displacement_handle,
        normal_map_handle: None,
        earth_entity,
        earth_material: None,
    });
}
