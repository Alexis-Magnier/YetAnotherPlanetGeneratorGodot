use godot::prelude::*;


#[derive(Debug)]
pub struct NodeProperty{
    pub node: Gd<Node>
}

impl NodeProperty{
    pub fn new(node: Gd<Node>) -> Self{
        Self {
            node
        }
    }
}

unsafe impl Send for NodeProperty {}
unsafe impl Sync for NodeProperty {}