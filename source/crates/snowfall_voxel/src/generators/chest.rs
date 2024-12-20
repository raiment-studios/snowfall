use crate::internal::*;
use bevy_color::{Hsla, Srgba};

pub fn chest_cluster(ctx: &GenContext, scene: &mut Scene2) -> Group {
    let mut rng = ctx.make_rng();

    scene.terrain = generators::flat_ground(&ctx.fork("flat_ground", rng.seed8()), scene);

    let mut group = Group::new();

    let mut ctx = ctx.fork("cluster2", rng.seed8());
    ctx.params = serde_json::json!({
        "count": [40, 50],
        "range": 40,
        "closest_distance": 12.0,
        "generators": [
            (10, "chest".to_string()), //
        ],
    });
    let g = generate_model(&ctx, scene);
    group.merge(g);

    let count = 64;
    for i in 0..count {
        let angle = (i as f32) * (std::f32::consts::PI * 2.0) / (count as f32);
        let angle = angle + rng.sign() as f32 * rng.range(0.0..=0.05);
        let r = rng.range(70.0..120.0);
        let x = r * angle.cos();
        let y = r * angle.sin();
        let position = IVec3::new(x.round() as i32, y.round() as i32, 1);

        let mut ctx = ctx.fork("pine_tree", rng.seed8());
        ctx.center = position;
        ctx.params = serde_json::Value::Null;
        let voxel_set = generators::pine_tree(&ctx, scene);
        group.push(&ctx, voxel_set);
    }

    group
}

pub fn chest(ctx: &GenContext, scene: &mut Scene2) -> VoxelSet {
    let mut rng = ctx.make_rng();

    let width = rng.range(3..=5);
    let depth = 3;

    let block_rgb = |name: &str, (r, g, b)| Block::color(name, r, g, b);
    let wood_colors = {
        *rng.select(&vec![
            ((60, 50, 20), (53, 43, 16)), //
            ((40, 30, 10), (35, 28, 14)),
            ((25, 21, 10), (21, 16, 8)),
        ])
    };

    let mut voxel_set = VoxelSet::new();
    voxel_set.register_block(block_rgb("wood1", wood_colors.0));
    voxel_set.register_block(block_rgb("wood2", wood_colors.1));
    voxel_set.register_block({
        let (r, g, b) = *rng.select(&vec![wood_colors.0, wood_colors.1]);

        let scale = rng.range(0.25..=0.65);
        let r = (r as f32 * scale).round() as u8;
        let g = (g as f32 * scale).round() as u8;
        let b = (b as f32 * scale).round() as u8;
        Block::color("trim", r, g, b)
    });
    voxel_set.register_block({
        let (r, g, b) = *rng.select(&vec![
            (5, 5, 4), //
            (123, 123, 30),
            (50, 50, 50),
            (70, 66, 30),
        ]);
        Block::color("handle", r, g, b)
    });

    for dx in -width..=width as i32 {
        for dy in -depth..=depth as i32 {
            for dz in 0..=6 as i32 {
                if dz == 6 {
                    continue;
                }
                let k = (2 * depth) - dz;
                if dy.abs() > k {
                    continue;
                }

                let mut block = *rng.select(&vec!["wood1", "wood2"]);
                if dx.abs() == width && (dy.abs() == depth || dy.abs() == k || dz == 5) {
                    block = "trim";
                }
                if dz == 0 || dz == 3 {
                    block = "trim";
                }

                voxel_set.set_voxel((dx, dy, dz), block);
            }
        }
    }

    let mut rect = |cx: i32, cy: i32, cz: i32, rx: i32, ry: i32, rz: i32, block: &str| {
        let (rx, ry, rz) = (rx - 1, ry - 1, rz - 1);
        for dx in -rx..=rx {
            for dy in -ry..=ry {
                for dz in -rz..=rz {
                    voxel_set.set_voxel((cx + dx, cy + dy, cz + dz), block);
                }
            }
        }
    };

    if depth > 2 {
        rect(0, -depth, 3, 2, 1, 1, "handle");
    }
    rect(0, -depth, 2, 2, 1, 1, "handle");

    voxel_set
}
