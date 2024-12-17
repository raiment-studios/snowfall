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

/// This isn't fully the generation context since generators also can
/// account for the terrain and other objects in the scene.
pub struct GenContext {
    pub generator: String,
    pub seed: u64,
    pub center: IVec3,
    pub params: serde_json::Value,
}

impl GenContext {
    pub fn new<T>(generator: T, seed: u64) -> Self
    where
        T: Into<String>,
    {
        Self {
            generator: generator.into(),
            seed,
            center: IVec3::new(0, 0, 0),
            params: serde_json::Value::Null,
        }
    }

    pub fn fork<T>(&self, generator: T, seed: u64) -> Self
    where
        T: Into<String>,
    {
        Self {
            generator: generator.into(),
            seed,
            center: self.center,
            params: self.params.clone(),
        }
    }

    pub fn params<T>(&self) -> T
    where
        T: serde::de::DeserializeOwned + Default,
    {
        let t: Result<T, _> = serde_json::from_value(self.params.clone());
        t.unwrap_or_default()
    }

    pub fn make_rng(&self) -> RNG {
        RNG::new(self.seed)
    }
}
