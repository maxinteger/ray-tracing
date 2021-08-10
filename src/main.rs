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
use rand::Rng;
use ray::*;
use utils::*;
use vector::*;

fn write_color(pixel_color: Color, sample_per_pixel: usize) {
    let scale = 1.0 / (sample_per_pixel as f64);

    let r = clamp(pixel_color.x() * scale, 0.0, 0.999) * 256.0;
    let g = clamp(pixel_color.y() * scale, 0.0, 0.999) * 256.0;
    let b = clamp(pixel_color.z() * scale, 0.0, 0.999) * 256.0;

    println!("{} {} {}", r as i64, g as i64, b as i64);
}

fn ray_color(ray: Ray, world: &HittableList) -> Color {
    let mut hit_record = HitRecord {
        point: Point3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 0.0, 0.0),
        t: 0.0,
    };

    if world.hit(&ray, 0.0, f64::INFINITY, &mut hit_record) {
        0.5 * (hit_record.normal + Color::new(1.0, 1.0, 1.0))
    } else {
        let unit_direction = ray.direction.unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as usize;
    const SAMPLE_PER_PIXEL: usize = 100;

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

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("256");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!(
            "\rScanlines remaining: {:0>3} - {:0>3}%",
            j,
            (100.0 / (IMAGE_HEIGHT as f64) * ((IMAGE_HEIGHT - j) as f64)).round()
        );
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLE_PER_PIXEL {
                let u = ((i as f64) + rng.gen::<f64>()) / ((IMAGE_WIDTH - 1) as f64);
                let v = ((j as f64) + rng.gen::<f64>()) / ((IMAGE_HEIGHT - 1) as f64);
                let ray = camera.get_ray(u, v);
                pixel_color = pixel_color + ray_color(ray, &world);
            }

            write_color(pixel_color, SAMPLE_PER_PIXEL);
        }
    }
    eprintln!("\nDone");
}
