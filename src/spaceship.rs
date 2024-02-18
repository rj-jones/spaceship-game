use bevy::prelude::*;

use crate::{
    asset_loader::SceneAssets,
    collision::Collider,
    movement::{Acceleration, MovingObjectBundle, Velocity},
};

const MISSILE_FORWARD_SPAWN_DISTANCE: f32 = 7.5;
const MISSILE_RADIUS: f32 = 1.0;
const MISSILE_SPEED: f32 = 50.0;
const SPACESHIP_RADIUS: f32 = 5.0;
const SPACESHIP_ROLL_SPEED: f32 = 2.5;
const SPACESHIP_ROTATION_SPEED: f32 = 2.5;
const SPACESHIP_SPEED: f32 = 25.0;
const STARTING_TRANSLATION: Vec3 = Vec3::new(0., 0., -20.);

#[derive(Component, Debug)]
pub struct Spaceship;

#[derive(Component, Debug)]
pub struct SpaceshipMissle;

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_spaceship).add_systems(
            Update,
            (spaceship_movement_controls, spaceship_weapon_controls),
        );
    }
}

fn spawn_spaceship(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity::new(Vec3::ZERO),
            acceleration: Acceleration::new(Vec3::ZERO),
            collider: Collider::new(SPACESHIP_RADIUS),
            model: SceneBundle {
                scene: scene_assets.spaceship.clone(),
                transform: Transform::from_translation(STARTING_TRANSLATION),
                ..default()
            },
        },
        Spaceship,
    ));
}

fn spaceship_movement_controls(
    mut query: Query<(&mut Transform, &mut Velocity), With<Spaceship>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut transform, mut velocity) = query.single_mut();
    let mut rotation = 0.0;
    let mut roll = 0.0;
    let mut movement = 0.0;

    // forwards backwords
    if keyboard_input.pressed(KeyCode::W) {
        movement = SPACESHIP_SPEED;
    } else if keyboard_input.pressed(KeyCode::S) {
        movement = -SPACESHIP_SPEED;
    }

    // left right
    if keyboard_input.pressed(KeyCode::A) {
        rotation = SPACESHIP_ROTATION_SPEED * time.delta_seconds();
    } else if keyboard_input.pressed(KeyCode::D) {
        rotation = -SPACESHIP_ROTATION_SPEED * time.delta_seconds();
    }

    // roll
    if keyboard_input.pressed(KeyCode::Q) {
        roll = -SPACESHIP_ROLL_SPEED * time.delta_seconds();
    } else if keyboard_input.pressed(KeyCode::E) {
        roll = SPACESHIP_ROLL_SPEED * time.delta_seconds();
    }

    transform.rotate_y(rotation);
    transform.rotate_local_z(roll);

    // Update the spaceship's velocity based on new direction. Note that we are
    // flipping the direction of the velocity because the spaceship is facing
    // the negative z-axis. This could be fixed using Blender to change the
    // forward direction of the spaceship model.
    velocity.value = -transform.forward() * movement;
}

fn spaceship_weapon_controls(
    mut commands: Commands,
    query: Query<&Transform, With<Spaceship>>,
    keyboard_input: Res<Input<KeyCode>>,
    scene_assets: Res<SceneAssets>,
) {
    let transform = query.single();

    if keyboard_input.pressed(KeyCode::Space) {
        commands.spawn((
            MovingObjectBundle {
                velocity: Velocity::new(-transform.forward() * MISSILE_SPEED),
                acceleration: Acceleration::new(Vec3::ZERO),
                collider: Collider::new(MISSILE_RADIUS),
                model: SceneBundle {
                    scene: scene_assets.missle.clone(),
                    transform: Transform::from_translation(
                        transform.translation
                            + -transform.forward() * MISSILE_FORWARD_SPAWN_DISTANCE,
                    ),
                    ..default()
                },
            },
            SpaceshipMissle,
        ));
    }
}
