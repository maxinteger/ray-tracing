mod camera;
mod hit_record;
mod objects;
mod ray;
mod utils;
mod vector;

use camera::*;
use hit_record::*;
use objects::hittable::*;
use objects::list::*;
use objects::sphere::*;
use rand::prelude::ThreadRng;
use rand::Rng;
use ray::*;
use rayon::prelude::*;
use utils::*;
use vector::*;

fn write_color(pixel_color: &Color, sample_per_pixel: usize) {
    let scale = 1.0 / (sample_per_pixel as f64);

    let r = clamp((pixel_color.x() * scale).sqrt(), 0.0, 0.999) * 256.0;
    let g = clamp((pixel_color.y() * scale).sqrt(), 0.0, 0.999) * 256.0;
    let b = clamp((pixel_color.z() * scale).sqrt(), 0.0, 0.999) * 256.0;

    println!("{} {} {}", r as i64, g as i64, b as i64);
}

fn ray_color(rng: &mut ThreadRng, ray: Ray, world: &HittableList, depth: usize) -> Color {
    let mut hit_record = HitRecord::new();

    if depth <= 0 {
        return Color::default();
    }

    if world.hit(&ray, 0.001, f64::INFINITY, &mut hit_record) {
        let target = hit_record.point + hit_record.normal + random_unit_vector(rng);
        0.5 * ray_color(
            rng,
            Ray::new(hit_record.point, target - hit_record.point),
            world,
            depth - 1,
        )
    } else {
        let unit_direction = ray.direction.unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as usize;
    const SAMPLE_PER_PIXEL: usize = 10;
    const MAX_DEPTH: usize = 50;

    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere {
        center: Point3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    }));
    world.add(Box::new(Sphere {
        center: Point3::new(0.0, -100.5, -1.0),
        radius: 100.0,
    }));

    // Camera
    let camera = Camera::new();

    let number_of_pixels = IMAGE_WIDTH * IMAGE_HEIGHT;

    let pixels = (0..number_of_pixels)
        .into_par_iter()
        .map(|index| {
            let index = number_of_pixels - index;
            let i = (index % IMAGE_WIDTH) as f64;
            let j = (index / IMAGE_WIDTH) as f64;
            let mut rng = rand::thread_rng();
            let mut pixel_color = Color::default();
            for _ in 0..SAMPLE_PER_PIXEL {
                let u = (i + rng.gen::<f64>()) / ((IMAGE_WIDTH - 1) as f64);
                let v = (j + rng.gen::<f64>()) / ((IMAGE_HEIGHT - 1) as f64);
                let ray = camera.get_ray(u, v);
                pixel_color = pixel_color + ray_color(&mut rng, ray, &world, MAX_DEPTH);
            }
            pixel_color
        })
        .collect::<Vec<Color>>();

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("256");

    pixels.iter().for_each(|&pixel| {
        write_color(&pixel, SAMPLE_PER_PIXEL);
    });

    eprintln!("\nDone");
}
