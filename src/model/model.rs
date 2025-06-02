use crate::types::vector::{Vec2, Vec3};

/// Represents a 3D model.
pub struct Model<'a> {
    pub verts: Vec<VertexData>,
    pub faces: Vec<FaceData<'a>>
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