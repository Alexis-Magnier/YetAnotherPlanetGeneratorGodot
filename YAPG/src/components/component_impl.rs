use crate::storage::planet_properties_registry::PlanetPropertyRegistry;
use downcast_rs::{DowncastSync, impl_downcast};
use thiserror::Error;


#[derive(Error, Debug)]
pub enum ComponentError {
    #[error("Property missing in registry: {property_name} (type_id: {type_id:?})")]
    MissingProperty {
        property_name: &'static str,
        type_id: std::any::TypeId,
    },

    #[error("Failed during generation: {reason}")]
    GenerationFailed {
        reason: String,
    },

    #[error("Internal component error: {0}")]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}

impl ComponentError{
    pub fn missing_property<T: 'static>() -> Self {
        Self::MissingProperty {
            property_name: std::any::type_name::<T>(),
            type_id: std::any::TypeId::of::<T>(),
        }
    }

    pub fn failed_generation(reason: String) -> Self{
        Self::GenerationFailed {
            reason
        }
    }
}

pub trait ComponentImpl: DowncastSync{
    fn name(&self) -> &'static str;
    fn generate(&self, _storage: &mut PlanetPropertyRegistry) -> Result<(), ComponentError> { Ok(()) }
    fn update(&self, _storage: &mut PlanetPropertyRegistry, _dt: f32) -> Result<(), ComponentError> { Ok(()) }
    fn cleanup(&self, _storage: &mut PlanetPropertyRegistry) -> Result<(), ComponentError> { Ok(()) }
}

impl_downcast!(sync ComponentImpl);