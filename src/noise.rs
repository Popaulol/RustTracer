use crate::point3::Point3;
use crate::vec3::Vec3;
use rand::{thread_rng, Rng};
use std::ops::MulAssign;
use std::rc::Rc;

const POINT_COUNT: usize = 256;
pub struct Perlin {
    ranvec: Vec<Vec3>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    pub fn new() -> Self {
        let mut ranvec = Vec::with_capacity(POINT_COUNT);
        for _ in 0..POINT_COUNT {
            ranvec.push(Vec3::random_range(-1.0, 1.0).unit_vector())
        }
        Self {
            ranvec,
            perm_x: perlin_generate_perm(),
            perm_y: perlin_generate_perm(),
            perm_z: perlin_generate_perm(),
        }
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let u = u * u * (3.0 - 2.0 * u);
        let v = v * v * (3.0 - 2.0 * v);
        let w = w * w * (3.0 - 2.0 * w);

        let i = p.x().floor();
        let j = p.y().floor();
        let k = p.z().floor();

        let mut c = [[[Vec3::default(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranvec[self.perm_x
                        [(((i as isize) + (di as isize)) & 255) as usize]
                        ^ self.perm_y[(((j as isize) + (dj as isize)) & 255) as usize]
                        ^ self.perm_z[(((k as isize) + (dk as isize)) & 255) as usize]]
                }
            }
        }

        perlin_interpolation(c, u, v, w)
    }

    pub fn turb(&self, p: &Point3) -> f64 {
        self.turb_depth(p, 7)
    }

    pub fn turb_depth(&self, p: &Point3, depth: i32) -> f64 {
        let mut acc = 0.0;
        let mut weight = 1.0;
        let mut p = p.clone();

        for _ in 0..depth {
            acc += weight * self.noise(&p);
            weight *= 0.5;
            p *= 2.0;
        }

        acc.abs()
    }
}

fn perlin_generate_perm() -> Vec<usize> {
    let mut p = Vec::new();
    for i in 0..POINT_COUNT {
        p.push(i)
    }

    permute(p, POINT_COUNT)
}

fn permute(mut p: Vec<usize>, n: usize) -> Vec<usize> {
    let mut rng = thread_rng();
    for i in (n - 1)..0 {
        let target = rng.gen();
        p.swap(i, target);
    }
    p
}

fn perlin_interpolation(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);

    let mut acc = 0.0;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let fi = i as f64;
                let fj = j as f64;
                let fk = k as f64;

                let weight_v = Vec3::new(u - fi, v - fj, w - fk);

                acc += (fi * uu + (1.0 - fi) * (1.0 - uu))
                    * (fj * vv + (1.0 - fj) * (1.0 - vv))
                    * (fk * ww + (1.0 - fk) * (1.0 - ww))
                    * c[i][j][k].dot(&weight_v);
            }
        }
    }
    acc
}

fn trilinear_interpolation(c: [[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    let mut acc = 0.0;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let fi = i as f64;
                let fj = j as f64;
                let fk = k as f64;

                acc += (fi * u + (1.0 - fi) * (1.0 - u))
                    * (fj * v + (1.0 - fj) * (1.0 - v))
                    * (fk * w + (1.0 - fk) * (1.0 - w))
                    * c[i][j][k];
            }
        }
    }
    acc
}
