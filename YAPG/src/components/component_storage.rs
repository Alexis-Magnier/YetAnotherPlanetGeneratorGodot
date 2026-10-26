use std::sync::Arc;

use super::component_impl::ComponentImpl;
use super::ComponentEnum;

use crate::types::PlanetID;


pub struct Component{
    pub planet: PlanetID,
    pub enum_type: ComponentEnum,
    pub implementation: Arc<dyn ComponentImpl>
}