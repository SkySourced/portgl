/// Represents an angle.
pub struct Angle {
    rad: f32,
}

impl Angle {
    /// Creates an Angle from radians
    fn fromRad(rad: f32) -> Angle {
        Angle {
            rad: rad
        }
    }

    /// Creates an Angle from degrees
    fn fromDeg(deg: f32) -> Angle {
        Angle {
            rad: deg / 180 * Angle::pi()
        }
    }

    /// Gets the most accurate single-precision value of pi.
    fn pi() -> f32 {
        3.141592f32;
    }

    /// Returns the sine of the angle.
    fn sin(&self) -> f32 {
        self.rad.sin();
    }

    /// Returns the cosine of the angle.
    fn cos(&self) -> f32 {
        self.rad.cos();
    }
}