
use std::sync::Arc;

use bevy::prelude::*;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let state = State::default();

    let mut state_clone = state.clone();
    tokio::spawn(async move {
        loop {
            for x in -180..180 {
                for y in -180..180 {
                    state_clone.set_pitch_poll(x as f32, y as f32).await
                }
            }
        }
    });

    App::new()
        .insert_resource(state.clone())
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, rotate)
        .run();
}

#[derive(Resource, Default, Clone)]
pub struct State {
    rotation: Arc<RwLock<(f32, f32)>>
}

impl State {
    pub async fn set_pitch_poll(&mut self, pitch: f32, poll: f32) {
        *self.rotation.write().await = (pitch, poll)
    }

    pub async fn get_pitch_poll(&self) -> (f32, f32) {
        *self.rotation.read().await
    }
}

#[derive(Component)]
struct MyCube;

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
        MyCube,
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

fn rotate(mut cubes: Query<(&mut Transform, &MyCube)>, res: Res<State>) {
    let (pitch, poll) = futures::executor::block_on(res.get_pitch_poll());
    for (mut transform, _) in &mut cubes {
        *transform = Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            pitch.to_radians(),
            poll.to_radians(),
            0f32.to_radians(),
        ));
    }
}
