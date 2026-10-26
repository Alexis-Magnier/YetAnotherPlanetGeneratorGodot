use std::sync::RwLock;

use godot::global::godot_print;

use crate::components::ComponentImpl;
use crate::storage::planet_properties_registry::PlanetPropertyRegistry;
use super::ComponentError;
use super::GeometryProperty;


use super::stages::{
    cell_generator::CellGenerator,
    plate_generator::PlateGenerator,
    detect_collisions::CollisionDetection,
    distances::AssignDistances,
    base_properties::AssignBaseProperties,
    temperature::Temperatures,
    strength::Strength,
    stress::Stress,
    isostasy::Isostasy
};


#[derive(Clone, Debug)]
pub struct Parameters{
    pub cell_generator: CellGenerator,
    pub plate_generator: PlateGenerator,
    pub collision_detection: CollisionDetection,
    pub distances: AssignDistances,
    pub base_properties: AssignBaseProperties,
    pub temperatures: Temperatures,
    pub strength: Strength,
    pub stress: Stress,
    pub isostasy: Isostasy,
}

impl Parameters{
    pub fn default() -> Self {
        Self {
            cell_generator: CellGenerator::default(),
            plate_generator: PlateGenerator::default(),
            collision_detection: CollisionDetection::default(),
            distances: AssignDistances::default(),
            base_properties: AssignBaseProperties::default(),
            temperatures: Temperatures::default(),
            strength: Strength::default(),
            stress: Stress::default(),
            isostasy: Isostasy::default()
        }
    }

    pub fn run(&self, mut data: &mut PlanetData) -> anyhow::Result<()>{

        self.cell_generator.run(&mut data)?;
        self.plate_generator.run(&mut data)?;
        self.collision_detection.run(&mut data)?;
        self.distances.run(&mut data)?;
        self.base_properties.run(&mut data)?;
        self.temperatures.run(&mut data)?;
        self.strength.run(&mut data)?;
        self.stress.run(&mut data)?;
        self.isostasy.run(&mut data)?;
        
        Ok(())
    }
}

pub struct GeometryGeneratorComponent{
    pub parameters: RwLock<Parameters>
}


impl GeometryGeneratorComponent{
    pub fn default() -> Self{
        Self {
            parameters: RwLock::new(Parameters::default())
        }
    }

    fn generate(&self) -> anyhow::Result<PlanetData> {
        let parameters = self.parameters.read().unwrap().clone();

        let mut data = PlanetData::default();

        parameters.run(&mut data)?;

        Ok(data)
    }
}

use super::data::PlanetData;

impl ComponentImpl for GeometryGeneratorComponent{
    fn name(&self) -> &'static str {
        return "Geometry Generator"
    }
    
    fn generate(&self, storage: &mut PlanetPropertyRegistry) -> Result<(), ComponentError> {

        let data = self.generate()
            .map_err(|err| ComponentError::failed_generation(err.to_string()))?;

        storage.insert::<GeometryProperty>(
            GeometryProperty{
                points: data.cells.position,
                height: data.cells.height,
                mesh: data.mesh
            }
        );
        
        Ok(())
    }
    
    fn update(&self, _storage: &mut PlanetPropertyRegistry, _dt: f32) -> Result<(), ComponentError> {
        Ok(())
    }
}
