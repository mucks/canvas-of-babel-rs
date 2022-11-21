use std::{collections::HashMap, path::Path};

use image::{ImageBuffer, Rgb, RgbImage};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use rand_pcg::Pcg64;
use rand_seeder::Seeder;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

fn main() {
    let time = std::time::Instant::now();

    gen_many_images();

    println!("{}", time.elapsed().as_millis());
}

fn gen_many_images() {
    for i in 0..1000 {
        println!("generating image number: {}", i);
        gen_one_image_random_seed();
    }
}

fn gen_one_image_random_seed() {
    // could use this to generate images too since it can create strings of arbitary length
    let seed: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(20)
        .map(char::from)
        .collect();
    gen_one_image(&seed);
}

fn gen_one_image(seed: &str) {
    let pixels = gen_pixels_4bit_fast(seed);
    gen_image(&pixels, seed);
}

#[derive(Debug)]
struct Pixel4bit {
    pub x: u32,
    pub y: u32,
    pub rgb: Rgb4bit,
}

#[derive(Debug)]
struct Rgb4bit {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

fn gen_pixels_4bit_fast(seed: &str) -> Vec<Pixel4bit> {
    let rng: Pcg64 = Seeder::from(seed).make_rng();

    let len: usize = (WIDTH * HEIGHT * 3) as usize;

    println!("Seed: {}", seed);

    let hash: String = rng
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect();

    // println!("Picture Hash: {}", hash);

    let mut pixels = vec![];

    let mut x = 0;
    let mut y = 0;

    for i in (0..hash.len() - 3).step_by(3) {
        let val: &str = &hash[i..i + 3];
        let mut chars = val.chars();

        let r: u8 = chars.next().unwrap() as u8 / 8;
        let g: u8 = chars.next().unwrap() as u8 / 8;
        let b: u8 = chars.next().unwrap() as u8 / 8;

        pixels.push(Pixel4bit {
            x,
            y,
            rgb: Rgb4bit { r, g, b },
        });

        x += 1;
        if x >= WIDTH {
            y += 1;
            x = 0;
        }
    }

    pixels
}

fn gen_pixels_4bit(seed: &str) -> Vec<Pixel4bit> {
    let mut rng: Pcg64 = Seeder::from(seed).make_rng();
    println!("{}", rng.gen::<u64>());

    let color_count = 4096_usize;

    let mut color_map: HashMap<usize, (u8, u8, u8)> = HashMap::new();

    let mut i = 0;

    for r in 0..16 {
        for g in 0..16 {
            for b in 0..16 {
                color_map.insert(i, (r, g, b));
                i += 1;
            }
        }
    }

    let mut pixels = vec![];

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let mut rng: Pcg64 = Seeder::from(format!("{}{}{}", x, y, seed)).make_rng();
            let random_color = rng.gen_range(0..color_count);
            let (r, g, b) = color_map.get(&random_color).unwrap();
            pixels.push(Pixel4bit {
                x,
                y,
                rgb: Rgb4bit {
                    r: *r,
                    g: *g,
                    b: *b,
                },
            });
        }
    }
    pixels
}

fn gen_image(pixels: &Vec<Pixel4bit>, seed: &str) {
    let mut img: RgbImage = ImageBuffer::new(WIDTH, HEIGHT);
    for p in pixels {
        let pixel = Rgb([p.rgb.r * 16, p.rgb.g * 16, p.rgb.b * 16]);
        img.put_pixel(p.x, p.y, pixel);
    }
    let out_path = Path::new("./out");
    let out_img_path = out_path.join(format!("{}.png", seed));
    img.save(out_img_path.to_str().unwrap()).unwrap();
}
