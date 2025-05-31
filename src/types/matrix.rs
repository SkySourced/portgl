/// Represents a 4x4 matrix.
pub struct Mat4<T> {
    v_00: T, 
    v_01: T,
    v_02: T,
    v_03: T,
    v_10: T,
    v_11: T,
    v_12: T,
    v_13: T,
    v_20: T,
    v_21: T,
    v_22: T,
    v_23: T,
    v_30: T,
    v_31: T,
    v_32: T,
    v_33: T,
}

impl Mat4<T> {
    /// Adds the provided matrix to this matrix, and returns the matrix for chaining.
    fn add(&self, other: &Mat4<T>) -> Mat4<T> {
        self.v_00 += other.v_00;
        self.v_01 += other.v_01;
        self.v_02 += other.v_02;
        self.v_03 += other.v_03;
        self.v_10 += other.v_10;
        self.v_11 += other.v_11;
        self.v_12 += other.v_12;
        self.v_13 += other.v_13;
        self.v_20 += other.v_20;
        self.v_21 += other.v_21;
        self.v_22 += other.v_22;
        self.v_23 += other.v_23;
        self.v_30 += other.v_30;
        self.v_31 += other.v_31;
        self.v_32 += other.v_32;
        self.v_33 += other.v_33;
        self
    }

    /// Subtracts the provided matrix from this matrix, and returns the matrix for chaining.
    fn sub(&self, other: &Mat4<T>) -> Mat4<T> {
        self.v_00 -= other.v_00;
        self.v_01 -= other.v_01;
        self.v_02 -= other.v_02;
        self.v_03 -= other.v_03;
        self.v_10 -= other.v_10;
        self.v_11 -= other.v_11;
        self.v_12 -= other.v_12;
        self.v_13 -= other.v_13;
        self.v_20 -= other.v_20;
        self.v_21 -= other.v_21;
        self.v_22 -= other.v_22;
        self.v_23 -= other.v_23;
        self.v_30 -= other.v_30;
        self.v_31 -= other.v_31;
        self.v_32 -= other.v_32;
        self.v_33 -= other.v_33;
        self
    }

    /// Multiplies this matrix by the provided matrix, and returns the matrix for chaining.
    fn mul(&self, other: &Mat4<T>) -> Mat4<T> {
        Mat4<T> {
            v_00: self.v_00 * other.v_00 + self.v_01 * other.v_10 + self.v_02 * other.v_20 + self.v_03 * other.v_30,
            v_01: self.v_00 * other.v_01 + self.v_01 * other.v_11 + self.v_02 * other.v_21 + self.v_03 * other.v_31,
            v_02: self.v_00 * other.v_02 + self.v_01 * other.v_12 + self.v_02 * other.v_22 + self.v_03 * other.v_32,
            v_03: self.v_00 * other.v_03 + self.v_01 * other.v_13 + self.v_02 * other.v_23 + self.v_03 * other.v_33,
            v_10: self.v_10 * other.v_00 + self.v_11 * other.v_10 + self.v_12 * other.v_20 + self.v_13 * other.v_30,
            v_11: self.v_10 * other.v_01 + self.v_11 * other.v_11 + self.v_12 * other.v_21 + self.v_13 * other.v_31,
            v_12: self.v_10 * other.v_02 + self.v_11 * other.v_12 + self.v_12 * other.v_22 + self.v_13 * other.v_32,
            v_13: self.v_10 * other.v_03 + self.v_11 * other.v_13 + self.v_12 * other.v_23 + self.v_13 * other.v_33,
            v_20: self.v_20 * other.v_00 + self.v_21 * other.v_10 + self.v_22 * other.v_20 + self.v_23 * other.v_30,
            v_21: self.v_20 * other.v_01 + self.v_21 * other.v_11 + self.v_22 * other.v_21 + self.v_23 * other.v_31,
            v_22: self.v_20 * other.v_02 + self.v_21 * other.v_12 + self.v_22 * other.v_22 + self.v_23 * other.v_32,
            v_23: self.v_20 * other.v_03 + self.v_21 * other.v_13 + self.v_22 * other.v_23 + self.v_23 * other.v_33,
            v_30: self.v_30 * other.v_00 + self.v_31 * other.v_10 + self.v_32 * other.v_20 + self.v_33 * other.v_30,
            v_31: self.v_30 * other.v_01 + self.v_31 * other.v_11 + self.v_32 * other.v_21 + self.v_33 * other.v_31,
            v_32: self.v_30 * other.v_02 + self.v_31 * other.v_12 + self.v_32 * other.v_22 + self.v_33 * other.v_32,
            v_33: self.v_30 * other.v_03 + self.v_31 * other.v_13 + self.v_32 * other.v_23 + self.v_33 * other.v_33,
        }
    }

    /// Scales the matrix by a linear factor, and returns the matrix for chaining.
    fn scl(&self, factor: f32) -> Mat4<T> {
        self.v_00 *= factor;
        self.v_01 *= factor;
        self.v_02 *= factor;
        self.v_03 *= factor;
        self.v_10 *= factor;
        self.v_11 *= factor;
        self.v_12 *= factor;
        self.v_13 *= factor;
        self.v_10 *= factor;
        self.v_21 *= factor;
        self.v_22 *= factor;
        self.v_23 *= factor;
        self.v_30 *= factor;
        self.v_31 *= factor;
        self.v_32 *= factor;
        self.v_33 *= factor;
        self
    }

    /// Returns a distinct copy of this matrix.
    fn cpy(&self) -> Mat4<T> {
        Mat4<T> {
            v_00: self.v_00,
            v_01: self.v_01,
            v_02: self.v_02,
            v_03: self.v_03,
            v_10: self.v_10,
            v_11: self.v_11,
            v_12: self.v_12,
            v_13: self.v_13,
            v_20: self.v_20,
            v_21: self.v_21,
            v_22: self.v_22,
            v_23: self.v_23,
            v_30: self.v_30,
            v_31: self.v_31,
            v_32: self.v_32,
            v_33: self.v_33,
        }
    }

    /// Returns a new 4x4 identity matrix.
    fn idt() -> Mat4<T> {
        Mat4<T> {
            v_00: 1,
            v_01: 0,
            v_02: 0,
            v_03: 0,
            v_10: 0,
            v_11: 1,
            v_12: 0,
            v_13: 0,
            v_20: 0,
            v_21: 0,
            v_22: 1,
            v_23: 0,
            v_30: 0,
            v_31: 0,
            v_32: 0,
            v_33: 1,
        }
    }

    /// Creates a translation matrix.
    fn translate(Vec3<T> vec) -> Mat4<T> {
        Mat4<T> {
            v_00: 0,
            v_01: 0,
            v_02: 0,
            v_03: vec.x,
            v_10: 0,
            v_11: 0,
            v_12: 0,
            v_13: vec.y,
            v_20: 0,
            v_21: 0,
            v_22: 0,
            v_23: vec.z,
            v_30: 0,
            v_31: 0,
            v_32: 0,
            v_33: 1,
        }
    }
}