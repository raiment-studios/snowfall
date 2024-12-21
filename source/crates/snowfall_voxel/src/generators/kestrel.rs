use bevy_color::{Hsla, LinearRgba, Srgba};

use crate::internal::*;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
struct Params {
    color: Option<(u8, u8, u8)>,
}

pub fn kestrel(ctx: &GenContext, scene: &mut Scene2) -> VoxelSet {
    let params: Params = ctx.params();
    let base_color = params.color;

    let mut model = VoxelSet::new();
    let mut block_cache: HashMap<(u8, u8, u8), PaletteIndex> = HashMap::new();

    let path = "./assets/kestrel.png";
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
            if a < 10 {
                continue;
            }

            let rgb = match base_color {
                Some(rgb) => rgb,
                None => (r, g, b),
            };
            let block = *block_cache.entry(rgb).or_insert_with(|| {
                let (r, g, b) = (rgb.0, rgb.1, rgb.2);
                let name = format!("color_{}_{}_{}", r, g, b);

                let srgba = Srgba::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0);
                let lrgba = LinearRgba::from(srgba);
                let (r, g, b) = (
                    (lrgba.red * 255.0).floor() as u8,
                    (lrgba.green * 255.0).floor() as u8,
                    (lrgba.blue * 255.0).floor() as u8,
                );

                let block: PaletteIndex =
                    model.register_block(Block::color(name.as_str(), r, g, b));
                block
            });

            let vx = (x - img.width() / 2 - 1) as i32;
            let vz = (img.height() - y - 1) as i32;
            model.set((vx, 0, vz), block);
        }
    }

    model.attributes.push(VoxelSetAttribute::Scale(0.5));
    model.attributes.push(VoxelSetAttribute::Unlit);
    model.attributes.push(VoxelSetAttribute::BillboardZ);
    model
}
