mod vec3;
use vec3::Vec3;
use vec3::Color;
use vec3::Point3;

mod ray;
use ray::Ray;

mod hittable;
use hittable::{Hittable, HitRecord};

mod hittable_list;
use hittable_list::HittableList;

mod sphere;
use sphere::Sphere;

mod rtweekend;
use rtweekend::{INFINITY, PI};


fn ray_color(r : &ray::Ray, world: &HittableList) -> Color {
    if let Some(rec) = world.hit(&r, 0.0, INFINITY) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
    }
    let unit_direction = Vec3::unit_vector(&r.direction());
    let t = 0.5 * ( unit_direction.y() + 1.0 );
    (1.0-t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin() - *center;
    let a = r.direction().length_squared();
    let half_b = Vec3::dot(&oc, &r.direction());
    let c = oc.length_squared() - radius * radius;

    let discriminant = half_b*half_b - a*c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b-discriminant.sqrt()) / a
    }
}


fn main() {

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

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

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0,0.0, focal_length);

    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in  (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..image_width {
            let u = i as f64 / (image_width-1) as f64;
            let v = j as f64 / (image_height-1) as f64;

            let r = Ray {
                orig: origin, 
                dir: lower_left_corner + u* horizontal + v* vertical - origin ,
            };
            let pixel_color = ray_color(&r, &world);
            Color::write_color(pixel_color);
        }
    }
    eprintln!("\nDone.");
}
