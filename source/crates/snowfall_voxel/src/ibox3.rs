use crate::internal::*;

/// Integer based axis-aligned bounding box. The min and max are inclusive.
///
pub struct IBox3 {
    pub min: IVec3,
    pub max: IVec3,
}

impl IBox3 {
    // ---------------------------------------------------------------------
    // Creation
    // ---------------------------------------------------------------------

    /// Creates a new empty / infinitismal bounding box
    pub fn new() -> Self {
        Self {
            min: IVec3::new(i32::MAX, i32::MAX, i32::MAX),
            max: IVec3::new(i32::MIN, i32::MIN, i32::MIN),
        }
    }

    // ---------------------------------------------------------------------
    // Properties
    // ---------------------------------------------------------------------

    pub fn volume(&self) -> i32 {
        self.length_x() * self.length_y() * self.length_z()
    }

    pub fn length_x(&self) -> i32 {
        self.max.x - self.min.x + 1
    }
    pub fn length_y(&self) -> i32 {
        self.max.y - self.min.y + 1
    }
    pub fn length_z(&self) -> i32 {
        self.max.z - self.min.z + 1
    }

    pub fn diagonal_length(&self) -> f32 {
        let dx = if self.min.x <= self.max.x {
            self.max.x - self.min.x + 1
        } else {
            return 0.0;
        };
        let dy = if self.min.y <= self.max.y {
            self.max.y - self.min.y + 1
        } else {
            return 0.0;
        };
        let dz = if self.min.z <= self.max.z {
            self.max.z - self.min.z + 1
        } else {
            return 0.0;
        };
        ((dx * dx + dy * dy + dz * dz) as f32).sqrt()
    }

    pub fn center(&self) -> IVec3 {
        (self.min + self.max) / 2
    }

    pub fn center_f32(&self) -> Vec3 {
        let min = Vec3::new(self.min.x as f32, self.min.y as f32, self.min.z as f32);
        let max = Vec3::new(
            (self.max.x + 1) as f32,
            (self.max.y + 1) as f32,
            (self.max.z + 1) as f32,
        );
        (min + max) / 2.0
    }

    // ---------------------------------------------------------------------
    // Mutation
    // ---------------------------------------------------------------------

    pub fn add(&mut self, p: IVec3) {
        self.min = self.min.min(p);
        self.max = self.max.max(p);
    }

    pub fn merge(&mut self, other: &IBox3) {
        self.min = self.min.min(other.min);
        self.max = self.max.max(other.max);
    }

    pub fn translate(&mut self, v: IVec3) {
        self.min += v;
        self.max += v;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ibox3_new() {
        let ibox = IBox3::new();
        assert_eq!(ibox.min, IVec3::new(i32::MAX, i32::MAX, i32::MAX));
        assert_eq!(ibox.max, IVec3::new(i32::MIN, i32::MIN, i32::MIN));
    }

    #[test]
    fn test_ibox3_diagonal_length() {
        let mut ibox = IBox3::new();
        assert_eq!(ibox.diagonal_length(), 0.0);

        ibox.add(IVec3::new(0, 0, 0));
        assert_eq!(ibox.diagonal_length(), 3.0_f32.powf(0.5));
    }
}
