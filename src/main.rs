mod vec3;
use vec3::Vec3;
use vec3::Color;
use vec3::Point3;

mod ray;
use ray::Ray;

mod camera;
use camera::Camera;

mod hittable;
use hittable::{Hittable};

mod hittable_list;
use hittable_list::HittableList;

mod sphere;
use sphere::Sphere;

mod rtweekend;
use rtweekend::{INFINITY, random};


fn ray_color(r : &ray::Ray, world: &HittableList, depth: i32) -> Color {

    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    if let Some(rec) = world.hit(&r, 0.001, INFINITY) {
        let target = rec.p + rec.normal + Vec3::random_in_hemisphere(&rec.normal);
        let r = Ray{orig: rec.p, dir: target - rec.p};
        return 0.5 * ray_color(&r, world, depth-1);
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
    world.add(Box::new(Sphere {
        center: Point3::new(0.0, 0.0, -1.0),
        radius: 0.5
    }));

    world.add(Box::new(Sphere {
        center: Point3::new(0.0, -100.5, -1.0),
        radius: 100.0
    }));

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
