use std::f32::consts::TAU;

use bevy::{math::Vec2, prelude::*};
use rand::Rng;

use crate::{
    cell::Cell,
    layers::trail_map::TrailMap,
    math::{angle::Angle, vec2x::produce_in_direction},
    DEP_T, HEIGHT, RA, SA, SO, SS, SW, WIDTH,
};

#[derive(Debug, Clone, Copy)]
pub struct Agent {
    pub heading: Angle,
    pub pos: Vec2,
    /// Angle between straight-ahead sensor and side-sensor
    pub sensor_angle: Angle,
    pub sensor_offset_distance: f32,
    pub sensor_width: u8,
    pub rotation_angle: Angle,
    pub step_size: f32,
    pub deposition_size: u8,
}

impl Default for Agent {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        Agent {
            heading: Angle::Radians(rng.gen_range(0.0..TAU)),
            pos: Vec2::new(
                rng.gen_range(0.0..WIDTH as f32),
                rng.gen_range(0.0..HEIGHT as f32),
            ),
            sensor_angle: SA,
            sensor_offset_distance: SO,
            sensor_width: SW,
            rotation_angle: RA,
            step_size: SS,
            deposition_size: DEP_T,
        }
    }
}

enum Direction {
    Left,
    Middle,
    Right,
}

struct SensorDirections<'a> {
    pub left: Option<&'a Cell>,
    pub middle: Option<&'a Cell>,
    pub right: Option<&'a Cell>,
}

impl<'a> SensorDirections<'a> {
    pub fn max(&self) -> (bool, bool, bool) {
        match self.values() {
            (l, m, r) if l >= m && l >= r => (true, false, false),
            (l, m, r) if m >= l && m >= r => (false, true, false),
            (l, m, r) if r >= m && r >= l => (false, false, true),
            _ => (false, false, false),
        }
    }

    pub fn values(&self) -> (u8, u8, u8) {
        let (l, m, r) = (self.left, self.middle, self.right);
        (
            l.unwrap_or(&Cell::default()).value,
            m.unwrap_or(&Cell::default()).value,
            r.unwrap_or(&Cell::default()).value,
        )
    }
}

impl Agent {
    /// Detect values using this Agent's sensors.
    fn sense_and_move(mut q: Query<&mut Agent>, trail_map: ResMut<TrailMap>) {
        q.iter_mut().for_each(|mut agent| {
            let pos = &agent.pos;
            let d = agent.sensor_offset_distance;

            let (left_heading, mid_heading, right_heading) = (
                agent.heading - agent.sensor_angle,
                agent.heading,
                agent.heading + agent.sensor_angle,
            );

            let left = produce_in_direction(pos, left_heading, d);
            let mid = produce_in_direction(pos, mid_heading, d);
            let right = produce_in_direction(pos, right_heading, d);

            let directions = SensorDirections {
                left: trail_map.find_by_coords(left.x as i32, left.y as i32),
                middle: trail_map.find_by_coords(mid.x as i32, mid.y as i32),
                right: trail_map.find_by_coords(right.x as i32, right.y as i32),
            };

            let new_heading = match directions.max() {
                (true, false, false) => left_heading,
                (false, true, false) => mid_heading,
                (false, false, true) => right_heading,
                _ => left_heading,
            };

            agent.pos = produce_in_direction(&agent.pos, new_heading, agent.step_size)
        });
    }
}
