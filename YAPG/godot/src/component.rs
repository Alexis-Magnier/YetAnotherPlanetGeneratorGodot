use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node, init, tool, rename=__YAPG_Planet_Component)]
struct PlanetComponent{
    base: Base<Node>
}

#[godot_api]
impl PlanetComponent{
    #[func]
    pub fn hello_world(){
        godot_print!("Hello world !");
    }
}