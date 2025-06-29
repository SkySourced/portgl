use core::f32::consts::PI;
use defmt::info;
use heapless::Vec;

use crate::{
    display::dvi::DviInterface,
    graphics::texture::Texture,
    math::powi,
    model::model::{self, Model, Vertex},
    types::{
        angle::tan,
        matrix::Mat4,
        vector::{Vec3, Vec4, VEC3_Z, VEC3_ZERO},
    },
};

const LIGHT_POS: Vec3<f32> = Vec3 {
    x: 1.5,
    y: 0.0,
    z: -1.5,
};
const LIGHT_COL: Vec3<f32> = Vec3 {
    x: 1.0,
    y: 1.0,
    z: 1.0,
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
    pub fov_y: f32,
    texture: Texture<16, 16>, //pub fbo: FrameBuffer<Vec3<u8>, W, H>,
}

impl<const W: usize, const H: usize> Camera<W, H>
where
    [(); W * H]:,
{
    pub fn render_test(&mut self, output: &mut DviInterface) {
        for y in 0..H {
            for x in 0..W {
                output.render_pixel(Vec3 {
                    x: (x as f32 / W as f32 * 255.0) as u8,
                    y: (y as f32 / H as f32 * 255.0) as u8,
                    z: 0,
                });
            }
            output.end_row();
        }
        output.end_frame();
    }

    pub fn render(
        &mut self,
        object: &Model,
        model_transform: Mat4<f32>,
        output: &mut DviInterface,
    ) {
        let mut local_model: Model = object.clone();
        let mut pd_verts: Vec<Vec3<f32>, { model::NUM_VERTS }> = Vec::new(); // Transformed verts after perspective division

        // Vertex shader
        local_model.verts.iter_mut().for_each(|vertex| -> () {
            info!("Initial: {:?}", vertex.pos);
            vertex.pos = model_transform * vertex.pos;
            info!("Model: {:?}", vertex.pos);
            vertex.pos = self.view * vertex.pos;
            info!("View: {:?}", vertex.pos);
            vertex.pos = self.proj * vertex.pos;
            info!("Projection: {:?}", vertex.pos);
            info!("Perspective division: {:?}", vertex.pos.perspective_division());
            pd_verts
                .push(vertex.pos.perspective_division())
                .expect("vertex vectors should be the same length");
        });

        // Rasterisation and rendering
        for y in 0..H {
            for x in 0..W {
                let x_ndc = (x as f32 / W as f32 - 0.5) * 2.0;
                let y_ndc = (y as f32 / H as f32 - 0.5) * 2.0;

                let ray_origin = Vec3 {
                    x: x_ndc,
                    y: y_ndc,
                    z: -1.0,
                };
                let ray_direction = VEC3_Z;

                let mut vertex: Option<Vertex> = None;
                let fragment_colour: Vec3<u8>;

                // Vertex attribute interpolation
                for face in &local_model.faces {
                    let intersection_loc =
                        face.ray_intersects_face(&local_model, ray_origin, ray_direction);

                    if intersection_loc.is_some()
                        //&& face.ray_front_face(&local_model, ray_direction)
                    {
                        let p = intersection_loc.expect("there should be a value");
                        let vert_a = local_model
                            .verts
                            .get(face.verts[0])
                            .expect("there should be a vertex specified by the face");
                        let vert_b = local_model
                            .verts
                            .get(face.verts[1])
                            .expect("there should be a vertex specified by the face");
                        let vert_c = local_model
                            .verts
                            .get(face.verts[2])
                            .expect("there should be a vertex specified by the face");

                        let a = vert_a.pos.to_vec3();
                        let b = vert_b.pos.to_vec3();
                        let c = vert_c.pos.to_vec3();

                        let pbc_area = 0.5 * Vec3::len(&Vec3::cross(b - p, c - p));
                        let pac_area = 0.5 * Vec3::len(&Vec3::cross(a - p, c - p));
                        let pab_area = 0.5 * Vec3::len(&Vec3::cross(a - p, b - p));
                        let tot_area = pbc_area + pac_area + pab_area;

                        let w_a = pbc_area / tot_area;
                        let w_b = pac_area / tot_area;
                        let w_c = pab_area / tot_area;

                        vertex = Some(Vertex {
                            pos: Vec4::of(p, 1.0),
                            tex_coords: vert_a.tex_coords * w_a
                                + vert_b.tex_coords * w_b
                                + vert_c.tex_coords * w_c,
                            normal: vert_a.normal * w_a + vert_b.normal * w_b + vert_c.normal * w_c,
                        })
                    }
                }

                // Vertex shader
                if vertex.is_some() {
                    let vertex = vertex.expect("vertex should exist");
                    // Blinn Phong shading

                    // Config
                    let ambient_colour = Vec3::<f32> {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    };
                    let ambient_factor: f32 = 0.1;
                    let specular_colour = Vec3::<f32> {
                        x: 0.1,
                        y: 0.1,
                        z: 0.1,
                    };
                    let specular_factor: f32 = 0.25;
                    let diffuse_factor: f32 = 0.65;
                    let shininess: u8 = 32;

                    // general calc
                    let light_dir = *(vertex.pos.to_vec3() - LIGHT_POS).nor();

                    // Ambient
                    let ambient = ambient_colour * ambient_factor;

                    // Diffuse
                    let diffuse_colour: Vec3<f32> = self
                        .texture
                        .sample(vertex.tex_coords)
                        .expect("texture should have pixel for coordinates")
                        .to_float_colour();
                    let lambertian = f32::max(Vec3::dot(light_dir, vertex.normal.to_vec3()), 0.0);
                    let diffuse = diffuse_colour * LIGHT_COL * lambertian * diffuse_factor;

                    // Specular
                    let half_dir = *(light_dir + -*(vertex.pos.to_vec3()).nor()).nor();
                    let view_angle = f32::max(Vec3::dot(half_dir, vertex.normal.to_vec3()), 0.0);
                    let specular = specular_colour * powi(view_angle, shininess) * specular_factor;

                    fragment_colour = (ambient + specular + diffuse).to_8bit_colour();
                    // fragment_colour = VEC3_Z.to_8bit_colour();
                } else {
                    fragment_colour = VEC3_ZERO.to_8bit_colour();
                }

                output.render_pixel(fragment_colour);
            }

            output.end_row();
        }
        output.end_frame();
    }

    /// Creates a new perspective camera. `fov_h` is measured in degrees
    pub fn perspective(
        fov_h: f32,
        pos: Vec3<f32>,
        dir: Vec3<f32>,
        up: Vec3<f32>,
        near: f32,
        far: f32,
    ) -> Camera<W, H> {
        let view = Self::view(pos, pos + dir, up);
        info!("View: {:?}", view);
        let proj = Self::projection(near, far, fov_h, W as f32 / H as f32);
        info!("Proj: {:?}", proj);
        Camera {
            pos,
            dir,
            up,
            fov_y: fov_h,
            near,
            far,
            // fbo: FrameBuffer::<Vec3<u8>, W, H>::new(),
            proj,
            view,
            texture: Texture::<16, 16>::gen_checkerboard(),
        }
    }

    /// Creates a projection matrix
    pub fn projection(near: f32, far: f32, fov_h: f32, aspect: f32) -> Mat4<f32> {
        let l_fd = 1.0 / tan((fov_h * (PI / 180.0)) / 2.0);
        let l_a1 = (-(near-far))/(near-far);
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
            v_23: l_a2,
            v_30: 0.0,
            v_31: 0.0,
            v_32: 1.0,
            v_33: 0.0,
        }
    }

    /// Creates a view matrix
    pub fn view(eye: Vec3<f32>, target: Vec3<f32>, up: Vec3<f32>) -> Mat4<f32> {
        let forward = *(eye - target).nor();
        let right = *Vec3::cross(up, forward).nor();
        let new_up = Vec3::cross(forward, right);
        Mat4::<f32> {
            v_00: right.x, v_01: new_up.x, v_02: forward.x, v_03: 0.0,
            v_10: right.y, v_11: new_up.y, v_12: forward.y, v_13: 0.0,
            v_20: right.z, v_21: new_up.z, v_22: forward.z, v_23: 0.0,
            v_30: 0.0    , v_31: 0.0     , v_32: 0.0      , v_33: 1.0
        } * Mat4::<f32> {
            v_00: 1.0, v_01: 0.0, v_02: 0.0, v_03: 0.0,
            v_10: 0.0, v_11: 1.0, v_12: 0.0, v_13: 0.0,
            v_20: 0.0, v_21: 0.0, v_22: 1.0, v_23: 0.0,
            v_30: -eye.x, v_31: -eye.y, v_32: -eye.z, v_33: 1.0,
        }
    }
}
