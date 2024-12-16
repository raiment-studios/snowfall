use crate::internal::*;

pub struct Scene2 {
    pub terrain: VoxelGrid, // Eventually EditableVoxel
    pub root: Object,
}

impl Scene2 {
    pub fn new() -> Self {
        Self {
            terrain: VoxelGrid::new(),
            root: Object {
                generator_id: "".to_string(),
                seed: 0,
                params: serde_json::Value::Null,
                position: IVec3::ZERO.clone(),
                imp: ObjectImp::Empty,
            },
        }
    }
}

pub struct Object {
    pub generator_id: String,
    pub seed: u64,
    pub params: serde_json::Value,
    pub position: IVec3,

    pub imp: ObjectImp,
}

pub struct Group {
    pub objects: Vec<Object>,
}

impl Group {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }
}

pub enum ObjectImp {
    Empty,
    Stub,
    Actor(Box<dyn Actor>),
    VoxelSet(Box<VoxelSet>),
    Group(Box<Group>),
}

pub trait Actor {
    fn update(&mut self, scene: &mut Scene2);
}
