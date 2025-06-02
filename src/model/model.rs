use crate::types::vector::{Vec2, Vec3};
use heapless::Vec;
use log::info;

/// Represents a 3D model.
pub struct Model<'a> {
    pub verts: Vec<VertexData, 4096>,
    pub faces: Vec<FaceData<'a>, 8192>
}

/// Represents a vertex. Contains 
/// model-space position, texture
/// coordinates, and normals.
pub struct VertexData {
    pub pos: Vec3<f32>,
    pub tex_coords: Vec2<f32>,
    pub normal: Vec3<f32>
}

/// Represents a face. Contains
/// 3 composing vertices.
pub struct FaceData<'a> {
    pub verts: [&'a VertexData; 3],
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

    while let Some(i) = lines.next() {
        info!("{:?}", i);
        let mut parts = i.split(" ");
        info!("Part: {:?}", parts);

        let token: Option<&str> = parts.next();

        if token == Some("v") {
            points.push(Vec3{x: parts.next().expect("value in vertex is not float").parse().unwrap(), y: parts.next().expect("value in vertex is not float").parse().unwrap(), z: parts.next().expect("value in vertex is not float").parse().unwrap()});
            info!("Added point {} {} {}", points.last().unwrap().x, points.last().unwrap().y, points.last().unwrap().z)
        } else if token == Some("vt") {
            tex_coords.push(Vec2{x: parts.next().expect("value in tex coords is not float").parse().unwrap(), y: parts.next().expect("value in tex coords is not float").parse().unwrap()});
            info!("Added tex coord {} {}", tex_coords.last().unwrap().x, tex_coords.last().unwrap().y)
        } else if token == Some("vn") {
            normals.push(Vec3{x: parts.next().expect("value in vertex normal is not float").parse().unwrap(), y: parts.next().expect("value in vertex normal is not float").parse().unwrap(), z: parts.next().expect("value in vertex normal is not float").parse().unwrap()});
            info!("Added vertex normal {} {} {}", normals.last().unwrap().x, normals.last().unwrap().y, normals.last().unwrap().z)
        }
    }

    Model { verts: vertices, faces: faces}
}