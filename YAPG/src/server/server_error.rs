use thiserror::Error;

use crate::{components::{ComponentEnum, component_impl::ComponentError}, types::{ComponentID, PlanetID}};

#[derive(Error, Debug)]
pub enum PlanetServerError {
    #[error("Planet ID {0} not found in storage")]
    PlanetNotFound(PlanetID),

    #[error("Component ID {0} not found")]
    ComponentNotFound(ComponentID),

    #[error("Component Type {0} is not valid")]
    InvalidComponentType(ComponentEnum),

    #[error("Generation error : {0}")]
    PlanetGenerationError(ComponentError),

    #[error("Update error : {0}")]
    PlanetUpdateError(ComponentError),
}