use crate::internal::*;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct ClusterParams {
    count: Option<[i64; 2]>,
    range: Option<i32>,
}

pub fn cluster(seed: u64, ctx: &GenContext) -> VoxelScene {
    let mut rng = RNG::new(seed);

    let mut params: ClusterParams = ctx.params();
    let count_range = params.count.get_or_insert([12, 24]);
    let range = *params.range.get_or_insert(48);

    const MAX_ATTEMPTS: usize = 128;
    const CLOSEST_DISTANCE: f32 = 12.0;
    let mut count = rng.range(count_range[0]..=count_range[1]);

    let mut scene = VoxelScene::new();

    let mut point_set = PointSet::new();
    for _ in 0..MAX_ATTEMPTS {
        let model_id = *rng.select_weighted(&vec![
            (10, "tree1"), //
            (10, "tree2"),
            (80, "pine_tree"),
        ]);
        let seed = rng.range(1..8192);
        let position = IVec3::new(rng.range(-range..=range), rng.range(-range..=range), 0);

        let d = point_set.nearest_distance(&position).unwrap_or(f32::MAX);
        if d < CLOSEST_DISTANCE {
            continue;
        }

        point_set.add(position);
        scene.add_object(
            0,
            VoxelModelRef {
                model_id: model_id.to_string(),
                seed,
                position,
                params: serde_json::Value::Null,
            },
        );
        count -= 1;
        if count == 0 {
            break;
        }
    }
    scene
}
