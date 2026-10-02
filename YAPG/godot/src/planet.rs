use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=RefCounted, init, rename=__YAPG_Planet_Impl)]
struct Planet{

    base: Base<RefCounted>
}

#[godot_api]
impl Planet{
    #[func]
    pub fn hello_world(){
        godot_print!("Hello world !");
    }
}
