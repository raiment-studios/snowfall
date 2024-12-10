use crate::internal::*;

/// VSVec3 = Voxel-space Vector3
///
/// NOTE: it's worth considering if we can / should consolidate or take advantage
/// of bevy_math here OR if it's actually better to decouple them.  Right now
/// it is likely the snowfall engine **should** be built on top of Bevy and thus
/// not hesitate to use Bevy dependencies when it makes sense.
///
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VSVec3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl VSVec3 {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        VSVec3 { x, y, z }
    }

    /// Voxel-space from world-space coordinates
    pub fn from_ws(wx: f32, wy: f32, wz: f32) -> Self {
        VSVec3 {
            x: wx.floor() as i32,
            y: wy.floor() as i32,
            z: wz.floor() as i32,
        }
    }

    pub fn to_ws(&self) -> (f32, f32, f32) {
        (
            self.x as f32 + 0.5,
            self.y as f32 + 0.5,
            self.z as f32 + 0.5,
        )
    }

    pub fn midpoint(a: &VSVec3, b: &VSVec3) -> Self {
        VSVec3::new((a.x + b.x) / 2, (a.y + b.y) / 2, (a.z + b.z) / 2)
    }
}

impl From<(i32, i32, i32)> for VSVec3 {
    fn from(v: (i32, i32, i32)) -> Self {
        VSVec3::new(v.0, v.1, v.2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vsvec3() {
        let v = VSVec3::new(1, 2, 3);
        assert_eq!(v.x, 1);
        assert_eq!(v.y, 2);
        assert_eq!(v.z, 3);

        let v: VSVec3 = (1, 2, 3).into();
        assert_eq!(v.x, 1);
        assert_eq!(v.y, 2);
        assert_eq!(v.z, 3);
    }

    #[test]
    fn test_from_ws() {
        let v = VSVec3::from_ws(1.5, 2.5, 3.5);
        assert_eq!(v.x, 1);
        assert_eq!(v.y, 2);
        assert_eq!(v.z, 3);

        let v = VSVec3::from_ws(-0.5, 0.5, 0.0);
        assert_eq!(v.x, -1);
        assert_eq!(v.y, 0);
        assert_eq!(v.z, 0);
    }
}
