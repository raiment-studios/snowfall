use crate::internal::*;

pub fn bare_tree(ctx: &GenContext, _scene: &Scene2) -> VoxelSet {
    let mut rng = ctx.make_rng();

    let mut trunk = VoxelSet::new();
    trunk.register_block(Block::color("brown1", 60, 40, 20));
    trunk.register_block(Block::color("brown2", 30, 20, 5));
    trunk.register_block(Block::color("brown3", 22, 15, 4));
    trunk.register_block(Block::color("c1", 255, 0, 0));
    trunk.register_block(Block::color("c2", 0, 255, 0));
    trunk.register_block(Block::color("c3", 0, 0, 255));
    trunk.register_block(Block::color("c4", 255, 255, 0));
    trunk.register_block(Block::color("c5", 255, 0, 255));
    trunk.register_block(Block::color("c6", 0, 255, 255));

    let mut base_points = vec![(IVec3::new(0, 0, 0), 10)];

    let mut segment = 0;
    while !base_points.is_empty() {
        let set = base_points.drain(..).collect::<Vec<_>>();
        for (p, h) in set {
            let mut branches = Vec::new();

            let len = h - rng.range(1..=2);
            if len <= 2 {
                continue;
            }

            let bcount = match segment {
                0 => 1,
                1 => 2,
                _ => rng.range(1..=(1 + segment)),
            };
            if segment >= 2 && rng.range(0..100) < 10 {
                continue;
            }
            for _ in 0..bcount {
                let dirs = vec![(1, 0), (0, 1), (1, 1)];
                let mut dir = *rng.select(&dirs);
                let sign = *rng.select(&vec![-1, 1]);
                let scale = rng.range(1..=(1 + segment * 2).min(len + 1));
                dir.0 *= scale * sign;
                dir.1 *= scale * sign;

                let dp = IVec3::new(dir.0, dir.1, len);
                branches.push((p + dp, len));
            }

            let colors = if false {
                vec!["c1", "c2", "c3", "c4", "c5", "c6"]
            } else {
                vec!["brown1", "brown2", "brown2", "brown3"]
            };

            for (q, len) in branches {
                let line = bresenham3d(p, q);
                for r in line {
                    let block = *rng.select(&colors);
                    trunk.set_voxel((r.x, r.y, r.z), block);
                }
                base_points.push((q, len));
            }
        }
        segment += 1;
    }
    trunk
}
