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

    fn mul(&self, other: &Mat4<T>) -> Mat4<T> {
        Mat4 {
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
}