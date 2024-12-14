use serde::de::value;

use crate::internal::*;

pub fn bresenham3d(p: IVec3, q: IVec3) -> Vec<IVec3> {
    let mut v = Vec::new();
    for (x, y, z) in line_drawing::Bresenham3d::new((p.x, p.y, p.z), (q.x, q.y, q.z)) {
        v.push(IVec3::new(x, y, z));
    }
    v
}

pub struct Model {
    pub model: ModelType,
    pub position: IVec3,
}

pub struct GenContext<'a> {
    pub center: IVec3,
    pub ground_objects: Vec<&'a Model>,
}

impl<'a> GenContext<'a> {
    pub fn new(center: IVec3) -> Self {
        Self {
            center,
            ground_objects: Vec::new(),
        }
    }

    pub fn ground_height_at(&self, x: i32, y: i32) -> Option<i32> {
        let x = x + self.center.x;
        let y = y + self.center.y;

        let mut max_value: Option<i32> = None;
        for obj in &self.ground_objects {
            let mx = x - obj.position.x;
            let my = y - obj.position.y;
            let value = match &obj.model {
                ModelType::VoxelSet(m) => m.height_at(mx, my),
                ModelType::VoxelScene(_m) => None,
                _ => None,
            };
            let Some(value) = value else {
                continue;
            };
            max_value = Some(max_value.unwrap_or(value).max(value));
        }
        max_value
    }
}

pub enum ModelType {
    Empty,
    VoxelSet(Box<VoxelSet>),
    VoxelScene(Box<VoxelScene>),
}

impl Into<ModelType> for VoxelSet {
    fn into(self) -> ModelType {
        ModelType::VoxelSet(Box::new(self))
    }
}

impl Into<ModelType> for VoxelScene {
    fn into(self) -> ModelType {
        ModelType::VoxelScene(Box::new(self))
    }
}
