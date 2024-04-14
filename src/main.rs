use chrono::Utc;

use std::rc::Rc;

use rand::{thread_rng, Rng};

use crate::camera::Camera;
use crate::color::Color;
use crate::hittable::{BvhNode, HittableList, Quad, Sphere};
use crate::material::{Dielectric, DiffuseLight, Lambertian, Metal};
use crate::point3::Point3;
use crate::ray::Ray;
use crate::texture::{
    CheckerTexture, ImageTexture, MarbleTexture, NoiseTexture, SolidColor, Texture,
};
use crate::vec3::Vec3;

mod aabb;
mod camera;
mod color;
mod hit_record;
mod hittable;
mod interval;
mod material;
mod noise;
mod point3;
mod ray;
mod texture;
mod vec3;

fn random_spheres() -> std::io::Result<()> {
    let mut rng = thread_rng();

    let mut world = HittableList::default();

    //let material_ground = Rc::new(Lambertian::new_with_color(Color::new(0.5, 0.5, 0.5)));
    let checker = Rc::new(Lambertian::new(Rc::new(CheckerTexture::new_with_colours(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ))));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -1000.0, -1.0),
        1000.0,
        checker,
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
                    let material = Rc::new(Lambertian::new_with_color(albedo));
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

    let material2 = Rc::new(Lambertian::new_with_color(Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3.clone(),
    )));
    /*
        world.add(Rc::new(Sphere::new(
            Point3::new(-100.0, 1.0, 0.0),
            20.0,
            material3,
        )));
    */
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

    let world = BvhNode::from_hittable_list(world);

    Camera::default()
        .aspect_ratio(16.0 / 9.0)
        .image_width(1200)
        .samples_per_pixel(5000)
        .max_depth(10000)
        .background(Color::new(0.70, 0.80, 1.00))
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

fn two_spheres() -> std::io::Result<()> {
    let mut world = HittableList::default();

    let checker = Rc::new(CheckerTexture::new_with_colours(
        0.8,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    let material = Rc::new(Lambertian::new(checker));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        material.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        material.clone(),
    )));

    Camera::default()
        .aspect_ratio(16.0 / 9.0)
        .image_width(400)
        .samples_per_pixel(100)
        .max_depth(50)
        .background(Color::new(0.70, 0.80, 1.00))
        .vfov(20.0)
        .lookfrom(Point3::new(13.0, 2.0, 3.0))
        .lookat(Point3::new(0.0, 0.0, 0.0))
        .vup(Vec3::new(0.0, 1.0, 0.0))
        .defocus_angle(0.0)
        .render(
            format!("renders/{}.ppm", Utc::now().to_rfc2822()).as_str(),
            &world,
        )?;

    Ok(())
}

fn earth() {
    let earth_texture = Rc::new(ImageTexture::new("/home/Strawby/Downloads/earthmap.jpg"));
    let earth_surface = Rc::new(Lambertian::new(earth_texture));
    let globe = Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0, earth_surface);

    Camera::default()
        .aspect_ratio(16.0 / 9.0)
        .image_width(400)
        .samples_per_pixel(100)
        .max_depth(50)
        .background(Color::new(0.70, 0.80, 1.00))
        .vfov(20.0)
        .lookfrom(Point3::new(0.0, 0.0, 12.0))
        .lookat(Point3::new(0.0, 0.0, 0.0))
        .vup(Vec3::new(0.0, 1.0, 0.0))
        .defocus_angle(0.0)
        .render(
            format!("renders/{}.ppm", Utc::now().to_rfc2822()).as_str(),
            &globe,
        )
        .unwrap();
}

fn two_perlin_spheres() {
    let mut world = HittableList::default();

    let perlin_texture = Rc::new(MarbleTexture::new(4.0));
    let perlin_material = Rc::new(Lambertian::new(perlin_texture));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        perlin_material.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        perlin_material,
    )));

    Camera::default()
        .aspect_ratio(16.0 / 9.0)
        .image_width(400)
        .samples_per_pixel(100)
        .max_depth(50)
        .background(Color::new(0.70, 0.80, 1.00))
        .vfov(20.0)
        .lookfrom(Point3::new(12.0, 2.0, 3.0))
        .lookat(Point3::new(0.0, 0.0, 0.0))
        .vup(Vec3::new(0.0, 1.0, 0.0))
        .defocus_angle(0.0)
        .render(
            format!("renders/{}.ppm", Utc::now().to_rfc2822()).as_str(),
            &world,
        )
        .unwrap();
}

fn quads() {
    let mut world = HittableList::default();

    let red = Rc::new(Lambertian::new_with_color(Color::new(1.0, 0.2, 0.2)));
    let green = Rc::new(Lambertian::new_with_color(Color::new(0.2, 1.0, 0.2)));
    let blue = Rc::new(Lambertian::new_with_color(Color::new(0.2, 0.2, 1.0)));
    let orange = Rc::new(Lambertian::new_with_color(Color::new(1.0, 0.5, 0.0)));
    let teal = Rc::new(Lambertian::new_with_color(Color::new(0.2, 0.8, 0.8)));

    world.add(Rc::new(Quad::new(
        Point3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        red,
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        green,
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        blue,
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        orange,
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        teal,
    )));

    Camera::default()
        .aspect_ratio(1.0)
        .image_width(400)
        .samples_per_pixel(100)
        .max_depth(50)
        .background(Color::new(0.70, 0.80, 1.00))
        .vfov(80.0)
        .lookfrom(Point3::new(0.0, 0.0, 9.0))
        .lookat(Point3::new(0.0, 0.0, 0.0))
        .vup(Vec3::new(0.0, 1.0, 0.0))
        .defocus_angle(0.0)
        .render(
            format!("renders/{}.ppm", Utc::now().to_rfc2822()).as_str(),
            &world,
        )
        .unwrap();
}

fn simple_light() {
    let mut world = HittableList::default();

    let perlin_texture = Rc::new(NoiseTexture::new(4.0));
    let noise_material = Rc::new(Lambertian::new(perlin_texture));

    let mirror = Rc::new(Dielectric::new(1.0));

    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        noise_material.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        noise_material,
    )));

    let difflight = Rc::new(DiffuseLight::new(Rc::<SolidColor>::new(
        Color::new(4.0, 4.0, 4.0).into(),
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(3.0, 1.0, -2.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        difflight,
    )));

    // let world = BvhNode::from_hittable_list(world);

    Camera::default()
        .aspect_ratio(16.0 / 9.0)
        .image_width(900)
        .samples_per_pixel(400)
        .max_depth(50)
        .background(Color::new(0.0, 0.0, 0.0))
        .vfov(20.0)
        .lookfrom(Point3::new(26.0, 3.0, 6.0))
        .lookat(Point3::new(0.0, 2.0, 0.0))
        .vup(Vec3::new(0.0, 1.0, 0.0))
        .defocus_angle(0.0)
        .render(
            format!("renders/{}.ppm", Utc::now().to_rfc2822()).as_str(),
            &world,
        )
        .unwrap();
}
fn main() -> std::io::Result<()> {
    simple_light();

    Ok(())
}
