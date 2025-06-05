use super::vector::{Vec3, Vec4};

#[derive(Copy, Clone)]
/// Represents a 4x4 matrix.
/// Structure:
// [v_00 v_01 v_02 v_03]
// [v_10 v_11 v_12 v_13]
// [v_20 v_21 v_22 v_23]
// [v_30 v_31 v_32 v_33]
pub struct Mat4<T> {
    pub v_00: T,
    pub v_01: T,
    pub v_02: T,
    pub v_03: T,
    pub v_10: T,
    pub v_11: T,
    pub v_12: T,
    pub v_13: T,
    pub v_20: T,
    pub v_21: T,
    pub v_22: T,
    pub v_23: T,
    pub v_30: T,
    pub v_31: T,
    pub v_32: T,
    pub v_33: T,
}

impl core::ops::Add<Mat4<f32>> for Mat4<f32> {
    type Output = Mat4<f32>;
    fn add(self, rhs: Mat4<f32>) -> Self::Output {
        Mat4::<f32> {
            v_00: self.v_00 + rhs.v_00,
            v_01: self.v_01 + rhs.v_01,
            v_02: self.v_02 + rhs.v_02,
            v_03: self.v_03 + rhs.v_03,
            v_10: self.v_10 + rhs.v_10,
            v_11: self.v_11 + rhs.v_11,
            v_12: self.v_12 + rhs.v_12,
            v_13: self.v_13 + rhs.v_13,
            v_20: self.v_20 + rhs.v_20,
            v_21: self.v_21 + rhs.v_21,
            v_22: self.v_22 + rhs.v_22,
            v_23: self.v_23 + rhs.v_23,
            v_30: self.v_30 + rhs.v_30,
            v_31: self.v_31 + rhs.v_31,
            v_32: self.v_32 + rhs.v_32,
            v_33: self.v_33 + rhs.v_33,
        }
    }
}

impl core::ops::AddAssign<Mat4<f32>> for Mat4<f32> {
    fn add_assign(&mut self, rhs: Mat4<f32>) {
        self.v_00 += rhs.v_00;
        self.v_01 += rhs.v_01;
        self.v_02 += rhs.v_02;
        self.v_03 += rhs.v_03;
        self.v_10 += rhs.v_10;
        self.v_11 += rhs.v_11;
        self.v_12 += rhs.v_12;
        self.v_13 += rhs.v_13;
        self.v_20 += rhs.v_20;
        self.v_21 += rhs.v_21;
        self.v_22 += rhs.v_22;
        self.v_23 += rhs.v_23;
        self.v_30 += rhs.v_30;
        self.v_31 += rhs.v_31;
        self.v_32 += rhs.v_32;
        self.v_33 += rhs.v_33;
    }
}

impl core::ops::Mul<Mat4<f32>> for Mat4<f32> {
    type Output = Mat4<f32>;
    fn mul(self, rhs: Mat4<f32>) -> Self::Output {
        Mat4::<f32> {
            v_00: self.v_00 * rhs.v_00
                + self.v_01 * rhs.v_10
                + self.v_02 * rhs.v_20
                + self.v_03 * rhs.v_30,
            v_01: self.v_00 * rhs.v_01
                + self.v_01 * rhs.v_11
                + self.v_02 * rhs.v_21
                + self.v_03 * rhs.v_31,
            v_02: self.v_00 * rhs.v_02
                + self.v_01 * rhs.v_12
                + self.v_02 * rhs.v_22
                + self.v_03 * rhs.v_32,
            v_03: self.v_00 * rhs.v_03
                + self.v_01 * rhs.v_13
                + self.v_02 * rhs.v_23
                + self.v_03 * rhs.v_33,
            v_10: self.v_10 * rhs.v_00
                + self.v_11 * rhs.v_10
                + self.v_12 * rhs.v_20
                + self.v_13 * rhs.v_30,
            v_11: self.v_10 * rhs.v_01
                + self.v_11 * rhs.v_11
                + self.v_12 * rhs.v_21
                + self.v_13 * rhs.v_31,
            v_12: self.v_10 * rhs.v_02
                + self.v_11 * rhs.v_12
                + self.v_12 * rhs.v_22
                + self.v_13 * rhs.v_32,
            v_13: self.v_10 * rhs.v_03
                + self.v_11 * rhs.v_13
                + self.v_12 * rhs.v_23
                + self.v_13 * rhs.v_33,
            v_20: self.v_20 * rhs.v_00
                + self.v_21 * rhs.v_10
                + self.v_22 * rhs.v_20
                + self.v_23 * rhs.v_30,
            v_21: self.v_20 * rhs.v_01
                + self.v_21 * rhs.v_11
                + self.v_22 * rhs.v_21
                + self.v_23 * rhs.v_31,
            v_22: self.v_20 * rhs.v_02
                + self.v_21 * rhs.v_12
                + self.v_22 * rhs.v_22
                + self.v_23 * rhs.v_32,
            v_23: self.v_20 * rhs.v_03
                + self.v_21 * rhs.v_13
                + self.v_22 * rhs.v_23
                + self.v_23 * rhs.v_33,
            v_30: self.v_30 * rhs.v_00
                + self.v_31 * rhs.v_10
                + self.v_32 * rhs.v_20
                + self.v_33 * rhs.v_30,
            v_31: self.v_30 * rhs.v_01
                + self.v_31 * rhs.v_11
                + self.v_32 * rhs.v_21
                + self.v_33 * rhs.v_31,
            v_32: self.v_30 * rhs.v_02
                + self.v_31 * rhs.v_12
                + self.v_32 * rhs.v_22
                + self.v_33 * rhs.v_32,
            v_33: self.v_30 * rhs.v_03
                + self.v_31 * rhs.v_13
                + self.v_32 * rhs.v_23
                + self.v_33 * rhs.v_33,
        }
    }
}

