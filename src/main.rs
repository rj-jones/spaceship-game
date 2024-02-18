mod asset_loader;
mod asteroids;
mod camera;
mod collision;
mod debug;
mod despawn;
mod movement;
mod spaceship;

use asset_loader::AssetLoaderPlugin;
use asteroids::AsteroidsPlugin;
use bevy::{
    prelude::*,
    render::{
        settings::{Backends, WgpuSettings},
        RenderPlugin,
    },
};
use camera::CameraPlugin;
use collision::CollisionDetectionPlugin;
use debug::DebugPlugin;
use despawn::DespawnPlugin;
use movement::MovementPlugin;
use spaceship::SpaceshipPlugin;

#[derive(Component, Debug)]
pub struct Velocity(Vec3);

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.75,
        })
        .add_plugins((
            CameraPlugin,
            SpaceshipPlugin,
            MovementPlugin,
            DebugPlugin,
            AsteroidsPlugin,
            AssetLoaderPlugin,
            CollisionDetectionPlugin,
            DespawnPlugin,
        ))
        .add_plugins(
            DefaultPlugins.set(RenderPlugin {
                render_creation: WgpuSettings {
                    backends: Some(Backends::VULKAN),
                    ..default()
                }
                .into(),
            }),
        )
        .run();
}
