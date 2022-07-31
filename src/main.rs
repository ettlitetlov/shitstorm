use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

pub const BACKGROUND: Color = Color::rgb(0.75, 0.90, 1.0);

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(BACKGROUND))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(bevy::window::close_on_esc)
        .add_system(camera_control)
        .add_system(mouse_motion)
        .run();
}

#[derive(Component)]
struct Camera {
    velocity: Vec3,
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 15.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });

    // ambient light
    commands.insert_resource(AmbientLight {
        color: Color::rgb(0.0, 0.90, 1.0),
        brightness: 0.05,
    });

    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1450.0,
            color: Color::ORANGE_RED,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // camera
    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.0, 0.0)
                .looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
            ..default()
        })
        .insert(Camera {
            velocity: Vec3::ZERO,
        });
}

fn mouse_motion(
    mut motion_evr: EventReader<MouseMotion>,
    mut windows: ResMut<Windows>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    let mut camera = query.single_mut();
    let window = windows.get_primary_mut().unwrap();

    // Hide mouse cursor
    window.set_cursor_visibility(false);
    
    window.set_cursor_lock_mode(true);

    for ev in motion_evr.iter() {
        let yaw = (ev.delta.x * (-0.2)).to_radians();
        let pitch = (ev.delta.y * (-0.2)).to_radians();

        let rot = camera.rotation;
        camera.rotation =
            Quat::from_axis_angle(Vec3::Y, yaw) * rot * Quat::from_axis_angle(Vec3::X, pitch);
    }
}

fn camera_control(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Camera), With<Camera>>,
) {
    for (mut transform, mut camera) in query.iter_mut() {
        let mut velocity = Vec3::ZERO;
        let local_z = transform.local_z();
        let forward = -Vec3::new(local_z.x, 0., local_z.z);
        let right = Vec3::new(local_z.z, 0., -local_z.x);

        for key in input.get_pressed() {
            match key {
                KeyCode::W => velocity += forward,
                KeyCode::S => velocity -= forward,
                KeyCode::A => velocity -= right,
                KeyCode::D => velocity += right,
                KeyCode::Space => {
                    if transform.translation.y <= 2.0 {
                        camera.velocity += Vec3::Y * 150.0;
                    }
                }
                _ => (),
            }
        }

        // If we are above ground
        if transform.translation.y > 2.0 {
            camera.velocity -= Vec3::Y * (9.80 * (time.delta_seconds() / time.delta_seconds()));
        }

        velocity = velocity + camera.velocity * time.delta_seconds();
        transform.translation += velocity * time.delta_seconds() * 5.0;

        // Reset velocity if the ground is reached
        if transform.translation.y <= 2.0 {
            camera.velocity = Vec3::ZERO;
        }
    }
}

fn is_equal(lhs: f32, rhs: f32) -> bool {
    return (lhs - rhs).abs() < 0.01;
}
