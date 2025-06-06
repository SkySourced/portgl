use crate::types::vector::{Vec2, Vec3};
use core::fmt::Write;
use heapless::Vec;
use log::info;

/// Represents a 3D model.
pub struct Model<'a> {
    pub verts: Vec<VertexData, 4096>,
    pub faces: Vec<FaceData<'a>, 8192>,
}

/// Represents a vertex. Contains
/// model-space position, texture
/// coordinates, and normals.
pub struct VertexData {
    pub pos: Vec3<f32>,
    pub tex_coords: Vec2<f32>,
    pub normal: Vec3<f32>,
}

impl core::fmt::Debug for VertexData {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("VertexData")
            .field("pos", &self.pos)
            .field("tex_coords", &self.tex_coords)
            .field("normal", &self.normal)
            .finish()
    }
}

/// Represents a face. Contains
/// 3 composing vertices.
pub struct FaceData<'a> {
    pub verts: [&'a VertexData; 3],
}

impl<'a> FaceData<'a> {
    /// Creates a new face initialized to all zeroes.
    pub fn new() -> FaceData<'a> {
        FaceData {
            verts: [
                &VertexData {
                    pos: Vec3 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    tex_coords: Vec2 { x: 0.0, y: 0.0 },
                    normal: Vec3 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                },
                &VertexData {
                    pos: Vec3 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    tex_coords: Vec2 { x: 0.0, y: 0.0 },
                    normal: Vec3 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                },
                &VertexData {
                    pos: Vec3 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    tex_coords: Vec2 { x: 0.0, y: 0.0 },
                    normal: Vec3 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                },
            ],
        }
    }
}

/// Creates a Model from an OBJ
/// file.
pub fn from_obj(obj_file: &str) -> Model {
    let mut points: Vec<Vec3<f32>, 4096> = Vec::new();
    let mut normals: Vec<Vec3<f32>, 4096> = Vec::new();
    let mut tex_coords: Vec<Vec2<f32>, 4096> = Vec::new();

    let mut lines: core::str::Split<'_, &'static str> = obj_file.split("\n");

    let mut vertices: Vec<VertexData, 4096> = Vec::new();
    let mut faces: Vec<FaceData, 8192> = Vec::new();

    let mut context = heapless::String::<16>::new();

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

                face.verts[i] = match get_vertex(
                    point_index,
                    tex_coord_index,
                    normal_index,
                    &points,
                    &tex_coords,
                    &normals,
                    &vertices,
                ) {
                    Some(i) => i,
                    None => {
                        vertices.push(VertexData {
                            pos: points[point_index].clone(),
                            tex_coords: tex_coords[tex_coord_index].clone(),
                            normal: normals[normal_index].clone(),
                        });
                        &vertices
                            .last()
                            .expect("there should be a vertex to retrieve")
                    }
                }

                // //TODO: repeating vertices
                // vertices
                //     .push(VertexData {
                //         pos: points[parse_size(vertex_components.next(), &context)],
                //         tex_coords: tex_coords[parse_size(vertex_components.next(), &context)],
                //         normal: normals[parse_size(vertex_components.next(), &context)],
                //     })
                //     .expect("faces should only have three vertices");
                // face.verts[i] = vertices.last().expect("vertex array should not be empty")
            }
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
fn get_vertex<'a>(
    point_index: usize,
    tex_coord_index: usize,
    normal_index: usize,
    points: &Vec<Vec3<f32>, 4096>,
    tex_coords: &Vec<Vec2<f32>, 4096>,
    normals: &Vec<Vec3<f32>, 4096>,
    vertices: &'a Vec<VertexData, 4096>,
) -> Option<&'a VertexData> {
    let given_point: Vec3<f32> = points[point_index];
    let given_tex_coord: Vec2<f32> = tex_coords[tex_coord_index];
    let given_normal: Vec3<f32> = normals[normal_index];

    let matching_points = vertices.iter().filter(|v: &&VertexData| {
        v.pos == given_point && v.tex_coords == given_tex_coord && v.normal == given_normal
    });
    let num_matched_points = matching_points.clone().count();
    if num_matched_points > 1 {
        panic!(
            "more than one matching point for {:?} {:?} {:?}",
            given_point, given_tex_coord, given_normal
        )
    } else if num_matched_points == 1 {
        Some(
            matching_points
                .last()
                .expect("should be at least one matched point"),
        )
    } else {
        None
    }
}
