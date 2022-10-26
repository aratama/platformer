use image::{GenericImage, GenericImageView, ImageBuffer, Rgb, RgbImage};
use std::fs;

fn main() {
    let img = image::open("world.png").unwrap();

    println!("dimensions {:?}", img.dimensions());

    println!("{:?}", img.color());

    let rgb: RgbImage = img.to_rgb8();

    let mut cell: u32 = 0;
    let mut count: u32 = 0;
    let mut vec: Vec<u32> = vec![];
    for (x, y, pixel) in rgb.enumerate_pixels() {
        let v = match (pixel[0], pixel[1], pixel[2]) {
            (0, 0, 0) => 0,    // 空間
            (34, 32, 52) => 1, // 永久壁
            _ => 0,            // 無視
        };
        if x == 0 && y == 0 {
            cell = v;
            count = 1;
        } else if (cell == v) {
            count += 1;
        } else {
            vec.push(cell);
            vec.push(count);
            cell = v;
            count = 1;
        }
    }

    let mut source: String = format!("const WORLD_RLE: [u8; {}] = [", vec.len());
    for (i, v) in vec.iter().enumerate() {
        if i != 0 {
            source = source + ", ";
        }
        source = source + &format!("{}", v);
    }
    source.push_str("];");

    fs::write("world.rs", source);
}
