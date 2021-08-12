use bevy::math::Vec2;

use crate::math::angle::Angle;

pub trait Vec2Ext {
    fn produce_in_direction(self, theta: Angle, d: f32) -> Vec2;
    fn is_in_area(self, dimensions: &Vec2) -> bool;
}

impl Vec2Ext for Vec2 {
    fn produce_in_direction(self, theta: Angle, d: f32) -> Vec2 {
        Vec2::new(
            self.x + d * f32::cos(theta.to_degrees()),
            self.y + d * f32::sin(theta.to_degrees()),
        )
    }

    fn is_in_area(self, dimensions: &Vec2) -> bool {
        self.abs().cmple(dimensions.abs()).all()
    }
}