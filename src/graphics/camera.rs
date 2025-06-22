use core::f32::consts::PI;

use crate::{
    display::dvi::DviInterface,
    graphics::viewport::FrameBuffer,
    model::model::Model,
    types::{angle::tan, matrix::Mat4, vector::Vec3},
};

/// A virtual camera for rendering
pub struct Camera<const W: usize, const H: usize>
where
    [(); W * H]:,
{
    pub pos: Vec3<f32>,
    pub dir: Vec3<f32>,
    pub up: Vec3<f32>,
    pub proj: Mat4<f32>,
    pub view: Mat4<f32>,
    pub near: f32,
    pub far: f32,
    pub fov: f32,
    // pub fbo: FrameBuffer<W, H>,
}

impl<const W: usize, const H: usize> Camera<W, H>
where
    [(); W * H]:,
{
    pub fn render(&self, object: &Model, model_transform: Mat4<f32>, output: &DviInterface) {
        for x in 0..W {
            for y in 0..H {}
        }
    }

    /// Creates a new perspective camera
    pub fn perspective(
        fov_h: f32,
        pos: Vec3<f32>,
        dir: Vec3<f32>,
        up: Vec3<f32>,
        near: f32,
        far: f32,
    ) -> Camera<W, H> {
        Camera {
            pos: pos,
            dir: dir,
            up: up,
            fov: fov_h,
            near: near,
            far: far,
            // fbo: FrameBuffer::<W, H>::new(),
            proj: Self::projection(near, far, fov_h, W as f32 / H as f32),
            view: Self::view(pos, pos + dir, up),
        }
    }

    /// Creates a projection matrix
    pub fn projection(near: f32, far: f32, fov_h: f32, aspect: f32) -> Mat4<f32> {
        let l_fd = 1.0 / tan((fov_h * (PI / 180.0)) / 2.0);
        let l_a1 = (far + near) / (near - far);
        let l_a2 = (2.0 * far * near) / (near - far);
        Mat4 {
            v_00: l_fd / aspect,
            v_01: 0.0,
            v_02: 0.0,
            v_03: 0.0,
            v_10: 0.0,
            v_11: l_fd,
            v_12: 0.0,
            v_13: 0.0,
            v_20: 0.0,
            v_21: 0.0,
            v_22: l_a1,
            v_23: -1.0,
            v_30: 0.0,
            v_31: 0.0,
            v_32: l_a2,
            v_33: 0.0,
        }
    }

    /// Creates a view matrix
    pub fn view(pos: Vec3<f32>, target: Vec3<f32>, up: Vec3<f32>) -> Mat4<f32> {
        let n_pos = *pos.clone().nor();
        let n_target = *target.clone().nor();
        let n_up = *up.clone().nor();
        let dir: Vec3<f32> = *(n_target - n_pos).nor();
        let right: Vec3<f32> = *Vec3::cross(dir, n_up).nor();
        let new_up: Vec3<f32> = *Vec3::cross(right, dir).nor();
        Mat4::<f32> {
            v_00: right.x,
            v_01: right.y,
            v_02: right.z,
            v_03: 0.0,
            v_10: new_up.x,
            v_11: new_up.y,
            v_12: new_up.z,
            v_13: 0.0,
            v_20: dir.x,
            v_21: dir.y,
            v_22: dir.z,
            v_23: 0.0,
            v_30: 0.0,
            v_31: 0.0,
            v_32: 0.0,
            v_33: 0.0,
        } * Mat4::translate(-pos)
    }
}
