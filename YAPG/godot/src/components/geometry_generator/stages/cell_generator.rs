use godot::prelude::*;
use crate::delegate_inner;

#[derive(GodotClass)]
#[class(base=RefCounted, init, rename=__YAPG_Geometry_Generator_Cell_Generator_Impl)]
struct CellGenerator{
    pub inner: yapg_stages::geometry_generator::stages::cell_generator::CellGenerator,

    base: Base<RefCounted>
}

delegate_inner! {
    CellGenerator {
        cell_count: u32,
        jitter: f32,

        seed: i64 => {
            get => |this| this.inner.seed as i64,
            set => |this, val| this.inner.seed = val.max(0) as u64
        },
    }
}