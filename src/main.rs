mod vec3;
use std::rc::Rc;

use vec3::Vec3;
use vec3::Color;
use vec3::Point3;

mod ray;

mod camera;
use camera::Camera;

mod hittable;
use hittable::{Hittable};

mod hittable_list;
use hittable_list::HittableList;

mod sphere;
use sphere::Sphere;

mod material;
use material::{Lambertian, Metal};

mod rtweekend;
use rtweekend::{INFINITY, random};


fn ray_color(r : &ray::Ray, world: &HittableList, depth: i32) -> Color {

    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(&r, 0.001, INFINITY) {
        if let Some((scattered, attenuation)) = rec.mat.scatter(r, &rec) {
            return attenuation * ray_color(&scattered, world, depth-1);
        }
        return Color::new(0.0, 0.0, 0.0);
    }

    let unit_direction = Vec3::unit_vector(&r.direction());
    let t = 0.5 * ( unit_direction.y() + 1.0 );
    (1.0-t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World 
    let mut world = HittableList {
        objects: Vec::new()
    }; 

    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let material_left = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground.clone()
    )));

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_center
    )));

    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left
    )));

    world.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    // Camera
    let cam = Camera::new();

    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in  (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..image_width {
            let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + random()) / (image_width-1) as f64;
                let v = (j as f64 + random()) / (image_height-1) as f64;

                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, max_depth);
            }
            Color::write_color(pixel_color, samples_per_pixel);
        }
    }
    eprintln!("\nDone.");
}
