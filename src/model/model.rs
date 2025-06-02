use crate::types::vector::{Vec2, Vec3};
use heapless::Vec;

/// Represents a 3D model.
pub struct Model {
    pub verts: Vec<VertexData, 4096>,
    pub faces: Vec<FaceData, 8192>
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
pub struct FaceData {
    pub verts: [&VertexData; 3],
}

/// Creates a Model from an OBJ
/// file.
pub fn from_obj(obj_file: &str) -> Model {
    let points: Vec<Vec3<f32>, 4096>;
    let normals: Vec<Vec3<f32>, 4096>;
    let tex_coords: Vec<Vec2<f32>, 4096>;
    let lines: core::str::Split<'_, &'static str> = obj_file.split("\n");

}