use bevy::{prelude::*};

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(camera_control)
        .run();
}

#[derive(Component)]
struct Camera;

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
        color: Color::BLUE,
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
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2.0, 2.0, 0.0).looking_at(Vec3::new(0.0,1.0,0.0), Vec3::Y),
        ..default()
    })
    .insert(Camera);
}

fn camera_control(
    input: Res<Input<KeyCode>>, 
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Camera>>){
        for mut transform in query.iter_mut(){
            let mut direction = Vec3::ZERO;
            if input.pressed(KeyCode::W){
                direction.x += 1.0;
            }
            if input.pressed(KeyCode::A){
                direction.z -= 1.0;
            }
            if input.pressed(KeyCode::S){
                direction.x -= 1.0;
            }
            if input.pressed(KeyCode::D){
                direction.z += 1.0;
            }
            transform.translation += time.delta_seconds() * 2.0 * direction;
        }
    }
