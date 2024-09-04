use bevy::prelude::*;
use hyper::server::conn::http1;
use hyper_util::rt::TokioIo;
use renderer::service::{GyroService, GyroState};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let state = GyroState::default();
    let listener = TcpListener::bind("0.0.0.0:7878")
        .await
        .expect("Failed to bind to TcpListener on port 7878");

    let state_clone = state.clone();
    tokio::spawn(async move {
        loop {
            let (socket, _) = listener
                .accept()
                .await
                .expect("Failed to accept incoming connection");
            let io = TokioIo::new(socket);
            let service = GyroService::new(state_clone.clone());

            tokio::spawn(async move {
                if let Err(e) = http1::Builder::new()
                    .serve_connection(io, service)
                    .with_upgrades()
                    .await
                {
                    eprintln!("Error serving connection: {}", e);
                }
            });
        }
    });

    App::new()
        .insert_resource(state.clone())
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, rotate)
        .run();
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
            mesh: meshes.add(Sphere::default()),
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

fn rotate(mut cubes: Query<(&mut Transform, &MyCube)>, res: Res<GyroState>) {
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
