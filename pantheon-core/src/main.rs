use bevy::app::ScheduleRunnerPlugin;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use mercury_auto::{MercuryPlugin, SteeringController, VehicleKinematics};
use std::time::Duration;

fn main() {
    App::new()
        // MinimalPlugins is perfect for headless mode
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(
            Duration::from_secs_f64(1.0 / 60.0),
        )))
        .add_plugins(TransformPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(MercuryPlugin)
        .add_systems(Startup, setup_scene)
        .add_systems(Update, print_car_position)
        .run();
}

fn setup_scene(mut commands: Commands) {
    // Ground
    commands.spawn((
        Transform::from_xyz(0.0, -0.1, 0.0),
        Collider::cuboid(50.0, 0.1, 50.0),
    ));

    // Vehicle
    let waypoint_pos = Vec3::new(20.0, 0.5, -20.0);
    commands.spawn((
        Transform::from_xyz(0.0, 0.5, 0.0),
        RigidBody::KinematicVelocityBased,
        Collider::cuboid(1.0, 0.5, 2.0),
        Velocity::default(),
        VehicleKinematics {
            target_waypoint: waypoint_pos,
            max_speed: 10.0,
        },
        SteeringController::default(),
    ));
}

fn print_car_position(
    query: Query<(&Transform, &VehicleKinematics)>,
    mut frames: Local<u32>,
) {
    *frames += 1;
    // Print every 60 frames (1 second)
    if *frames % 60 == 0 {
        for (transform, kinematics) in query.iter() {
            let pos = transform.translation;
            let dist = (kinematics.target_waypoint - pos).length();
            println!(
                "Car position: [{:.2}, {:.2}, {:.2}] | Distance to waypoint: {:.2}",
                pos.x, pos.y, pos.z, dist
            );
            
            if dist < 0.5 {
                println!("Waypoint reached! Simulation complete.");
                std::process::exit(0);
            }
        }
    }
}
