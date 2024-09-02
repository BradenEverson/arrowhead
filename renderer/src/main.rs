use bevy::prelude::*;

#[derive(Component)]
struct RotatableObject {
    pitch: f32,
    poll: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, rotate)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::default()),
            material: materials.add(Color::WHITE),
            transform: Transform::from_translation(Vec3::ZERO),
            ..default()
        },
        RotatableObject {
            pitch: 45f32,
            poll: 45f32,
        },
    ));

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 10.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(3.0, 3.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn rotate(mut cubes: Query<(&mut Transform, &RotatableObject)>) {
    for (mut transform, cube) in &mut cubes {
        *transform = Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            (cube.pitch).to_radians(),
            (cube.poll).to_radians(),
            (0f32).to_radians(),
        ));
    }
}
