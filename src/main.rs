use chrono::Utc;

use std::rc::Rc;

use rand::{thread_rng, Rng};

use crate::camera::Camera;
use crate::color::Color;
use crate::hittable::{HittableList, Sphere};
use crate::material::{Dielectric, Lambertian, Metal};
use crate::point3::Point3;
use crate::ray::Ray;
use crate::vec3::Vec3;

mod camera;
mod color;
mod hit_record;
mod hittable;
mod interval;
mod material;
mod point3;
mod ray;
mod vec3;

fn main() -> std::io::Result<()> {
    let mut rng = thread_rng();

    let mut world = HittableList::default();

    let material_ground = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));

    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -1000.0, -1.0),
        1000.0,
        material_ground,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen_range(0.0..1.0);
            let center = Point3::new(
                a as f64 + 0.9 * rng.gen_range(0.0..1.0),
                0.2,
                b as f64 + 0.9 * rng.gen_range(0.0..1.0),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    let material = Rc::new(Lambertian::new(albedo));
                    let center2 = center + &Vec3::new(0.0, rng.gen_range(0.0..0.2), 0.0);
                    world.add(Rc::new(Sphere::new_moving(center, center2, 0.2, material)))
                } else if choose_mat < 0.95 {
                    let albedo = Color::random() * Color::random_range(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    let material = Rc::new(Metal::new(albedo, fuzz));
                    world.add(Rc::new(Sphere::new(center, 0.2, material)))
                } else {
                    let material = Rc::new(Dielectric::new(1.5));
                    world.add(Rc::new(Sphere::new(center, 0.2, material.clone())));
                    if rng.gen_bool(0.5) {
                        world.add(Rc::new(Sphere::new(center, -0.15, material)));
                    }
                }
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1.clone(),
    )));

    let material2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    let material4 = Rc::new(Dielectric::new(1.9));
    world.add(Rc::new(Sphere::new(
        Point3::new(8.0, 1.0, 0.0),
        1.0,
        material4.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(8.0, 1.0, 0.0),
        -0.9,
        material4,
    )));

    Camera::default()
        .aspect_ratio(16.0 / 9.0)
        .image_width(1200)
        .samples_per_pixel(500)
        .max_depth(100)
        .vfov(20.0)
        .lookfrom(Point3::new(13.0, 2.0, 3.0))
        .lookat(Point3::new(0.0, 0.0, 0.0))
        .vup(Vec3::new(0.0, 1.0, 0.0))
        .defocus_angle(0.6)
        .focus_dist(10.0)
        .render(
            format!("renders/{}.ppm", Utc::now().to_rfc2822()).as_str(),
            &world,
        )?;

    Ok(())
}
