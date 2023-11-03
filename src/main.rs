// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::prelude::*;
use bevy_fpc::FpcBundle;
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(bevy_fpc::FpcPlugin)
        .add_systems(Startup, setup_lights)
        .add_systems(Startup, setup_physics_objects)
        .add_systems(Startup, setup_character)
        .run();
}

fn setup_lights(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    commands.insert_resource(AmbientLight {
        brightness: 0.2,
        ..Default::default()
    });
}

fn setup_physics_objects(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(100.0, 0.1, 100.0))
        .insert(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(100.0, 0.1, 100.0))),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            transform: Transform::from_xyz(0.0, -2.0, 0.0),
            ..default()
        });

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(0.5))
        .insert(Restitution::coefficient(0.7))
        .insert(PbrBundle {
            mesh: meshes.add(
                shape::UVSphere {
                    radius: 0.5,
                    ..default()
                }
                .into(),
            ),
            material: materials.add(Color::rgb(0.2, 0.1, 0.7).into()),
            transform: Transform::from_xyz(0.0, 4.0, 0.0),
            ..default()
        });
}

fn setup_character(mut commands: Commands) {
    commands
        .spawn(FpcBundle::default())
        .insert(bevy_fpc::Player)
        .insert(TransformBundle::from(Transform::from_xyz(0., 2.75, 0.)));
}
