use std::f32::consts::TAU;

use rand::{Rng, distributions::Standard, prelude::Distribution};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Angle {
    Degrees(f32),
    Radians(f32),
}

impl Angle {
    pub fn to_degrees(&self) -> f32 {
        match self {
            Angle::Degrees(n) => *n,
            Angle::Radians(n) => n.to_degrees(),
        }
    }

    pub fn to_radians(&self) -> f32 {
        match self {
            Angle::Degrees(n) => n.to_radians(),
            Angle::Radians(n) => *n,
        }
    }
}

impl std::ops::Add for Angle {
    type Output = Angle;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Angle::Degrees(deg) => Angle::Degrees(deg + rhs.to_degrees()),
            Angle::Radians(rad) => Angle::Radians(rad + rhs.to_radians()),
        }
    }
}

impl std::ops::Sub for Angle {
    type Output = Angle;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Angle::Degrees(deg) => Angle::Degrees(deg - rhs.to_degrees()),
            Angle::Radians(rad) => Angle::Radians(rad - rhs.to_radians()),
        }
    }
}

impl Distribution<Angle> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Angle {
        Angle::Radians(rng.gen_range(0f32..TAU))
    }
}