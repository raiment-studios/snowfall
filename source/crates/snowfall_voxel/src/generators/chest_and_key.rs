use bevy_color::{Hsla, Srgba};

use crate::internal::*;

pub fn chest_and_key(ctx: &GenContext, scene: &mut Scene2) -> Group {
    let mut rng = ctx.make_rng();

    let mut model = Group::new();
    scene.terrain = generators::flat_ground(
        &ctx.fork("flat_ground", rng.seed8())
            .with_params(serde_json::json!({
                "ground_type": "dirt",
            })),
        scene,
    );

    generators::rocks(&ctx.fork("rocks", rng.seed8()), scene);

    for _ in 0..48 {
        const R: i32 = 72;
        let hue = rng.range(0.0..360.0);

        let (sat, lit) = if rng.bool() {
            (rng.range(0.25..0.8), rng.range(0.5..0.8))
        } else {
            (0.1, rng.range(0.25..0.4))
        };
        let rgb = hsla_to_rgb(&Hsla::new(hue, sat, lit, 1.0));

        let mut p = IVec3::new(rng.range(-R..=R), rng.range(-R..=R), 0);
        let z = scene.terrain.height_at(p.x, p.y).unwrap_or(0) + 1;

        let mut occupied = false;
        for dx in -2..=2 {
            for dy in -2..=2 {
                let z = scene.terrain.height_at(p.x + dx, p.y + dy).unwrap_or(0);
                let block = scene.terrain.get_voxel((p.x + dx, p.y + dy, z));
                if block.occupied {
                    occupied = true;
                }
            }
        }
        if occupied {
            continue;
        }

        p.z = z;
        let ctx = ctx
            .fork("key", rng.seed8())
            .with_center(p)
            .with_params(serde_json::json!({
                "color": rgb,
            }));
        let voxels = key(&ctx, scene);
        let mut obj = ctx.to_object(voxels);
        obj.scale = 0.35;
        model.objects.push(obj);
    }

    model
}

pub fn hsla_to_rgb(c: &Hsla) -> (u8, u8, u8) {
    let c = Srgba::from(*c);
    (
        (c.red * 255.0).round() as u8,
        (c.green * 255.0).round() as u8,
        (c.blue * 255.0).round() as u8,
    )
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
struct Params {
    color: Option<(u8, u8, u8)>,
}

pub fn key(ctx: &GenContext, scene: &mut Scene2) -> VoxelSet {
    let params: Params = ctx.params();
    let base_color = params.color;

    let mut model = VoxelSet::new();
    let mut block_cache: HashMap<(u8, u8, u8), PaletteIndex> = HashMap::new();

    let path = "./assets/key-sprite.png";
    let img = image::open(path)
        .expect("Failed to open image")
        .into_rgba8();

    for y in 0..img.height() {
        for x in 0..img.width() {
            let pixel = img.get_pixel(x, y);
            let data = pixel.0;
            let r = data[0];
            let g = data[1];
            let b = data[2];
            let a = data[3];
            if a == 0 {
                continue;
            }

            let rgb = match base_color {
                Some(rgb) => rgb,
                None => (r, g, b),
            };
            let block = *block_cache.entry(rgb).or_insert_with(|| {
                let (r, g, b) = (rgb.0, rgb.1, rgb.2);
                let name = format!("color_{}_{}_{}", r, g, b);
                let block: PaletteIndex =
                    model.register_block(Block::color(name.as_str(), r, g, b));
                block
            });

            let vx = (x - img.width() / 2 - 1) as i32;
            let vz = (img.height() - y - 1) as i32;
            model.set((vx, 0, vz), block);
        }
    }

    model
}
