use defmt::Format;

use crate::{
    math::sqrt,
    types::{
        angle::{cos, sin, Angle},
        vector::Vec3,
    },
};

#[derive(Clone, Copy, Format)]
/// Represents a quaternion.
pub struct Quaternion {
    /// real part
    pub a: f32,
    // imag x
    pub i: f32,
    // imag y
    pub j: f32,
    // imag z
    pub k: f32,
}

impl Quaternion {
    pub fn of(axis: Vec3<f32>, angle: Angle) -> Quaternion {
        let h_sin_a = sin(angle.rad / 2.0);
        Quaternion {
            i: h_sin_a * axis.x,
            j: h_sin_a * axis.y,
            k: h_sin_a * axis.z,
            a: cos(angle.rad / 2.0),
        }
    }

    /// Normalises the quaternion to length 1.
    pub fn nor(&mut self) -> &Self {
        let len = self.len();
        self.i /= len;
        self.j /= len;
        self.k /= len;
        self.a /= len;
        self
    }

    pub fn len(&self) -> f32 {
        sqrt((self.i * self.i + self.j * self.j + self.k * self.k + self.a * self.a) as f32)
    }
}
