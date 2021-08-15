mod camera;
mod hit_record;
mod material;
mod objects;
mod ray;
mod utils;
mod vector;

use crate::material::lambertian::Lambertian;
use crate::material::metal::Metal;
use camera::*;
use hit_record::*;
use objects::base::*;
use objects::list::*;
use objects::sphere::*;
use rand::prelude::ThreadRng;
use rand::Rng;
use ray::*;
use rayon::prelude::*;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::Arc;
use utils::*;
use vector::*;

fn write_color(pixel_color: &Color, sample_per_pixel: usize) -> String {
    let scale = 1.0 / (sample_per_pixel as f64);

    let r = clamp((pixel_color.x() * scale).sqrt(), 0.0, 0.999) * 256.0;
    let g = clamp((pixel_color.y() * scale).sqrt(), 0.0, 0.999) * 256.0;
    let b = clamp((pixel_color.z() * scale).sqrt(), 0.0, 0.999) * 256.0;

    format!("{} {} {}", r as i64, g as i64, b as i64)
}

fn ray_color(rng: &mut ThreadRng, ray: Ray, world: &HittableList, depth: usize) -> Color {
    let mut hit_record = HitRecord::new();

    if depth <= 0 {
        return Color::default();
    }

    return if world.hit(&ray, 0.001, f64::INFINITY, &mut hit_record) {
        if let Some(ref mat) = hit_record.material {
            let (is_scatter, scattered_ray, attenuation) = mat.scatter(rng, &ray, &hit_record);
            if is_scatter {
                return attenuation * ray_color(rng, scattered_ray, world, depth - 1);
            }
        }
        Color::default()
    } else {
        let unit_direction = ray.direction.unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    };
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as usize;
    const SAMPLE_PER_PIXEL: usize = 10;
    const MAX_DEPTH: usize = 50;

    let material_ground = Lambertian {
        albedo: Color::new(0.8, 0.8, 0.0),
    };
    let material_center = Lambertian {
        albedo: Color::new(0.7, 0.3, 0.3),
    };
    let material_left = Metal {
        albedo: Color::new(0.8, 0.8, 0.8),
        fuzz: 0.3,
    };
    let material_right = Metal {
        albedo: Color::new(0.8, 0.6, 0.2),
        fuzz: 1.0,
    };

    // World
    let mut world = HittableList::new();
    // ground
    world.add(Box::new(Sphere {
        center: Point3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        material: Arc::new(material_ground),
    }));
    // center
    world.add(Box::new(Sphere {
        center: Point3::new(0.0, 0.0, -1.0),
        radius: 0.5,
        material: Arc::new(material_center),
    }));
    // left
    world.add(Box::new(Sphere {
        center: Point3::new(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: Arc::new(material_left),
    }));
    // right
    world.add(Box::new(Sphere {
        center: Point3::new(1.0, 0.0, -1.0),
        radius: 0.5,
        material: Arc::new(material_right),
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

    let data = pixels
        .iter()
        .map(|&pixel| write_color(&pixel, SAMPLE_PER_PIXEL))
        .collect::<Vec<String>>()
        .join("\n");

    fs::write(
        "./output.ppm",
        format!("P3\n{} {}\n{}\n{}", IMAGE_WIDTH, IMAGE_HEIGHT, 256, data),
    );

    eprintln!("\nDone");
}
