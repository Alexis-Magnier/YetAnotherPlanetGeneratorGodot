use godot::prelude::*;
use crate::delegate_inner;

#[derive(GodotClass)]
#[class(base=RefCounted, init, rename=__YAPG_Geometry_Generator_Distances_Impl)]
struct Distances{
    pub inner: yapg_stages::geometry_generator::stages::distances::AssignDistances,
    base: Base<RefCounted>
}

delegate_inner! {
    Distances {}
}