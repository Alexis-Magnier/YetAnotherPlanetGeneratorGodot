use godot::prelude::*;

#[derive(GodotClass)]
#[class(init)]
struct Test{}

#[godot_api]
impl Test{

    #[func]
    pub fn hello_world(){
        godot_print!("Hello world !");
    }
}
