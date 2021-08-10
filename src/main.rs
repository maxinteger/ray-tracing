mod hit_record;
mod objects;
mod ray;
mod vector;

use hit_record::*;
use objects::hittable::*;
use objects::list::*;
use objects::sphere::*;
use ray::*;
use vector::*;

fn write_color(pixel_color: Color) {
    let r = (255.999 * pixel_color.x()) as i64;
    let g = (255.999 * pixel_color.y()) as i64;
    let b = (255.999 * pixel_color.z()) as i64;

    println!("{} {} {}", r, g, b);
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

fn hit_sphere(center: Point3, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin - center;
    let a = ray.direction.length_squared();
    let half_b = oc.dot(ray.direction);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as usize;

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
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0., 0.);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

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
            let u = (i as f64) / ((IMAGE_WIDTH - 1) as f64);
            let v = (j as f64) / ((IMAGE_HEIGHT - 1) as f64);
            let ray = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color = ray_color(ray, &world);

            write_color(pixel_color);
        }
    }
    eprintln!("\nDone");
}
