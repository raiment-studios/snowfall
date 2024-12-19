use crate::internal::*;

pub fn desolate_hill(ctx: &GenContext, scene: &mut Scene2) -> Group {
    let mut rng = ctx.make_rng();

    let mut ctx = ctx.fork("hill4", rng.seed8());
    ctx.params = serde_json::json!({
        "ground_type": "dirt",
    });
    scene.terrain = generators::hill4(&ctx, scene);

    generators::rocks(&ctx.fork("rocks", rng.seed8()), scene);

    for _ in 0..2 {
        generators::road(&ctx.fork("road", rng.seed8()), scene);
    }

    let mut group = Group::new();

    const R: i32 = 255 - 32;
    for _ in 0..32 {
        let mut ctx = ctx.fork("cluster2", rng.seed8());
        ctx.center = IVec3::new(rng.range(-R..=R), rng.range(-R..=R), 0);
        ctx.params = serde_json::json!({
            "count": [3, 8],
            "range": 32,
            "generators": [
                (10, "bare_tree".to_string()), //
            ],
        });
        let g: VoxelModel = generate_model(&ctx, scene);
        let VoxelModel::Group(g) = g else {
            panic!("expected group");
        };
        for object in g.objects {
            group.objects.push(object);
        }
    }
    group
}
