use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct MercuryPlugin;

impl Plugin for MercuryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, control_vehicle);
    }
}

#[derive(Component)]
pub struct VehicleKinematics {
    pub target_waypoint: Vec3,
    pub max_speed: f32,
}

#[derive(Component)]
pub struct SteeringController {
    pub p: f32,
    pub i: f32,
    pub d: f32,
    pub prev_error: f32,
    pub integral: f32,
}

impl Default for SteeringController {
    fn default() -> Self {
        Self {
            p: 2.0,
            i: 0.0,
            d: 0.5,
            prev_error: 0.0,
            integral: 0.0,
        }
    }
}

fn control_vehicle(
    time: Res<Time>,
    mut query: Query<(
        &Transform,
        &VehicleKinematics,
        &mut SteeringController,
        &mut Velocity,
    )>,
) {
    let dt = time.delta_secs();
    
    for (transform, kinematics, mut pid, mut velocity) in query.iter_mut() {
        let current_pos = transform.translation;
        let target_pos = kinematics.target_waypoint;
        
        let direction_to_target = target_pos - current_pos;
        
        if direction_to_target.length() < 0.5 {
            velocity.linear = Vec3::ZERO;
            velocity.angular = Vec3::ZERO;
            continue;
        }

        let direction_to_target = direction_to_target.normalize();
        
        let forward: Vec3 = (*transform.forward()).into(); 
        
        let forward_2d = Vec2::new(forward.x, forward.z).normalize_or_zero();
        let target_2d = Vec2::new(direction_to_target.x, direction_to_target.z).normalize_or_zero();
        
        if forward_2d.length() == 0.0 || target_2d.length() == 0.0 {
            continue;
        }
        
        let angle_forward = forward_2d.y.atan2(forward_2d.x);
        let angle_target = target_2d.y.atan2(target_2d.x);
        let mut error = angle_target - angle_forward;
        
        while error > std::f32::consts::PI { error -= 2.0 * std::f32::consts::PI; }
        while error < -std::f32::consts::PI { error += 2.0 * std::f32::consts::PI; }
        
        pid.integral += error * dt;
        let derivative = if dt > 0.0 { (error - pid.prev_error) / dt } else { 0.0 };
        pid.prev_error = error;
        
        let steering_output = pid.p * error + pid.i * pid.integral + pid.d * derivative;
        
        velocity.angular = Vec3::new(0.0, -steering_output, 0.0);
        velocity.linear = forward * kinematics.max_speed;
    }
}
