#![allow(dead_code)]
use std::ops::Div;

use bevy::math::Vec2;
use lazy_static::lazy_static;
use math::angle::Angle;

pub mod math {
    pub mod angle;
    pub mod percent;
    pub mod vec2ext;
}

pub mod agent;
pub mod blur;
pub mod board;
pub mod cell;
pub mod timestep;
pub mod triplet;

lazy_static! {
    pub static ref DIMENSIONS: Vec2 = Vec2::new(160.0, 160.0);
    pub static ref MIN: Vec2 = DIMENSIONS.div(-2.0);
    pub static ref MAX: Vec2 = DIMENSIONS.div(2.0);
    pub static ref AGENT_COUNT: u32 = (DIMENSIONS.x * DIMENSIONS.y * P) as u32;
    /// Vertical and horizontal dimension of a cell in pixels
    pub static ref CELL_SIZE: Vec2 = Vec2::splat(1.0);
}

/// Population as percentage of image area
const P: f32 = 0.0025;
/// Diffusion kernel size
const DIFF_K: u8 = 3;
/// Trail-map chemoattractant diffusion decay factor
const DECAY_T: f32 = 0.1;
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
