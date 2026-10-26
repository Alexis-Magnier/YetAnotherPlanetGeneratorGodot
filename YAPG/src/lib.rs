pub mod types;
pub mod yapg;
pub mod storage;
pub mod components;
pub mod server;
pub mod planet;

use godot::prelude::*;

struct YAPG;

#[gdextension]
unsafe impl ExtensionLibrary for YAPG {
    fn min_level() -> InitLevel {
        InitLevel::Servers
    }

    fn on_stage_init(level: InitStage) {
        match level {
            InitStage::Servers => {
                server::register_server();
            }
            _ => (),
        }
    }

    fn on_stage_deinit(level: InitStage) {
        match level {
            InitStage::Servers => {
                server::unregister_server();
            }
            _ => (),
        }
    }
}