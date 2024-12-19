use crate::internal::*;
use bevy_color::{Hsla, Srgba};

pub fn flower_field(ctx: &GenContext, scene: &mut Scene2) -> Group {
    let mut rng = ctx.make_rng();

    let mut ctx = ctx.fork("cluster2", rng.seed8());
    ctx.params = serde_json::json!({
        "count": [30, 40],
        "range": 100,
        "closest_distance": 16.0,
        "generators": [
            (10, "flower_cluster".to_string()), //
        ],
    });
    let VoxelModel::Group(group) = generate_model(&ctx, scene) else {
        panic!("expected group");
    };
    *group
}

pub fn flower_cluster(ctx: &GenContext, scene: &mut Scene2) -> Group {
    let mut rng = ctx.make_rng();

    let mut ctx = ctx.fork("cluster2", rng.seed8());
    ctx.params = serde_json::json!({
        "count": [6, 12],
        "range": 12,
        "closest_distance": 4.0,
        "generators": [
            (10, "flower".to_string()), //
        ],
    });
    let VoxelModel::Group(group) = generate_model(&ctx, scene) else {
        panic!("expected group");
    };
    *group
}

pub fn flower(ctx: &GenContext, scene: &mut Scene2) -> VoxelSet {
    let mut rng = ctx.make_rng();

    let mut voxel_set = VoxelSet::new();
    voxel_set.register_block(Block::color("flower", 160, 5, 40));

    let hue = *rng.select(&vec![9.0, 347.0, 326.0, 266.0, 47.0, 55.0, 60.0]);
    let mut flower = Hsla::new(hue, 0.81, 0.20, 1.0);
    flower.hue += rng.range(-10.0..10.0);
    flower.lightness += rng.range(-0.1..0.1);
    flower.saturation += rng.range(-0.1..0.1);
    let (r, g, b) = hsla_to_rgb(&flower);
    voxel_set.register_block(Block::color("flower", r, g, b));

    let mut green = Hsla::new(106.0, 0.67, 0.17, 1.0);
    green.hue += rng.range(-10.0..10.0);
    green.lightness += rng.range(-0.1..0.1);
    green.saturation += rng.range(-0.1..0.1);
    let (r, g, b) = hsla_to_rgb(&green);
    voxel_set.register_block(Block::color("green", r, g, b));

    voxel_set.set_voxel((0, 0, 0), "green");
    voxel_set.set_voxel((1, 0, 1), "green");
    voxel_set.set_voxel((0, 1, 1), "green");
    voxel_set.set_voxel((-1, 0, 1), "green");
    voxel_set.set_voxel((0, -1, 1), "green");
    voxel_set.set_voxel((0, 0, 1), "flower");

    voxel_set
}

pub fn hsla_to_rgb(c: &Hsla) -> (u8, u8, u8) {
    let c = Srgba::from(*c);
    (
        (c.red * 255.0).round() as u8,
        (c.green * 255.0).round() as u8,
        (c.blue * 255.0).round() as u8,
    )
}
