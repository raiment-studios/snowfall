use crate::internal::*;

pub struct PointSet<T>
where
    T: Point3D,
{
    pub points: Vec<T>,
}

pub trait Point3D {
    fn distance_to(&self, other: &Self) -> f32;
}

impl Point3D for IVec3 {
    fn distance_to(&self, other: &Self) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        ((dx * dx + dy * dy + dz * dz) as f32).sqrt()
    }
}

impl<T> PointSet<T>
where
    T: Point3D,
{
    pub fn new() -> Self {
        Self { points: Vec::new() }
    }

    pub fn add(&mut self, point: T) {
        self.points.push(point);
    }

    // TODO: in theory, it may be sense to use a optimized
    // data structure if the point set size is large + static.
    pub fn nearest(&self, point: &T) -> Option<&T> {
        let mut min_distance = f32::MAX;
        let mut value: Option<&T> = None;
        for p in self.points.iter() {
            let distance = p.distance_to(&point);
            if distance < min_distance {
                min_distance = distance;
                value = Some(p);
            }
        }
        value
    }

    pub fn nearest_distance(&self, point: &T) -> Option<f32> {
        self.nearest(point).map(|p| p.distance_to(&point))
    }
}
