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
        self.ground_objects
            .iter()
            .map(|m| {
                let mx = x - m.position.x;
                let my = y - m.position.y;
                match &m.model {
                    ModelType::VoxelSet(m) => m.height_at(mx, my),
                    ModelType::VoxelScene(m) => None,
                    _ => None,
                }
            })
            .max()
            .unwrap_or(None)
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

pub fn generate_model(model_id: &str, seed: u64, ctx: &GenContext) -> ModelType {
    match model_id {
        "tree1" => generators::tree1(seed).into(),
        "tree2" => generators::tree2(seed).into(),
        "pine_tree" => generators::pine_tree(seed, ctx).into(),
        "small_hill" => generators::small_hill(seed, ctx).into(),
        "fence" => generators::fence(seed, ctx).into(),

        "tree_cluster" => generators::tree_cluster(seed).into(),
        "tree_hill" => generators::tree_hill(seed).into(),

        _ => ModelType::Empty,
    }
}