impl core::ops::Mul<Vec4<f32>> for Mat4<f32> {
    type Output = Vec4<f32>;
    fn mul(self, rhs: Vec4<f32>) -> Self::Output {
        Vec4::<f32> {
            x: self.v_00 * rhs.x + self.v_01 * rhs.y + self.v_02 * rhs.z + self.v_03 * rhs.w,
            y: self.v_10 * rhs.x + self.v_11 * rhs.y + self.v_12 * rhs.z + self.v_13 * rhs.w,
            z: self.v_20 * rhs.x + self.v_21 * rhs.y + self.v_22 * rhs.z + self.v_23 * rhs.w,
            w: self.v_30 * rhs.x + self.v_31 * rhs.y + self.v_32 * rhs.z + self.v_33 * rhs.w,
        }
    }
}

impl core::ops::Mul<f32> for Mat4<f32> {
    type Output = Mat4<f32>;
    fn mul(self, rhs: f32) -> Self::Output {
        Mat4::<f32> {
            v_00: self.v_00 * rhs,
            v_01: self.v_01 * rhs,
            v_02: self.v_02 * rhs,
            v_03: self.v_03 * rhs,
            v_10: self.v_10 * rhs,
            v_11: self.v_11 * rhs,
            v_12: self.v_12 * rhs,
            v_13: self.v_13 * rhs,
            v_20: self.v_20 * rhs,
            v_21: self.v_21 * rhs,
            v_22: self.v_22 * rhs,
            v_23: self.v_23 * rhs,
            v_30: self.v_30 * rhs,
            v_31: self.v_31 * rhs,
            v_32: self.v_32 * rhs,
            v_33: self.v_33 * rhs,
        }
    }
}

