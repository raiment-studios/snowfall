use crate::internal::*;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct ClusterParams {
    count: Option<[i64; 2]>,
    range: Option<i32>,
    generators: Option<Vec<(u32, String)>>,
}

pub fn cluster2(ctx: &GenContext, scene: &mut Scene2) -> Group {
    let mut rng = ctx.make_rng();

    let mut params: ClusterParams = ctx.params();
    let count_range = params.count.get_or_insert([12, 24]);
    let range = *params.range.get_or_insert(48);
    let genlist = params
        .generators
        .get_or_insert(vec![
            (10, "tree1".to_string()), //
            (10, "tree2".to_string()),
            (80, "pine_tree".to_string()),
        ])
        .clone();

    const MAX_ATTEMPTS: usize = 128;
    const CLOSEST_DISTANCE: f32 = 12.0;
    let mut count = rng.range(count_range[0]..=count_range[1]);

    let mut group = Group::new();

    let mut point_set = PointSet::new();
    for _ in 0..MAX_ATTEMPTS {
        let position = IVec3::new(rng.range(-range..=range), rng.range(-range..=range), 0);
        let mut position = position + ctx.center;
        position.z = scene.terrain.height_at(position.x, position.y).unwrap_or(1);

        //
        // Reject the position if the nearest distance is too close to another tree
        // in the cluster (perhaps this should be too close to **any** object?) OR
        // if the block it would be placed on is marked as occupied already.
        //
        let d = point_set.nearest_distance_2d(&position).unwrap_or(f32::MAX);
        if d < CLOSEST_DISTANCE {
            continue;
        }
        if let Some(block) = scene.terrain.top_block_at(position.x, position.y) {
            if block.occupied {
                continue;
            }
        }

        point_set.add(position);

        let model_id = rng.select_weighted(&genlist);
        let seed = rng.seed8();
        let mut ctx = ctx.fork(model_id.clone(), seed);
        ctx.center = position;
        let VoxelModel::VoxelSet(voxel_set) = generate_model(&ctx, scene) else {
            panic!("expected voxel set");
        };

        group.objects.push(Object {
            generator_id: model_id.to_string(),
            seed,
            params: serde_json::Value::Null,
            position,
            imp: ObjectImp::VoxelSet(voxel_set),
        });

        count -= 1;
        if count == 0 {
            break;
        }
    }
    group
}
