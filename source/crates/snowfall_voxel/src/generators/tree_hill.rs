use crate::internal::*;

pub fn tree_hill(seed: u64, scene: &Scene2) -> VoxelScene {
    let mut rng = RNG::new(seed);

    let hill_seed = rng.range(1..8192);
    let tree_cluster_seed = rng.range(1..8192);

    let ctx = GenContext::new("small_hill", hill_seed);
    let hill = generators::small_hill(&ctx, scene);
    let mut scene = VoxelScene::new();
    scene.add_object(
        0,
        VoxelModelRef {
            model_id: "small_hill".to_string(),
            seed: hill_seed,
            position: IVec3::new(0, 0, 0),
            params: serde_json::Value::Null,
        },
    );

    let tree_cluster = generators::tree_cluster(tree_cluster_seed);
    for object in tree_cluster.layers[0].models.iter() {
        let mut p = object.position.clone();
        let z = hill.height_at(p.x, p.y).unwrap_or(0);
        p.z = z + 1;

        scene.add_object(
            1,
            VoxelModelRef {
                model_id: object.model_id.clone(),
                seed: object.seed,
                position: p,
                params: serde_json::Value::Null,
            },
        );
    }
    scene
}
