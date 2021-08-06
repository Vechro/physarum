use bevy::math::Vec2;

use super::angle::Angle;

pub fn produce_in_direction(v: &Vec2, theta: Angle, d: f32) -> Vec2 {
    Vec2::new(
        v.x + d * f32::cos(theta.to_degrees()),
        v.y + d * f32::sin(theta.to_degrees()),
    )
}
