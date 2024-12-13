use crate::internal::*;

pub struct Model {
    pub model: ModelType,
    pub position: IVec3,
}

pub struct GenContext<'a> {
    pub center: IVec3,
    pub ground_objects: Vec<&'a Model>,
}

impl<'a> GenContext<'a> {
    pub fn new(center: IVec3) -> Self {
        Self {
            center,
            ground_objects: Vec::new(),
        }
    }

    pub fn ground_height_at(&self, x: i32, y: i32) -> Option<i32> {
        self.ground_objects
            .iter()
            .map(|m| {
                let mx = x - m.position.x;
                let my = y - m.position.y;
                match &m.model {
                    ModelType::VoxelSet(m) => m.height_at(mx, my),
                    ModelType::VoxelScene(m) => None,
                    _ => None,
                }
            })
            .max()
            .unwrap_or(None)
    }
}

pub enum ModelType {
    Empty,
    VoxelSet(Box<VoxelSet>),
    VoxelScene(Box<VoxelScene>),
}

impl Into<ModelType> for VoxelSet {
    fn into(self) -> ModelType {
        ModelType::VoxelSet(Box::new(self))
    }
}

impl Into<ModelType> for VoxelScene {
    fn into(self) -> ModelType {
        ModelType::VoxelScene(Box::new(self))
    }
}

fn bresenham3d(p: IVec3, q: IVec3) -> Vec<IVec3> {
    let mut v = Vec::new();
    for (x, y, z) in line_drawing::Bresenham3d::new((p.x, p.y, p.z), (q.x, q.y, q.z)) {
        v.push(IVec3::new(x, y, z));
    }
    v
}

pub fn generate_model(model_id: &str, seed: u64, ctx: &GenContext) -> ModelType {
    match model_id {
        "tree1" => generate_tree1(seed).into(),
        "tree2" => generate_tree2(seed).into(),
        "pine_tree" => generate_pine_tree(seed, ctx).into(),
        "small_hill" => generate_small_hill(seed, ctx).into(),
        "fence" => generate_fence(seed, ctx).into(),

        "tree_cluster" => generate_tree_cluster(seed).into(),
        "tree_hill" => generate_tree_hill(seed).into(),

        _ => ModelType::Empty,
    }
}

pub fn generate_fence(seed: u64, ctx: &GenContext) -> VoxelSet {
    let mut model = VoxelSet::new();
    model.register_block(Block::color("wood", 30, 12, 5));
    model.register_block(Block::color("wood2", 22, 11, 8));
    model.register_block(Block::color("wood3", 31, 8, 3));
    model.register_block(Block::color("red", 255, 8, 3));
    model.register_block(Block::color("blue", 31, 8, 255));

    use std::f32::consts::PI;
    let mut rng = RNG::new(seed);
    let offset = rng.range(0.0..PI);
    let mut wood_select = rng.select_fn(vec!["wood", "wood2", "wood3"]);

    // Compute segment points
    let mut base_pts = Vec::new();
    for i in (0..360).step_by(30) {
        let angle = offset + (i as f32 * std::f32::consts::PI / 180.0);
        let r = rng.range(40.0..70.0);
        let x = r * angle.cos();
        let y = r * angle.sin();
        let x = x.floor() as i32;
        let y = y.floor() as i32;

        let z = ctx.ground_height_at(x, y).unwrap_or(0);
        base_pts.push(IVec3::new(x, y, z));
    }
    base_pts.push(base_pts[0]); // close the loop

    // Draw posts
    let mut posts = Vec::new();
    for pairs in base_pts.windows(2) {
        let p = pairs[0];
        let q = pairs[1];

        let line = bresenham3d(p, q);
        for i in 0..line.len() {
            let v = &line[i];
            if (i % 5 != 0) || i + 3 >= line.len() {
                continue;
            }

            let gz = ctx.ground_height_at(v.x, v.y).unwrap_or(0);

            let block = wood_select();
            for dz in 0..6 {
                model.set_voxel((v.x, v.y, gz + dz), block);
            }
            posts.push(IVec3::new(v.x, v.y, gz));
        }
    }
    posts.push(posts[0]); // close the loop

    // Draw beams
    for pairs in posts.windows(2) {
        let p = pairs[0];
        let q = pairs[1];
        let line = bresenham3d(IVec3::new(p.x, p.y, p.z), IVec3::new(q.x, q.y, q.z));

        let block_top = wood_select();
        let block_bottom = wood_select();
        for i in 1..line.len() - 1 {
            let v = &line[i];
            model.set_voxel((v.x, v.y, v.z + 4), block_top);
            model.set_voxel((v.x, v.y, v.z + 2), block_bottom);
        }
    }

    model
}

