use crate::types::vector::{Vec2, Vec3, Vec4};
use core::fmt::{Debug, Write};
use defmt::{debug, Format};
use heapless::Vec;

pub const NUM_VERTS: usize = 256;
pub const NUM_FACES: usize = 512;

#[derive(Clone)]
/// Represents a 3D model.
pub struct Model {
    pub verts: Vec<Vertex, NUM_VERTS>,
    pub faces: Vec<Face, NUM_FACES>,
}

#[derive(Clone, Copy, Format)]
/// Represents a vertex. Contains
/// model-space position, texture
/// coordinates, and normals.
pub struct Vertex {
    pub pos: Vec4<f32>,
    pub tex_coords: Vec2<f32>,
    pub normal: Vec4<f32>,
}

#[derive(Debug, Clone, Copy)]
/// Represents a face. Contains
/// indices of 3 composing vertices.
pub struct Face {
    pub verts: [usize; 3],
}

impl Face {
    /// Creates a new face initialized to all zeroes.
    pub fn new() -> Face {
        Face { verts: [0, 0, 0] }
    }

    /// Detects if a ray intersects with a triangular face.
    /// `direction` should be normalised.
    pub fn ray_intersects_face(
        &self,
        model: &Model,
        origin: Vec3<f32>,
        direction: Vec3<f32>,
    ) -> Option<Vec3<f32>> {
        let tri_a = model
            .verts
            .get(self.verts[0])
            .expect("face should have a valid vertex index")
            .pos
            .to_vec3();
        let tri_b = model
            .verts
            .get(self.verts[1])
            .expect("face should have a valid vertex index")
            .pos
            .to_vec3();
        let tri_c = model
            .verts
            .get(self.verts[2])
            .expect("face should have a valid vertex index")
            .pos
            .to_vec3();

        let e1 = tri_b - tri_a;
        let e2 = tri_c - tri_a;

        let ray_cross_e2 = Vec3::<f32>::cross(direction, e2);
        let det = Vec3::<f32>::dot(e1, ray_cross_e2);

        if det > -f32::EPSILON && det < f32::EPSILON {
            return None; // This ray is parallel to this triangle.
        }

        let inv_det = 1.0 / det;
        let s = origin - tri_a;
        let u = inv_det * Vec3::<f32>::dot(s, ray_cross_e2);
        if u < 0.0 || u > 1.0 {
            return None;
        }

        let s_cross_e1 = Vec3::<f32>::cross(s, e1);
        let v = inv_det * Vec3::<f32>::dot(direction, s_cross_e1);
        if v < 0.0 || u + v > 1.0 {
            return None;
        }
        // At this stage we can compute t to find out where the intersection point is on the line.
        let t = inv_det * Vec3::<f32>::dot(e2, s_cross_e1);

        if t > f32::EPSILON {
            // ray intersection
            let intersection_point = origin + direction * t;
            return Some(intersection_point);
        } else {
            // This means that there is a line intersection but not a ray intersection.
            return None;
        }
    }

    /// Returns true if a given ray intersects the front side (towards the normal) of this face.
    /// Returns false if the ray is orthogonal to the normal, or is intersecting the back side.
    /// Does not check if the ray collides. For this, `ray_intersects_face` should be used.
    pub fn ray_front_face(&self, model: &Model, direction: Vec3<f32>) -> bool {
        Vec3::<f32>::dot(
            direction,
            model
                .verts
                .get(self.verts[0])
                .expect("face should have a valid vertex index")
                .normal
                .to_vec3(),
        ) < 0.0
    }
}

