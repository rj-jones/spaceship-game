use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub asteroid: Handle<Scene>,
    pub spaceship: Handle<Scene>,
    pub missle: Handle<Scene>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SceneAssets {
            asteroid: Handle::default(),
            spaceship: Handle::default(),
            missle: Handle::default(),
        })
        .add_systems(Startup, load_assets);
    }
}

fn load_assets(mut scene_assets: ResMut<SceneAssets>, asset_server: Res<AssetServer>) {
    scene_assets.asteroid = asset_server.load("Asteroid.glb#Scene0");
    scene_assets.spaceship = asset_server.load("Spaceship.glb#Scene0");
    scene_assets.missle = asset_server.load("Bullets Pickup.glb#Scene0");
}
