use super::mesh::Mesh;
use glam::Vec3;


#[derive(Debug)]
pub struct GeometryProperty{
    pub points: Vec<Vec3>,
    pub height: Vec<f32>,
    pub mesh: Mesh
}

impl GeometryProperty{
    pub fn default() -> Self {
        Self {
            points: Vec::default(),
            height: Vec::default(),
            mesh: Mesh::default()
        }
    }
}


unsafe impl Sync for GeometryProperty {}
unsafe impl Send for GeometryProperty {}