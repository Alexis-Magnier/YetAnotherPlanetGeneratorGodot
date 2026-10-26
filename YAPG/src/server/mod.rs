use godot::{classes::Engine, prelude::*};

use crate::server::yapg_server::YAPGServer;

pub mod server_error;
pub mod yapg_server;
pub mod yapg_server_impl;

const SINGLETONE_NAME:&str = "YAPGServer";

pub fn register_server() {
    let server = YAPGServer::new_alloc();

    Engine::singleton().register_singleton(
        SINGLETONE_NAME,
        &server,
    );
}

pub fn unregister_server(){
    let mut engine = Engine::singleton();
    let name = StringName::from(SINGLETONE_NAME);
    
    if let Some(server) = engine.get_singleton(&name) {
        engine.unregister_singleton(&name);
        server.free();
    }
}