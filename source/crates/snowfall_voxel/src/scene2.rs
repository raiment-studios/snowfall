use crate::internal::*;

pub struct Scene2 {
    pub terrain: VoxelSet, // Eventually EditableVoxel
    pub root: Object,
}

impl Scene2 {
    pub fn new() -> Self {
        Self {
            terrain: VoxelSet::new(),
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

pub trait IntoObjectImp {
    fn into_object_imp(self, ctx: &GenContext) -> ObjectImp;
}

impl IntoObjectImp for VoxelSet {
    fn into_object_imp(self, _ctx: &GenContext) -> ObjectImp {
        ObjectImp::VoxelSet(Box::new(self))
    }
}

pub struct Group {
    pub objects: Vec<Object>,
}

impl Group {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn push<T>(&mut self, ctx: &GenContext, model: T)
    where
        T: IntoObjectImp,
    {
        self.objects.push(Object {
            generator_id: ctx.generator.clone(),
            seed: ctx.seed,
            position: ctx.center,
            params: ctx.params.clone(),
            imp: model.into_object_imp(ctx),
        });
    }

    pub fn merge(&mut self, model: VoxelModel) {
        match model {
            VoxelModel::Group(model_group) => {
                for object in model_group.objects {
                    self.objects.push(object);
                }
            }
            _ => panic!("expected group"),
        }
    }
}

pub enum ObjectImp {
    Empty,
    Stub,
    Actor(Box<dyn Actor>),
    VoxelSet(Box<VoxelSet>),
    Group(Box<Group>),
}

impl ObjectImp {
    pub fn type_str(&self) -> &'static str {
        match self {
            ObjectImp::Empty => "Empty",
            ObjectImp::Stub => "Stub",
            ObjectImp::Actor(_) => "Actor",
            ObjectImp::VoxelSet(_) => "VoxelSet",
            ObjectImp::Group(_) => "Group",
        }
    }
}

pub trait Actor {
    fn update(&mut self, scene: &mut Scene2);
}
