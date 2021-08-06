#![allow(dead_code)]

use math::{angle::Angle, percent::Percent};

pub mod math {
    pub mod angle;
    pub mod percent;
    pub mod vec2x;
}

pub mod layers {
    pub mod trail_map;
}

pub mod agent;
pub mod blur;
pub mod cell;

// Canvas dimensions
pub const WIDTH: u16 = 180;
pub const HEIGHT: u16 = 180;

pub const MIN_X: i16 = WIDTH as i16 / -2;
pub const MAX_X: i16 = WIDTH as i16 / 2;
pub const MIN_Y: i16 = HEIGHT as i16 / -2;
pub const MAX_Y: i16 = HEIGHT as i16 / 2;

const TIME_STEP: f32 = 1.0 / 60.0;

/// Vertical and horizontal dimension of a cell in pixels
const CELL_SIZE: u16 = 4;

/// Population as percentage of image area
const P: Percent = Percent(15.0);
/// Diffusion kernel size
const DIFF_K: u8 = 3;
/// Pre-pattern stimuli projection weight
const W_PROJ: f32 = 0.1;
/// Front-left and front-right sensor angle from forward position
const SA: Angle = Angle::Degrees(22.5);
/// Agent rotation angle
const RA: Angle = Angle::Degrees(45.0);
/// Sensor offset distance
const SO: f32 = 9.0;
/// Sensor width
const SW: u8 = 1;
/// Step size — how far agent moves per step
const SS: f32 = 1.0;
/// Chemoattractant deposition per step
const DEP_T: u8 = 5;
/// Probability of a random change in direction
const P_CD: f32 = 0.0;
/// Sensitivity threshold
const S_MIN: f32 = 0.0;
