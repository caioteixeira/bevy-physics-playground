// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use std::f32::consts::PI;

use bevy::{prelude::*, render::view::screenshot::ScreenshotManager, window::PrimaryWindow};
use bevy_flycam::prelude::*;
use bevy_rapier3d::prelude::*;
use rfd::FileDialog;

#[derive(Resource)]
struct SharedAssets {
    ball_mesh: Handle<Mesh>,
    ball_material: Handle<StandardMaterial>,
    cube_mesh: Handle<Mesh>,
    cube_material: Handle<StandardMaterial>,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
            NoCameraPlayerPlugin,
        ))
        .add_systems(PreStartup, setup_shared_assets)
        .add_systems(
            Startup,
            (setup_lights, setup_physics_objects, setup_camera, setup_ui),
        )
        .add_systems(Update, (launch_ball, save_screenshot))
        .run();
}

fn setup_shared_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(SharedAssets {
        ball_mesh: meshes.add(
            shape::UVSphere {
                radius: 0.5,
                ..default()
            }
            .into(),
        ),
        ball_material: materials.add(Color::rgb(0.2, 0.1, 0.7).into()),
        cube_mesh: meshes.add(Mesh::from(shape::Box::new(1.0, 1.0, 1.0))),
        cube_material: materials.add(Color::rgb(0.1, 0.1, 0.1).into()),
    });
}

fn setup_lights(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        brightness: 0.3,
        ..Default::default()
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 5.),
            ..default()
        },
        ..default()
    });
}

fn setup_physics_objects(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    shared_assets: Res<SharedAssets>,
) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(50.0, 0.1, 50.0))
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
            mesh: shared_assets.ball_mesh.clone(),
            material: shared_assets.ball_material.clone(),
            transform: Transform::from_xyz(0.0, 100.0, 0.0),
            ..default()
        });

    /* Create a wall of cubes */
    for x in 0..10 {
        for y in 0..5 {
            for z in 0..3 {
                commands
                    .spawn(RigidBody::Dynamic)
                    .insert(Collider::cuboid(0.5, 0.5, 0.5))
                    .insert(PbrBundle {
                        mesh: shared_assets.cube_mesh.clone(),
                        material: shared_assets.cube_material.clone(),
                        transform: Transform::from_xyz(x as f32, 0.1 + y as f32, z as f32),
                        ..default()
                    });
            }
        }
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(10.0, 2.0, 10.0),
            ..default()
        },
        FlyCam,
    ));
}

fn setup_ui(mut commands: Commands) {
    commands.spawn(
        TextBundle::from_section(
            "WASD : Move\n\
            Space: Ascend\n\
            Left Mouse Button: Shoot project\n\
            Left Shift: Descend\n\
            Esc: Grab/release cursor\n\
            P: Take screenshot",
            TextStyle {
                font_size: 20.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_text_alignment(TextAlignment::Left)
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            left: Val::Px(15.0),
            ..default()
        }),
    );
}

fn save_screenshot(
    input: Res<Input<KeyCode>>,
    main_window: Query<Entity, With<PrimaryWindow>>,
    mut screenshot_manager: ResMut<ScreenshotManager>,
) {
    if input.just_pressed(KeyCode::P) {
        let path = FileDialog::new()
            .add_filter("PNG Image", &["png"])
            .set_file_name("screenshot.png")
            .save_file();

        match path {
            Some(target_path) => {
                info!("Saving screenshot to {}", target_path.to_str().unwrap());
                screenshot_manager
                    .save_screenshot_to_disk(main_window.single(), target_path)
                    .unwrap();
            }
            None => info!("Screenshot cancelled"),
        };
    }
}

fn launch_ball(
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
    camera_query: Query<&Transform, With<Camera>>,
    shared_assets: Res<SharedAssets>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let camera_transform = camera_query.single();
        let ball_position = camera_transform.translation + camera_transform.forward() * 2.0;

        commands
            .spawn(RigidBody::Dynamic)
            .insert(Collider::ball(0.5))
            .insert(Restitution::coefficient(0.7))
            .insert(ExternalImpulse {
                impulse: camera_transform.forward() * 50.0,
                torque_impulse: Vec3::new(0.1, 0.1, 0.1),
            })
            .insert(PbrBundle {
                mesh: shared_assets.ball_mesh.clone(),
                material: shared_assets.ball_material.clone(),
                transform: Transform::from_translation(ball_position),
                ..default()
            });
    }
}
