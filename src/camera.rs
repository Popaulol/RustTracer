use std::cmp::max;
use std::fs::File;
use std::io::{stdout, Write};
use std::time::Instant;

use rand::Rng;

use crate::color::Color;
use crate::hit_record::HitRecord;
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::point3::Point3;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
    aspect_ratio: f64,
    image_width: i32,
    samples_per_pixel: i32,
    max_depth: i32,

    vfov: f64,
    lookfrom: Point3,
    lookat: Point3,
    vup: Vec3,

    defocus_angle: f64,
    focus_dist: f64,

    image_height: i32,
    center: Point3,
    pixel_00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    fn ray_color(r: &Ray, depth: i32, world: &impl Hittable) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let mut rec: HitRecord = Default::default();
        if world.hit(r, Interval::new(0.001, f64::INFINITY), &mut rec) {
            let mut scattered = Ray::default();
            let mut attenuation = Color::default();
            if rec
                .material
                .as_ref()
                .unwrap()
                .scatter(r, &rec, &mut attenuation, &mut scattered)
            {
                return attenuation * Self::ray_color(&scattered, depth - 1, world);
            }
            return Color::new(0.0, 0.0, 0.0);
        }

        let unit_direction = r.direction().unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            vfov: 90.0,
            lookfrom: Point3::new(0.0, 0.0, -1.0),
            lookat: Point3::new(0.0, 0.0, 0.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,
            image_height: 0,
            center: Default::default(),
            pixel_00_loc: Default::default(),
            pixel_delta_u: Default::default(),
            pixel_delta_v: Default::default(),
            u: Default::default(),
            v: Default::default(),
            w: Default::default(),
            defocus_disk_u: Default::default(),
            defocus_disk_v: Default::default(),
        }
    }
}

impl Camera {
    pub(crate) fn aspect_ratio(mut self, aspect_ratio: f64) -> Self {
        self.aspect_ratio = aspect_ratio;
        self
    }

    pub(crate) fn image_width(mut self, width: i32) -> Self {
        self.image_width = width;
        self
    }
    pub(crate) fn samples_per_pixel(mut self, samples_per_pixel: i32) -> Self {
        self.samples_per_pixel = samples_per_pixel;
        self
    }
    pub(crate) fn max_depth(mut self, depth: i32) -> Self {
        self.max_depth = depth;
        self
    }

    pub(crate) fn vfov(mut self, vfov: f64) -> Self {
        self.vfov = vfov;
        self
    }

    pub(crate) fn lookfrom(mut self, lookfrom: Point3) -> Self {
        self.lookfrom = lookfrom;
        self
    }

    pub(crate) fn lookat(mut self, lookat: Point3) -> Self {
        self.lookat = lookat;
        self
    }
    pub(crate) fn vup(mut self, vup: Vec3) -> Self {
        self.vup = vup;
        self
    }

    pub fn defocus_angle(mut self, defocus_angle: f64) -> Self {
        self.defocus_angle = defocus_angle;
        self
    }

    pub fn focus_dist(mut self, focus_dist: f64) -> Self {
        self.focus_dist = focus_dist;
        self
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let mut rng = rand::thread_rng();
        let px = -0.5 + rng.gen::<f64>();
        let py = -0.5 + rng.gen::<f64>();
        (px * self.pixel_delta_u) + (py * self.pixel_delta_v)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = Vec3::random_in_unit_disk();

        self.center + &(p.x() * self.defocus_disk_u) + &(p.y() * self.defocus_disk_v)
    }

    fn get_ray(&mut self, i: i32, j: i32) -> Ray {
        let pixel_center =
            self.pixel_00_loc + &(i as f64 * self.pixel_delta_u) + &(j as f64 * self.pixel_delta_v);
        let pixel_sample = pixel_center + &self.pixel_sample_square();

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        self.image_height = max(self.image_height, 1);

        self.center = self.lookfrom;

        let theta = self.vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        self.w = (self.lookfrom - self.lookat).unit_vector();
        self.u = self.vup.cross(&self.w).unit_vector();
        self.v = self.w.cross(&self.u);

        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * -self.v;

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left =
            self.center - (self.focus_dist * self.w) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel_00_loc =
            &viewport_upper_left + 0.5 * (&self.pixel_delta_u + &self.pixel_delta_v);

        let defocus_radius = self.focus_dist * (self.defocus_angle / 2.0).to_radians().tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }
    pub fn render(&mut self, file_name: &str, world: &impl Hittable) -> std::io::Result<()> {
        self.initialize();

        let mut file = File::create(file_name)?;

        writeln!(file, "P3")?;
        writeln!(file, "{}", self.image_width)?;
        writeln!(file, "{}", self.image_height)?;
        writeln!(file, "{}", 255)?;

        let start = Instant::now();

        for j in 0..self.image_height {
            print!(
                "\rScanline {}/{} ({:.2}%), {} remaining. Time Spend: {:.2}s. Time Remaining Estimate: {:.2}s     ",
                j + 1,
                self.image_height,
                ((j + 1) as f64 / self.image_height as f64) * 100.0,
                (self.image_height - j),
                start.elapsed().as_secs_f64(),
                (start.elapsed().as_secs_f64() / max(j, 1) as f64) * (self.image_height - j) as f64,
            );
            stdout().flush().unwrap();
            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += Self::ray_color(&r, self.max_depth, world);
                }
                pixel_color.sample_scale(self.samples_per_pixel);
                writeln!(file, "{}", pixel_color.gamma())?;
            }
        }
        println!("\r...done. Total Time: {}s                                                                                                         ", start.elapsed().as_secs_f64());

        Ok(())
    }
}