/// Creates a Model from an OBJ
/// file.
pub fn from_obj(obj_file: &str) -> Model {
    debug!("Creating model");

    let mut points: Vec<Vec4<f32>, NUM_VERTS> = Vec::new();
    let mut normals: Vec<Vec4<f32>, NUM_VERTS> = Vec::new();
    let mut tex_coords: Vec<Vec2<f32>, NUM_VERTS> = Vec::new();

    debug!("Created vecs");

    let mut lines: core::str::Split<'_, &'static str> = obj_file.split("\n");

    debug!("Split lines");

    let mut vertices: Vec<Vertex, NUM_VERTS> = Vec::new();
    let mut faces: Vec<Face, NUM_FACES> = Vec::new();

    debug!("Created vecs 2");

    let mut context = heapless::String::<16>::new();

    debug!("Created context");

    while let Some(i) = lines.next() {
        // debug!("{:?}", i);
        let mut parts = i.split(" ");

        let token: Option<&str> = parts.next();

        if token == Some("v") {
            context.clear();
            context
                .write_str("vertex")
                .expect("model parse context should be less than 16 chars");

            defmt::expect!(
                points.push(Vec4 {
                    x: parse_float(parts.next(), &context),
                    y: parse_float(parts.next(), &context),
                    z: parse_float(parts.next(), &context),
                    w: 1.0
                }),
                "loaded model should not contain more than 4096 points"
            );

            debug!(
                "Added point_index {} {} {}",
                points.last().unwrap().x,
                points.last().unwrap().y,
                points.last().unwrap().z
            )
        } else if token == Some("vt") {
            context.clear();
            context
                .write_str("tex coords")
                .expect("model parse context should be less than 16 chars");

            defmt::expect!(
                tex_coords.push(Vec2 {
                    x: parse_float(parts.next(), &context),
                    y: parse_float(parts.next(), &context),
                }),
                "loaded model should not contain more than 4096 tex coords"
            );

            debug!(
                "Added tex coord {} {}",
                tex_coords.last().unwrap().x,
                tex_coords.last().unwrap().y
            )
        } else if token == Some("vn") {
            context.clear();
            context
                .write_str("vertex normal")
                .expect("model parse context should be less than 16 chars");

            defmt::expect!(
                normals.push(Vec4 {
                    x: parse_float(parts.next(), &context),
                    y: parse_float(parts.next(), &context),
                    z: parse_float(parts.next(), &context),
                    w: 0.0
                }),
                "loaded model should not contain more than 4096 normals"
            );

            debug!(
                "Added vertex normal {} {} {}",
                normals.last().unwrap().x,
                normals.last().unwrap().y,
                normals.last().unwrap().z
            )
        } else if token == Some("f") {
            context.clear();
            context
                .write_str("face")
                .expect("model parse context should be less than 16 chars");

            let mut face: Face = Face::new();

            for i in 0..3 {
                let mut vertex_components = parts.next().unwrap().split("/");

                let point_index: usize = parse_size(vertex_components.next(), &context);
                let tex_coord_index: usize = parse_size(vertex_components.next(), &context);
                let normal_index: usize = parse_size(vertex_components.next(), &context);

                let vertex_in_array = {
                    get_vertex(
                        point_index,
                        tex_coord_index,
                        normal_index,
                        &points,
                        &tex_coords,
                        &normals,
                        &vertices,
                    )
                };

                face.verts[i] = match vertex_in_array {
                    Some(i) => i,
                    None => {
                        debug!(
                            "Adding vertex with indices {:?} {:?} {:?}",
                            point_index, tex_coord_index, normal_index
                        );
                        defmt::expect!(
                            vertices.push(Vertex {
                                pos: points[point_index - 1].clone(),
                                tex_coords: tex_coords[tex_coord_index - 1].clone(),
                                normal: normals[normal_index - 1].clone(),
                            }),
                            "model should not contain more than 4096 unique vertices"
                        );
                        vertices.len() - 1
                    }
                };
            }

            faces
                .push(face)
                .expect("model should not contain more than 8192 unique faces");

            debug!("Added face {}", faces.last().unwrap().verts)
        }
    }

    Model {
        verts: vertices,
        faces: faces,
    }
}

/// Safely reads a float from a string slice option.
fn parse_float(word: Option<&'_ str>, context: &heapless::String<16>) -> f32 {
    let mut error1 = heapless::String::<32>::new();
    write!(error1, "{} value not found", context).unwrap();
    let mut error2 = heapless::String::<41>::new();
    write!(error2, "value in {} should be float", context).unwrap();

    word.expect(error1.as_str())
        .parse::<f32>()
        .expect(error2.as_str())
}

/// Safely reads an int (usize) from a string slice option.
fn parse_size(word: Option<&'_ str>, context: &heapless::String<16>) -> usize {
    let mut error1 = heapless::String::<32>::new();
    write!(error1, "{} value not found", context).unwrap();
    let mut error2 = heapless::String::<41>::new();
    write!(error2, "value in {} should be usize", context).unwrap();

    word.expect(error1.as_str())
        .parse::<usize>()
        .expect(error2.as_str())
}

/// Gets the index of a vertex with the specified data.
/// Passed indices should start at 1, as in the OBJ face
/// data.
fn get_vertex(
    point_index: usize,
    tex_coord_index: usize,
    normal_index: usize,
    points: &Vec<Vec4<f32>, 256>,
    tex_coords: &Vec<Vec2<f32>, 256>,
    normals: &Vec<Vec4<f32>, 256>,
    vertices: &Vec<Vertex, 256>,
) -> Option<usize> {
    let given_point = points[point_index - 1];
    let given_tex_coord = tex_coords[tex_coord_index - 1];
    let given_normal = normals[normal_index - 1];

    let mut matched_index = None;

    for (i, v) in vertices.iter().enumerate() {
        if v.pos == given_point && v.tex_coords == given_tex_coord && v.normal == given_normal {
            if matched_index.is_some() {
                debug!("about to panic");
                defmt::panic!(
                    "more than one matching point for {:?} {:?} {:?}",
                    given_point,
                    given_tex_coord,
                    given_normal
                );
            }
            matched_index = Some(i);
        }
    }

    matched_index
}
