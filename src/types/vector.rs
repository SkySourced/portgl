use super::super::math::sqrt;
use core::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub};
use defmt::Format;

pub const VEC2_ZERO: Vec2<f32> = Vec2 { x: 0.0, y: 0.0 };
pub const VEC2_X: Vec2<f32> = Vec2 { x: 1.0, y: 0.0 };
pub const VEC2_Y: Vec2<f32> = Vec2 { x: 0.0, y: 1.0 };

pub const VEC3_ZERO: Vec3<f32> = Vec3 {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};
pub const VEC3_X: Vec3<f32> = Vec3 {
    x: 1.0,
    y: 0.0,
    z: 0.0,
};
pub const VEC3_Y: Vec3<f32> = Vec3 {
    x: 0.0,
    y: 1.0,
    z: 0.0,
};
pub const VEC3_Z: Vec3<f32> = Vec3 {
    x: 0.0,
    y: 0.0,
    z: 1.0,
};

pub const VEC4_ZERO: Vec4<f32> = Vec4 {
    x: 0.0,
    y: 0.0,
    z: 0.0,
    w: 0.0,
};
pub const VEC4_X: Vec4<f32> = Vec4 {
    x: 1.0,
    y: 0.0,
    z: 0.0,
    w: 0.0,
};
pub const VEC4_Y: Vec4<f32> = Vec4 {
    x: 0.0,
    y: 1.0,
    z: 0.0,
    w: 0.0,
};
pub const VEC4_Z: Vec4<f32> = Vec4 {
    x: 0.0,
    y: 0.0,
    z: 1.0,
    w: 0.0,
};
pub const VEC4_W: Vec4<f32> = Vec4 {
    x: 0.0,
    y: 0.0,
    z: 0.0,
    w: 1.0,
};

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
        if *self == VEC2_ZERO {
            return self;
        }
        let len = self.len();
        self.x /= len;
        self.y /= len;
        self
    }
}

#[derive(Copy, Clone, Format, Debug)]
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

impl Sub<Vec3<f32>> for Vec3<f32> {
    type Output = Vec3<f32>;

    fn sub(self, rhs: Vec3<f32>) -> Self::Output {
        Vec3::<f32> {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
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

/// Componentwise
impl Mul for Vec3<f32> {
    type Output = Vec3<f32>;
    fn mul(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
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

impl Neg for Vec3<f32> {
    type Output = Vec3<f32>;

    fn neg(self) -> Self::Output {
        self * -1.0
    }
}

impl PartialEq for Vec3<f32> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Add for Vec3<u8> {
    type Output = Vec3<u8>;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Vec3<u8> {
    /// Converts from 8-bit colour to f32 colour.
    pub fn to_float_colour(&self) -> Vec3<f32> {
        Vec3 {
            x: f32::min(self.x as f32 / 255.0, 1.0),
            y: f32::min(self.y as f32 / 255.0, 1.0),
            z: f32::min(self.z as f32 / 255.0, 1.0),
        }
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

    /// Projects a vector onto another vector.
    pub fn proj(v1: Vec3<f32>, v2: Vec3<f32>) -> Vec3<f32> {
        v2 * (Self::dot(v1, v2) / Self::dot(v2, v2))
    }

    /// Converts from f32 colour to 8-bit colour.
    pub fn to_8bit_colour(&self) -> Vec3<u8> {
        Vec3 {
            x: u8::min((self.x * 255.0) as u8, 255),
            y: u8::min((self.y * 255.0) as u8, 255),
            z: u8::min((self.z * 255.0) as u8, 255),
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

    /// Creates a Vec2 out of a Vec3, discarding `z`
    pub fn to_vec2(&self) -> Vec2<f32> {
        Vec2 {
            x: self.x,
            y: self.y,
        }
    }

    /// Calculates the length of this vector.
    pub fn len(&self) -> f32 {
        sqrt((self.x * self.x + self.y * self.y + self.z * self.z) as f32)
    }

    /// Normalises this vector, and returns it for chaining.
    pub fn nor(&mut self) -> &Self {
        if *self == VEC3_ZERO {
            return self;
        }
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

impl Sub<Vec4<f32>> for Vec4<f32> {
    type Output = Vec4<f32>;

    fn sub(self, rhs: Vec4<f32>) -> Self::Output {
        Vec4::<f32> {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
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

    /// Creates a `Vec4` out of a `Vec3` and a fourth component.
    pub fn of(vec3: Vec3<f32>, w: f32) -> Vec4<f32> {
        Vec4::<f32> {
            x: vec3.x,
            y: vec3.y,
            z: vec3.z,
            w: w,
        }
    }

    /// Creates a `Vec3` out of a `Vec4`, discarding `w`.
    pub fn to_vec3(&self) -> Vec3<f32> {
        Vec3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }

    /// Divides by the perspective component (`w`) creating a `Vec3`.
    pub fn perspective_division(&self) -> Vec3<f32> {
        if self.w == 0.0 {
            panic!("Zero division")
        }
        Vec3 {
            x: self.x / self.w,
            y: self.y / self.w,
            z: self.z / self.w,
        }
    }

    /// Calculates the length of this vector.
    pub fn len(&self) -> f32 {
        sqrt((self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w) as f32)
    }

    /// Normalises this vector, and returns it for chaining.
    pub fn nor(&mut self) -> &Self {
        if *self == VEC4_ZERO {
            return self;
        }
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
