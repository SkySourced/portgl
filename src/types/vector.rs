use super::super::math::sqrt;
use core::ops::{Add, AddAssign, Mul, MulAssign};
use defmt::Format;

pub const VEC2_ZERO: Vec2<f32> = Vec2 {x:0.0, y:0.0};
pub const VEC2_X: Vec2<f32> = Vec2 {x:1.0, y:0.0};
pub const VEC2_Y: Vec2<f32> = Vec2 {x:0.0, y:1.0};

pub const VEC3_ZERO: Vec3<f32> = Vec3 {x:0.0, y:0.0, z: 0.0};
pub const VEC3_X: Vec3<f32> = Vec3 {x:1.0, y:0.0, z: 0.0};
pub const VEC3_Y: Vec3<f32> = Vec3 {x:0.0, y:1.0, z: 0.0};
pub const VEC3_Z: Vec3<f32> = Vec3 {x:0.0, y:0.0, z: 1.0};

pub const VEC4_ZERO: Vec4<f32> = Vec4 {x:0.0, y:0.0, z: 0.0, w: 0.0};
pub const VEC4_X: Vec4<f32> = Vec4 {x:1.0, y:0.0, z: 0.0, w: 0.0};
pub const VEC4_Y: Vec4<f32> = Vec4 {x:0.0, y:1.0, z: 0.0, w: 0.0};
pub const VEC4_Z: Vec4<f32> = Vec4 {x:0.0, y:0.0, z: 1.0, w: 0.0};
pub const VEC4_W: Vec4<f32> = Vec4 {x:0.0, y:0.0, z: 0.0, w: 1.0};

#[derive(Debug, Copy, Clone, Format)]
/// Represents a 2D vector.
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl Add<Vec2<f32>> for Vec2<f32> {
    type Output = Vec2<f32>;
    fn add(self, rhs: Vec2<f32>) -> Self::Output {
        Vec2::<f32> {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<Vec2<f32>> for Vec2<f32> {
    fn add_assign(&mut self, rhs: Vec2<f32>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Mul<f32> for Vec2<f32> {
    type Output = Vec2<f32>;
    fn mul(self, rhs: f32) -> Self::Output {
        Vec2::<f32> {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl MulAssign<f32> for Vec2<f32> {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl PartialEq for Vec2<f32> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Vec2<f32> {
    /// Computes the dot/inner/scalar product of the two vectors provided.
    pub fn dot(v1: Vec2<f32>, v2: Vec2<f32>) -> f32 {
        v1.x * v2.x + v1.y * v2.y
    }

    /// Calculates the length of this vector.
    pub fn len(&self) -> f32 {
        sqrt((self.x * self.x + self.y * self.y) as f32)
    }

    /// Normalises this vector, and returns it for chaining.
    pub fn nor(&mut self) -> &Self {
        let len = self.len();
        self.x /= len;
        self.y /= len;
        self
    }
}

#[derive(Copy, Clone, Format)]
/// Represents a 3D vector.
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl Add<Vec3<f32>> for Vec3<f32> {
    type Output = Vec3<f32>;
    fn add(self, rhs: Vec3<f32>) -> Self::Output {
        Vec3::<f32> {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign<Vec3<f32>> for Vec3<f32> {
    fn add_assign(&mut self, rhs: Vec3<f32>) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Mul<f32> for Vec3<f32> {
    type Output = Vec3<f32>;
    fn mul(self, rhs: f32) -> Self::Output {
        Vec3::<f32> {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl MulAssign<f32> for Vec3<f32> {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl PartialEq for Vec3<f32> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Vec3<f32> {
    /// Computes the dot/inner/scalar product of the two vectors provided.
    pub fn dot(v1: Vec3<f32>, v2: Vec3<f32>) -> f32 {
        v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
    }

    /// Computes the cross/vector product of the two vectors provided.
    pub fn cross(v1: Vec3<f32>, v2: Vec3<f32>) -> Vec3<f32> {
        Vec3::<f32> {
            x: v1.y * v2.z - v1.z * v2.y,
            y: v1.x * v2.z - v1.z * v2.x,
            z: v1.x * v2.y - v1.y * v2.x,
        }
    }

    /// Creates a Vec3 out of a Vec2 and a third component.
    pub fn of(vec2: Vec2<f32>, z: f32) -> Vec3<f32> {
        Vec3::<f32> {
            x: vec2.x,
            y: vec2.y,
            z: z,
        }
    }

    /// Calculates the length of this vector.
    pub fn len(&self) -> f32 {
        sqrt((self.x * self.x + self.y * self.y + self.z * self.z) as f32)
    }

    /// Normalises this vector, and returns it for chaining.
    pub fn nor(&mut self) -> &Self {
        let len = self.len();
        self.x /= len;
        self.y /= len;
        self.z /= len;
        self
    }
}

#[derive(Copy, Clone, Format)]
/// Represents a 4D vector. Frequently used for homogenous coordinates.
pub struct Vec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl Add<Vec4<f32>> for Vec4<f32> {
    type Output = Vec4<f32>;
    fn add(self, rhs: Vec4<f32>) -> Self::Output {
        Vec4::<f32> {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl AddAssign<Vec4<f32>> for Vec4<f32> {
    fn add_assign(&mut self, rhs: Vec4<f32>) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }
}

impl Mul<f32> for Vec4<f32> {
    type Output = Vec4<f32>;
    fn mul(self, rhs: f32) -> Self::Output {
        Vec4::<f32> {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl MulAssign<f32> for Vec4<f32> {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self.w *= rhs;
    }
}

impl Vec4<f32> {
    /// Computes the dot/inner/scalar product of the two vectors provided.
    pub fn dot(v1: Vec4<f32>, v2: Vec4<f32>) -> f32 {
        v1.x * v2.x + v1.y * v2.y + v1.z * v2.z + v1.w * v2.w
    }

    /// Computes the cross/vector product of the two vectors provided.
    pub fn cross(v1: Vec3<f32>, v2: Vec3<f32>) -> Vec3<f32> {
        Vec3::<f32> {
            x: v1.y * v2.z - v1.z * v2.y,
            y: v1.x * v2.z - v1.z * v2.x,
            z: v1.x * v2.y - v1.y * v2.x,
        }
    }

    /// Creates a Vec4 out of a Vec3 and a fourth component.
    pub fn of(vec3: Vec3<f32>, w: f32) -> Vec4<f32> {
        Vec4::<f32> {
            x: vec3.x,
            y: vec3.y,
            z: vec3.z,
            w: w,
        }
    }

    /// Calculates the length of this vector.
    pub fn len(&self) -> f32 {
        sqrt((self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w) as f32)
    }

    /// Normalises this vector, and returns it for chaining.
    pub fn nor(&mut self) -> &Self {
        let len = self.len();
        self.x /= len;
        self.y /= len;
        self.z /= len;
        self.w /= len;
        self
    }
}

impl PartialEq for Vec4<f32> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z && self.w == other.w
    }
}
