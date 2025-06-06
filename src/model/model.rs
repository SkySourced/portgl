use crate::types::vector::{Vec2, Vec3};
use core::fmt::{Debug, Write};
use heapless::Vec;
use log::info;

#[derive(Debug)]
/// Represents a 3D model.
pub struct Model {
    pub verts: Vec<VertexData, 4096>,
    pub faces: Vec<FaceData, 8192>,
}

#[derive(Debug, Clone, Copy)]
/// Represents a vertex. Contains
/// model-space position, texture
/// coordinates, and normals.
pub struct VertexData {
    pub pos: Vec3<f32>,
    pub tex_coords: Vec2<f32>,
    pub normal: Vec3<f32>,
}

#[derive(Debug, Clone, Copy)]
/// Represents a face. Contains
/// indices of 3 composing vertices.
pub struct FaceData {
    pub verts: [usize; 3],
}

impl FaceData {
    /// Creates a new face initialized to all zeroes.
    pub fn new() -> FaceData {
        FaceData { verts: [0, 0, 0] }
    }
}

/// Creates a Model from an OBJ
/// file.
pub fn from_obj(obj_file: &str) -> Model {
    info!("Creating model");

    let mut points: Vec<Vec3<f32>, 4096> = Vec::new();
    let mut normals: Vec<Vec3<f32>, 4096> = Vec::new();
    let mut tex_coords: Vec<Vec2<f32>, 4096> = Vec::new();

    info!("Created vecs");

    let mut lines: core::str::Split<'_, &'static str> = obj_file.split("\n");

    info!("Split lines");

    let mut vertices: Vec<VertexData, 4096> = Vec::new();
    let mut faces: Vec<FaceData, 8192> = Vec::new();

    info!("Created vecs 2");

    let mut context = heapless::String::<16>::new();

    info!("Created context");

    while let Some(i) = lines.next() {
        info!("{:?}", i);
        let mut parts = i.split(" ");
        info!("Part: {:?}", parts);

        let token: Option<&str> = parts.next();

        if token == Some("v") {
            context.clear();
            context
                .write_str("vertex")
                .expect("model parse context should be less than 16 chars");

            points
                .push(Vec3 {
                    x: parse_float(parts.next(), &context),
                    y: parse_float(parts.next(), &context),
                    z: parse_float(parts.next(), &context),
                })
                .expect("loaded model should not contain more than 4096 points");

            info!(
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

            tex_coords
                .push(Vec2 {
                    x: parse_float(parts.next(), &context),
                    y: parse_float(parts.next(), &context),
                })
                .expect("loaded model should not contain more than 4096 tex coords");

            info!(
                "Added tex coord {} {}",
                tex_coords.last().unwrap().x,
                tex_coords.last().unwrap().y
            )
        } else if token == Some("vn") {
            context.clear();
            context
                .write_str("vertex normal")
                .expect("model parse context should be less than 16 chars");

            normals
                .push(Vec3 {
                    x: parse_float(parts.next(), &context),
                    y: parse_float(parts.next(), &context),
                    z: parse_float(parts.next(), &context),
                })
                .expect("loaded model should not contain more than 4096 normals");

            info!(
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

            let mut face: FaceData = FaceData::new();

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
                        vertices
                            .push(VertexData {
                                pos: points[point_index].clone(),
                                tex_coords: tex_coords[tex_coord_index].clone(),
                                normal: normals[normal_index].clone(),
                            })
                            .expect("model should not contain more than 4096 unique vertices");
                        vertices.len() - 1
                    }
                };
            }

            faces.push(face).expect("model should not contain more than 8192 unique faces");
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

fn parse_size(word: Option<&'_ str>, context: &heapless::String<16>) -> usize {
    let mut error1 = heapless::String::<32>::new();
    write!(error1, "{} value not found", context).unwrap();
    let mut error2 = heapless::String::<41>::new();
    write!(error2, "value in {} should be usize", context).unwrap();

    word.expect(error1.as_str())
        .parse::<usize>()
        .expect(error2.as_str())
}

///
fn get_vertex(
    point_index: usize,
    tex_coord_index: usize,
    normal_index: usize,
    points: &Vec<Vec3<f32>, 4096>,
    tex_coords: &Vec<Vec2<f32>, 4096>,
    normals: &Vec<Vec3<f32>, 4096>,
    vertices: &Vec<VertexData, 4096>,
) -> Option<usize> {
    let given_point = points[point_index];
    let given_tex_coord = tex_coords[tex_coord_index];
    let given_normal = normals[normal_index];

    let mut matched_index = None;

    for (i, v) in vertices.iter().enumerate() {
        if v.pos == given_point && v.tex_coords == given_tex_coord && v.normal == given_normal {
            if matched_index.is_some() {
                panic!(
                    "more than one matching point for {:?} {:?} {:?}",
                    given_point, given_tex_coord, given_normal
                );
            }
            matched_index = Some(i);
        }
    }

    matched_index
}
