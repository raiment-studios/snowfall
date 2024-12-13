use crate::internal::*;

pub fn tree_cluster(seed: u64) -> VoxelScene {
    let mut rng = RNG::new(seed);

    const MAX_ATTEMPTS: usize = 128;
    const CLOSEST_DISTANCE: f32 = 12.0;
    const RANGE: i32 = 48;
    let mut count = rng.range(12..=24);

    let mut scene = VoxelScene::new();

    let mut point_set = PointSet::new();
    for _ in 0..MAX_ATTEMPTS {
        let model_id = *rng.select_weighted(&vec![
            (10, "tree1"), //
            (10, "tree2"),
            (80, "pine_tree"),
        ]);
        let seed = rng.range(1..8192);
        let position = IVec3::new(rng.range(-RANGE..=RANGE), rng.range(-RANGE..=RANGE), 0);

        let d = point_set.nearest_distance(&position).unwrap_or(f32::MAX);
        if d < CLOSEST_DISTANCE {
            continue;
        }

        point_set.add(position);
        scene.add_object(
            0,
            Object {
                model_id: model_id.to_string(),
                seed,
                position,
            },
        );
        count -= 1;
        if count == 0 {
            break;
        }
    }
    scene
}
