use crate::internal::*;

pub fn hill_with_road(ctx: &GenContext, scene: &mut Scene2) -> Group {
    let mut rng = ctx.make_rng();

    scene.terrain = generators::hill4(&ctx.fork("hill", rng.seed8()), scene);
    for _ in 0..4 {
        generators::road(&ctx.fork("road", rng.seed8()), scene);
    }

    let mut group = Group::new();

    let model = generate_model(&ctx.fork("flower_field", rng.seed8()), scene);
    group.merge(model);

    for _ in 0..4 {
        let seed = rng.seed8();
        let mut ctx = ctx.fork("cluster2", seed);
        ctx.params = serde_json::json!({
            "count": [120, 400],
            "range": 248,
        });
        let model = generate_model(&ctx, scene);
        group.merge(model);
    }

    let model = generate_model(&ctx.fork("cloud_cluster", rng.seed8()), scene);
    group.merge(model);

    group
}
