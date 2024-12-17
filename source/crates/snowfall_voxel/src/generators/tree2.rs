use crate::internal::*;

pub fn tree2(ctx: &GenContext, scene: &Scene2) -> VoxelSet {
    let mut rng = ctx.make_rng();

    let mut model = VoxelSet::new();
    model.register_block(Block::color("leaves", 40, 180, 30));
    model.register_block(Block::color("leaves2", 30, 150, 30));
    model.register_block(Block::color("leaves3", 230, 150, 30));
    model.register_block(Block::color("sand", 180, 200, 20));
    model.register_block(Block::color("wood", 46, 38, 38));
    model.register_block(Block::color("birch_wood0", 200, 205, 203));
    model.register_block(Block::color("birch_wood1", 180, 185, 173));
    model.register_block(Block::color("birch_wood2", 30, 35, 33));

    let tree_type = *rng.select(&vec!["standard", "birch"]);

    let mut wood_select = {
        let mut rng = rng.fork();
        move || match tree_type {
            "birch" => *rng.select_weighted(&vec![
                (40, "birch_wood0"),
                (40, "birch_wood1"),
                (20, "birch_wood2"),
            ]),
            _ => "wood",
        }
    };
    let mut leaf_select = {
        let mut rng = rng.fork();
        move || match tree_type {
            "birch" => *rng.select(&vec!["leaves", "leaves2"]),
            _ => *rng.select(&vec!["leaves", "leaves2", "leaves3"]),
        }
    };

    const R: i32 = 8;
    let height: i32 = rng.range(12..=20);
    let noise = rng.open_simplex().scale(3.0).build();
    let base_z = scene.terrain.height_at(0, 0).unwrap_or(0);

    for z in 0..=height {
        let block_name = wood_select();
        model.set_voxel((0, 0, base_z + z), block_name);
    }

    for z in -R..=R {
        for y in -R..=R {
            for x in -R..=R {
                let d = (x.pow(2) + y.pow(2) + z.pow(2)) as f32;
                let r = R as f32;
                if d > r * r {
                    continue;
                }

                let p = ((x.abs() % 2) + (y.abs() % 2)) % 2;
                if p == (z.abs() % 2) {
                    continue;
                }

                let v = noise.gen_3d(x as f32, y as f32, z as f32);
                if v < 0.40 {
                    continue;
                }

                let name = leaf_select();
                model.set_voxel((x, y, base_z + z + R / 2 + height), name);
            }
        }
    }
    model
}
