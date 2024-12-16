use crate::internal::*;

pub fn bresenham3d(p: IVec3, q: IVec3) -> Vec<IVec3> {
    let mut v = Vec::new();
    for (x, y, z) in line_drawing::Bresenham3d::new((p.x, p.y, p.z), (q.x, q.y, q.z)) {
        v.push(IVec3::new(x, y, z));
    }
    v
}

pub fn rotate_2d(u: f32, v: f32, angle: f32) -> (f32, f32) {
    let u2 = u * angle.cos() - v * angle.sin();
    let v2 = u * angle.sin() + v * angle.cos();
    (u2, v2)
}

pub struct Model {
    pub model: VoxelModel,
    pub position: IVec3,
}

pub struct GenContext<'a> {
    pub center: IVec3,
    pub ground_objects: Vec<&'a Model>,
    pub params: serde_json::Value,
}

impl<'a> GenContext<'a> {
    pub fn new() -> Self {
        Self {
            center: IVec3::new(0, 0, 0),
            ground_objects: Vec::new(),
            params: serde_json::Value::Null,
        }
    }

    pub fn params<T>(&self) -> T
    where
        T: serde::de::DeserializeOwned + Default,
    {
        let t: Result<T, _> = serde_json::from_value(self.params.clone());
        t.unwrap_or_default()
    }

    pub fn ground_height_at(&self, x: i32, y: i32) -> Option<i32> {
        let x = x + self.center.x;
        let y = y + self.center.y;

        let mut max_value: Option<i32> = None;
        for obj in &self.ground_objects {
            let mx = x - obj.position.x;
            let my = y - obj.position.y;
            let value = match &obj.model {
                VoxelModel::VoxelSet(m) => m.height_at(mx, my),
                VoxelModel::VoxelScene(_m) => None,
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
