use super::super::math::sqrt;

/// Represents a 2D vector.
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl core::ops::Add<Vec2<f32>> for Vec2<f32> {
    type Output = Vec2<f32>;
    fn add(self, rhs: Vec2<f32>) -> Self::Output {
        Vec2::<f32> {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl core::ops::AddAssign<Vec2<f32>> for Vec2<f32> {
    fn add_assign(&mut self, rhs: Vec2<f32>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl core::ops::Mul<f32> for Vec2<f32> {
    type Output = Vec2<f32>;
    fn mul(self, rhs: f32) -> Self::Output {
        Vec2::<f32> {
            x: self.x * rhs,
            y: self.y * rhs,
        }
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

/// Represents a 3D vector.
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl core::ops::Add<Vec3<f32>> for Vec3<f32> {
    type Output = Vec3<f32>;
    fn add(self, rhs: Vec3<f32>) -> Self::Output {
        Vec3::<f32> {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl core::ops::AddAssign<Vec3<f32>> for Vec3<f32> {
    fn add_assign(&mut self, rhs: Vec3<f32>) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl core::ops::Mul<f32> for Vec3<f32> {
    type Output = Vec3<f32>;
    fn mul(self, rhs: f32) -> Self::Output {
        Vec3::<f32> {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
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

/// Represents a 4D vector. Frequently used for homogenous coordinates.
pub struct Vec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl core::ops::Add<Vec4<f32>> for Vec4<f32> {
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

impl core::ops::AddAssign<Vec4<f32>> for Vec4<f32> {
    fn add_assign(&mut self, rhs: Vec4<f32>) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }
}

impl core::ops::Mul<f32> for Vec4<f32> {
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
            w: w
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