pub fn generate_small_hill(seed: u64, ctx: &GenContext) -> VoxelSet {
    let mut rng = RNG::new(seed);

    let mut model = VoxelSet::new();
    model.register_block(Block::color("dirt", 25, 20, 10));
    model.register_block(Block::color("dirt2", 20, 15, 10));
    model.register_block(Block::color("grass1", 5, 60, 10));
    model.register_block(Block::color("grass2", 3, 45, 2));
    model.register_block(Block::color("red", 255, 0, 0));
    model.register_block(Block::color("blue", 0, 0, 255));

    let cos01 = |x: f32| (x.cos() + 1.0) / 2.0;

    const SIZE: f32 = 64.0;

    let size_x = (SIZE * rng.range(0.75..=1.25)).ceil() as i32;
    let size_y = (SIZE * rng.range(0.75..=1.25)).ceil() as i32;
    let power = rng.range(0.45..=0.55);
    let angle = rng.range(0.0..std::f32::consts::PI * 2.0);
    let mut dirt_block = rng.select_fn(vec!["dirt", "dirt2"]);
    let mut grass_block = rng.select_fn(vec!["grass1", "grass2"]);

    let noise = rng.open_simplex().scale(12.0).build();

    for y in -size_y..=size_y {
        for x in -size_x..=size_x {
            let u = (x as f32) / (size_x as f32);
            let v = (y as f32) / (size_y as f32);
            let u2 = u * angle.cos() - v * angle.sin();
            let v2 = u * angle.sin() + v * angle.cos();
            let u = u2 * 4.0;
            let v = v2 * 4.0;

            let h = 16.0 * cos01(u * 0.5).powf(power) * cos01(v * 0.5).powf(power) - 8.25;
            let h = 3.0 * h;
            let zh = h.floor() as i32;

            let base_z = ctx.ground_height_at(x, y).unwrap_or(0);
            for z in 0..=zh {
                let block = if z < zh {
                    "dirt"
                } else {
                    let v = noise.gen_3d(x as f32, y as f32, z as f32);
                    if v < 0.41 {
                        dirt_block()
                    } else {
                        grass_block()
                    }
                };
                model.set_voxel((x, y, base_z + z), block);
            }
        }
    }
    model
}

pub fn generate_pine_tree(seed: u64, ctx: &GenContext) -> VoxelSet {
    let mut rng = RNG::new(seed);

    let mut model = VoxelSet::new();
    model.register_block(Block::color("leaves", 10, 140, 30));
    model.register_block(Block::color("leaves2", 30, 90, 30));
    model.register_block(Block::color("leaves3", 30, 70, 30));
    model.register_block(Block::color("wood", 46, 38, 38));
    model.register_block(Block::color("wood2", 26, 28, 28));
    model.register_block(Block::color("wood3", 36, 38, 31));

    const R: i32 = 8;
    let base_height: i32 = rng.range(8..=12);
    let cone_height: i32 = base_height + rng.range(4..=16);
    let girth: f32 = rng.range(0.5..=0.75);

    let mut leaf_select = rng.select_fn(vec!["leaves", "leaves2", "leaves3"]);
    let mut wood_select = rng.select_fn(vec!["wood", "wood2", "wood3"]);

    let base_z = ctx
        .ground_height_at(ctx.center.x, ctx.center.y)
        .unwrap_or(0);

    for z in base_z..=base_z + base_height {
        model.set_voxel((0, 0, z), wood_select());
    }

    for z in 0..=cone_height {
        let r = (cone_height - z) + 1;
        let r = (r as f32).powf(girth).ceil() as i32;
        for y in -r..=r {
            for x in -r..=r {
                let d = (x.pow(2) + y.pow(2)) as f32;
                if d > r as f32 * r as f32 {
                    continue;
                }
                let p = ((x.abs() % 2) + (y.abs() % 2)) == 1;
                if p == ((z.abs() % 2) == 1) {
                    continue;
                }

                let block = leaf_select();
                model.set_voxel((x, y, z + base_height + base_z), block);
            }
        }
    }
    model
}

pub fn generate_tree2(seed: u64) -> VoxelSet {
    let mut rng = RNG::new(seed);

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

    for z in 0..=height {
        let block_name = wood_select();
        model.set_voxel((0, 0, z), block_name);
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
                model.set_voxel((x, y, z + R / 2 + height), name);
            }
        }
    }
    model
}

pub fn generate_tree1(_seed: u64) -> VoxelSet {
    let mut model = VoxelSet::new();
    model.register_block(Block::color("grass", 50, 200, 50));
    model.register_block(Block::color("sand", 180, 200, 20));
    model.register_block(Block::color("wood", 46, 38, 38));

    const R: i32 = 8;
    const H: i32 = 20;

    for z in 0..=H {
        model.set_voxel((0, 0, z), "wood");
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
                let name = if (y.abs() % 2) == 0 { "grass" } else { "sand" };
                model.set_voxel((x, y, z + R / 2 + H), name);
            }
        }
    }
    model
}

pub fn generate_tree_hill(seed: u64) -> VoxelScene {
    let mut rng = RNG::new(seed);

    let mut scene = VoxelScene::new();

    let hill_seed = rng.range(1..8192);
    let tree_cluster_seed = rng.range(1..8192);

    let ctx = GenContext {
        center: IVec3::new(0, 0, 0),
        ground_objects: vec![],
    };
    let hill = generate_small_hill(hill_seed, &ctx);
    scene.add_object(
        0,
        Object {
            model_id: "small_hill".to_string(),
            seed: hill_seed,
            position: IVec3::new(0, 0, 0),
        },
    );

    let tree_cluster = generate_tree_cluster(tree_cluster_seed);
    for object in tree_cluster.layers[0].objects.iter() {
        let mut p = object.position.clone();
        let z = hill.height_at(p.x, p.y).unwrap_or(0);
        p.z = z + 1;

        scene.add_object(
            1,
            Object {
                model_id: object.model_id.clone(),
                seed: object.seed,
                position: p,
            },
        );
    }
    scene
}

pub fn generate_tree_cluster(seed: u64) -> VoxelScene {
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
