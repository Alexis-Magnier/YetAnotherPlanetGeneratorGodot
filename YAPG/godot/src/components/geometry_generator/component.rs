use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Resource, init, rename=__YAPG_Geometry_Generator_Component_Stage_Base)]
struct StageBase{
    base: Base<Resource>
}

#[derive(GodotClass)]
#[class(base=RefCounted, init, rename=__YAPG_Geometry_Generator_Component_Impl)]
struct GeometryGeneratorComponent{
    base: Base<RefCounted>
}

#[godot_api]
impl GeometryGeneratorComponent{
    #[func]
    pub fn hello_world(){
        godot_print!("Hello world !");
    }
}
