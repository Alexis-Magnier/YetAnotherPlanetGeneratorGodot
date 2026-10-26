

use std::sync::{Mutex, RwLock};

use godot::prelude::*;
use godot::classes::{MeshInstance3D, ArrayMesh, BaseMaterial3D, Material, StandardMaterial3D, mesh};
use godot::classes::mesh::{ArrayCustomFormat, ArrayFormat};
use godot::obj::EngineBitfield;

use crate::components::geometry_generator::geometry_property::GeometryProperty;
use crate::components::ComponentImpl;
use crate::storage::node_property::NodeProperty;
use crate::storage::planet_properties_registry::PlanetPropertyRegistry;
use super::ComponentError;

use anyhow::bail;
use glam::Vec3;

#[derive(Clone)]
pub struct Parameters{
    pub material: Option<Gd<Material>>
}

impl Parameters{
    fn default() -> Self {
        Self {
            material: None
        }
    }
}

unsafe impl Sync for Parameters {}
unsafe impl Send for Parameters {}

pub struct MeshGeneratorComponent{
    pub parameters: RwLock<Parameters>,

    mesh_node: RwLock<Option<Gd<MeshInstance3D>>>
}

unsafe impl Sync for MeshGeneratorComponent {}
unsafe impl Send for MeshGeneratorComponent {}


impl MeshGeneratorComponent{
    pub fn default() -> Self{
        Self {
            parameters: RwLock::new(Parameters::default()),
            mesh_node: RwLock::new(None)
        }
    }

    fn get_mesh_node(&self, node: &mut Gd<Node>) -> anyhow::Result<Gd<MeshInstance3D>> {
        const MESH_NAME: &str = "MESH";

        if let Some(node) = node.try_get_node_as::<MeshInstance3D>(MESH_NAME) {
            return Ok(node);
        }

        if let Some(node) = node.get_node_or_null(MESH_NAME) {
            bail!(
                "Node '{}' exists but is not a MeshInstance3D (found {})",
                MESH_NAME,
                node.get_class()
            );
        }

        let mut mesh = MeshInstance3D::new_alloc();
        mesh.set_name(MESH_NAME);
        node.add_child(&mesh);

        if let Ok(mut mesh_node) = self.mesh_node.write(){
            mesh_node.replace(mesh.clone());
        }

        Ok(mesh)
    }

    fn generate_triangle_centers(data: &GeometryProperty) -> Vec<Vec3> {
        let mesh = &data.mesh;
        let positions = &data.points;
        // let height = &data.height;
        let triangle_count = mesh.num_triangles;

        (0..triangle_count).map(|t| {

            
            let a = mesh.s_begin_r(3*t) as usize;
            let b = mesh.s_begin_r(3*t+1) as usize;
            let c = mesh.s_begin_r(3*t+2) as usize;
            
            let a_pos = positions[a as usize];
            let b_pos = positions[b as usize];
            let c_pos = positions[c as usize];

            (a_pos + b_pos + c_pos) / 3.
        }).collect()
    }

    pub fn update_material(&self, material: Option<Gd<Material>>){
        if let Ok(mut parameters) = self.parameters.write() {
            parameters.material = material.clone();
        }

        if let Ok(mut mesh_lock) = self.mesh_node.write() {
            let maybe_mesh = mesh_lock.as_mut();

            if let Some(mesh) = maybe_mesh {
                match &material {
                    Some(mat) => mesh.set_material_override(mat),
                    None => mesh.set_material_override(Gd::<Material>::null_arg()),
                }
            }
        }
    }

}

impl ComponentImpl for MeshGeneratorComponent{
    fn name(&self) -> &'static str {
        return "Mesh Generator"
    }
    
    fn generate(&self, storage: &mut PlanetPropertyRegistry) -> Result<(), ComponentError> {
        let parameters = self.parameters.read().unwrap().clone();
        
        let data = storage.get::<GeometryProperty>()
            .ok_or_else(|| ComponentError::missing_property::<GeometryProperty>())?;

        let node_prop = storage.get_mut::<NodeProperty>()
            .ok_or_else(|| ComponentError::missing_property::<NodeProperty>())?;

        let mut node = node_prop.node.clone();

        let mut mesh_node = self.get_mesh_node(&mut node)
            .map_err(|err| ComponentError::failed_generation(err.to_string()))?;

        let mesh = &data.mesh;

        let mut vertices = PackedVector3Array::new();
        vertices.resize((mesh.num_sides * 3) as usize);

        let mut height = PackedFloat32Array::new();
        height.resize((mesh.num_sides * 3) as usize);
        height.fill(0.0);


        let t_center = Self::generate_triangle_centers(&data);

        for s in 0..mesh.num_sides{
            let s_idx = s as usize;

            let inner_t = mesh.s_inner_t(s) as usize;
            let outer_t = mesh.s_outer_t(s) as usize;
            let r = mesh.s_begin_r(s) as usize;

            let h = data.height[r] / 100.0;

            vertices[s_idx * 3 + 0] = Vector3::from_array(t_center[inner_t].to_array());
            vertices[s_idx * 3 + 1] = Vector3::from_array(t_center[outer_t].to_array());
            vertices[s_idx * 3 + 2] = Vector3::from_array(data.points[r].to_array());

            height[s_idx * 3 + 0] = h;
            height[s_idx * 3 + 1] = h;
            height[s_idx * 3 + 2] = h;
        }

        let mut surface_array = VarArray::new();
        surface_array.resize(mesh::ArrayType::MAX.ord() as usize, &Variant::nil());
        
        surface_array.set(mesh::ArrayType::VERTEX.ord() as usize, &vertices.to_variant());
        surface_array.set(mesh::ArrayType::NORMAL.ord() as usize, &vertices.to_variant());
        surface_array.set(mesh::ArrayType::CUSTOM0.ord() as usize, &height.to_variant());

        let custom_bits = (ArrayCustomFormat::R_FLOAT.ord() as u64) << ArrayFormat::CUSTOM0_SHIFT.ord();

        // Combine raw bitwise mask with enum flags
        let flags = EngineBitfield::from_ord(custom_bits);

        let mut array_mesh = ArrayMesh::new_gd();
        // array_mesh.add_surface_from_arrays(mesh::PrimitiveType::TRIANGLES, &surface_array);
        array_mesh.add_surface_from_arrays_ex(mesh::PrimitiveType::TRIANGLES, &surface_array)
            .flags(flags)
            .done();
        
        mesh_node.set_mesh(&array_mesh);

        if let Some(material) = &parameters.material{
            mesh_node.set_material_override(material);
        }

        Ok(())
    }
    
    fn update(&self, _storage: &mut PlanetPropertyRegistry, _dt: f32) -> Result<(), ComponentError> {
        Ok(())
    }
}