impl core::ops::MulAssign<Mat4<f32>> for Mat4<f32> {
    fn mul_assign(&mut self, rhs: Mat4<f32>) {
        self.v_00 = self.v_00 * rhs.v_00
            + self.v_01 * rhs.v_10
            + self.v_02 * rhs.v_20
            + self.v_03 * rhs.v_30;
        self.v_01 = self.v_00 * rhs.v_01
            + self.v_01 * rhs.v_11
            + self.v_02 * rhs.v_21
            + self.v_03 * rhs.v_31;
        self.v_02 = self.v_00 * rhs.v_02
            + self.v_01 * rhs.v_12
            + self.v_02 * rhs.v_22
            + self.v_03 * rhs.v_32;
        self.v_03 = self.v_00 * rhs.v_03
            + self.v_01 * rhs.v_13
            + self.v_02 * rhs.v_23
            + self.v_03 * rhs.v_33;
        self.v_10 = self.v_10 * rhs.v_00
            + self.v_11 * rhs.v_10
            + self.v_12 * rhs.v_20
            + self.v_13 * rhs.v_30;
        self.v_11 = self.v_10 * rhs.v_01
            + self.v_11 * rhs.v_11
            + self.v_12 * rhs.v_21
            + self.v_13 * rhs.v_31;
        self.v_12 = self.v_10 * rhs.v_02
            + self.v_11 * rhs.v_12
            + self.v_12 * rhs.v_22
            + self.v_13 * rhs.v_32;
        self.v_13 = self.v_10 * rhs.v_03
            + self.v_11 * rhs.v_13
            + self.v_12 * rhs.v_23
            + self.v_13 * rhs.v_33;
        self.v_20 = self.v_20 * rhs.v_00
            + self.v_21 * rhs.v_10
            + self.v_22 * rhs.v_20
            + self.v_23 * rhs.v_30;
        self.v_21 = self.v_20 * rhs.v_01
            + self.v_21 * rhs.v_11
            + self.v_22 * rhs.v_21
            + self.v_23 * rhs.v_31;
        self.v_22 = self.v_20 * rhs.v_02
            + self.v_21 * rhs.v_12
            + self.v_22 * rhs.v_22
            + self.v_23 * rhs.v_32;
        self.v_23 = self.v_20 * rhs.v_03
            + self.v_21 * rhs.v_13
            + self.v_22 * rhs.v_23
            + self.v_23 * rhs.v_33;
        self.v_30 = self.v_30 * rhs.v_00
            + self.v_31 * rhs.v_10
            + self.v_32 * rhs.v_20
            + self.v_33 * rhs.v_30;
        self.v_31 = self.v_30 * rhs.v_01
            + self.v_31 * rhs.v_11
            + self.v_32 * rhs.v_21
            + self.v_33 * rhs.v_31;
        self.v_32 = self.v_30 * rhs.v_02
            + self.v_31 * rhs.v_12
            + self.v_32 * rhs.v_22
            + self.v_33 * rhs.v_32;
        self.v_33 = self.v_30 * rhs.v_03
            + self.v_31 * rhs.v_13
            + self.v_32 * rhs.v_23
            + self.v_33 * rhs.v_33;
    }
}

impl core::ops::MulAssign<f32> for Mat4<f32> {
    fn mul_assign(&mut self, rhs: f32) {
        self.v_00 *= rhs;
        self.v_01 *= rhs;
        self.v_02 *= rhs;
        self.v_03 *= rhs;
        self.v_10 *= rhs;
        self.v_11 *= rhs;
        self.v_12 *= rhs;
        self.v_13 *= rhs;
        self.v_20 *= rhs;
        self.v_21 *= rhs;
        self.v_22 *= rhs;
        self.v_23 *= rhs;
        self.v_30 *= rhs;
        self.v_31 *= rhs;
        self.v_32 *= rhs;
        self.v_33 *= rhs;
    }
}

impl core::fmt::Debug for Mat4<f32> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("FloatMat4").field("v_00", &self.v_00).field("v_01", &self.v_01).field("v_02", &self.v_02).field("v_03", &self.v_03).field("v_10", &self.v_10).field("v_11", &self.v_11).field("v_12", &self.v_12).field("v_13", &self.v_13).field("v_20", &self.v_20).field("v_21", &self.v_21).field("v_22", &self.v_22).field("v_23", &self.v_23).field("v_30", &self.v_30).field("v_31", &self.v_31).field("v_32", &self.v_32).field("v_33", &self.v_33).finish()
    }
}

impl Mat4<f32> {
    /// Returns a new 4x4 identity matrix.
    pub fn idt() -> Mat4<f32> {
        Mat4::<f32> {
            v_00: 1.0,
            v_01: 0.0,
            v_02: 0.0,
            v_03: 0.0,
            v_10: 0.0,
            v_11: 1.0,
            v_12: 0.0,
            v_13: 0.0,
            v_20: 0.0,
            v_21: 0.0,
            v_22: 1.0,
            v_23: 0.0,
            v_30: 0.0,
            v_31: 0.0,
            v_32: 0.0,
            v_33: 1.0,
        }
    }

    /// Creates a translation matrix.
    pub fn translate(vec: Vec3<f32>) -> Mat4<f32> {
        Mat4::<f32> {
            v_00: 0.0,
            v_01: 0.0,
            v_02: 0.0,
            v_03: vec.x,
            v_10: 0.0,
            v_11: 0.0,
            v_12: 0.0,
            v_13: vec.y,
            v_20: 0.0,
            v_21: 0.0,
            v_22: 0.0,
            v_23: vec.z,
            v_30: 0.0,
            v_31: 0.0,
            v_32: 0.0,
            v_33: 1.0,
        }
    }
}
