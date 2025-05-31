/// Represents a 2D vector.
pub struct Vec2<T> {
    x: T,
    y: T
}

impl Vec2<T> {
    /// Adds the provided vector to this vector, and returns it for chaining.
    fn add(&self, other: Vec2<T>) -> Vec2<T> {
        self.x += other.x;
        self.y += other.y;
        self;
    }

    /// Subtracts the provided vector from this vector, and returns it for chaining.
    fn sub(&self, other: Vec2<T>) -> Vec2<T> {
        self.x -= other.x;
        self.y -= other.y;
        self;
    }

    /// Computes the dot/inner/scalar product of the two vectors provided.
    fn dot(v1: Vec2<T>, v2: Vec2<T>) -> T {
        v1.x * v2.x + v1.y * v2.y
    }

    /// Scales this vector by a linear factor, and returns it for chaining.
    fn scl(&self, factor: T) -> Vec2<T> {
        self.x *= factor;
        self.y *= factor;
        self;
    }

    /// Rotates this vector anti-clockwise by the given angle.
    fn rotate(&self, angle: Angle) -> Vec2<T> {
        self.x *= angle.cos();
        self.y *= angle.sin();
    }

    /// Calculates the length of this vector.
    fn len(&self) -> f32 {
        ((x*x+y*y) as f32).sqrt();
    }

    /// Normalises this vector, and returns it for chaining.
    fn nor(&self) -> Vec2<T> {
        let len = len(self);
        self.x /= len;
        self.y /= len;
        self;
    } 
}

/// Represents a 3D vector.
pub struct Vec3<T> {
    x: T,
    y: T,
    z: T
}

impl Vec3<T> {
    /// Adds the provided vector to this vector, and returns it for chaining.
    fn add(&self, other: Vec3<T>) -> Vec3<T> {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
        self;
    }

    /// Subtracts the provided vector from this vector, and returns it for chaining.
    fn sub(&self, other: Vec3<T>) -> Vec3<T> {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
        self;
    }

    /// Computes the dot/inner/scalar product of the two vectors provided.
    fn dot(v1: Vec3<T>, v2: Vec3<T>) -> T {
        v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
    }

    /// Computes the cross/vector product of the two vectors provided.
    fn cross(v1: Vec3<T>, v2: Vec3<T>) -> Vec3<T> {
        Vec3<T> {
            x: v1.y * v2.z - v1.z * v2.y,
            y: v1.x * v2.z - v1.z * v2.x,
            z: v1.x * v2.y - v1.y * v2.x 
        }
    }

    /// Scales this vector by a linear factor, and returns it for chaining.
    fn scl(&self, factor: T) -> Vec2<T> {
        self.x *= factor;
        self.y *= factor;
        self.z *= factor;
        self;
    }

    /// Creates a Vec3 out of a Vec2 and a third component.
    fn of(vec2: Vec2<T>, z: T) -> Vec3<T> {
        Vec3<T> {
            x: vec2.x,
            y: vec2.y,
            z: z
        }
    }

    /// Rotates this vector anti-clockwise by the given angle around the specified axis.
    fn rotate(&self, angle: Angle, axis: Vec3<T>) -> Vec3<T> {
        // TODO
    }

    /// Calculates the length of this vector.
    fn len(&self) -> f32 {
        ((x*x+y*y+zz) as f32).sqrt();
    }

    /// Normalises this vector, and returns it for chaining.
    fn nor(&self) -> Vec3<T> {
        let len = len(self);
        self.x /= len;
        self.y /= len;
        self.z /= len;
        self;
    } 
}