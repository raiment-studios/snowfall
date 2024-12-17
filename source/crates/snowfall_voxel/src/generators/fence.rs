use crate::internal::*;

pub fn fence(ctx: &GenContext, scene: &Scene2) -> VoxelSet {
    let mut model = VoxelSet::new();
    model.register_block(Block::color("wood", 30, 12, 5));
    model.register_block(Block::color("wood2", 22, 11, 8));
    model.register_block(Block::color("wood3", 31, 8, 3));
    model.register_block(Block::color("red", 255, 8, 3));
    model.register_block(Block::color("blue", 31, 8, 255));

    use std::f32::consts::PI;
    let mut rng = ctx.make_rng();
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

        let z = scene.terrain.height_at(x, y).unwrap_or(0);
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

            let gz = scene.terrain.height_at(v.x, v.y).unwrap_or(0);

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
