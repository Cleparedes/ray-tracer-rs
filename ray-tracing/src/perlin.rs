use crate::interval::Interval;
use crate::utilities::random_int;
use crate::vec3::{self, dot, unit_vector, Point3, Vec3};

#[derive(Clone)]
pub struct Perlin {
    point_count: usize,
    rand_vec: Vec<Vec3>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    pub fn new() -> Self {
        let mut result = Self::default();
        for i in 0..result.point_count {
            result.rand_vec[i] = unit_vector(&vec3::random(Some(Interval::new(-1.0, 1.0))))
        }
        result.perlin_generate_perm(0);
        result.perlin_generate_perm(1);
        result.perlin_generate_perm(2);
        result
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let u: f64 = p.x() - p.x().floor();
        let v: f64 = p.y() - p.y().floor();
        let w: f64 = p.z() - p.z().floor();

        let i: usize = (p.x().floor() + self.point_count as f64 / 2.0) as usize;
        let j: usize = p.y().floor() as usize;
        let k: usize = (p.z().floor() + self.point_count as f64 / 2.0) as usize;

        let mut c: Vec<Vec<Vec<Vec3>>> = vec![vec![vec![Vec3::default(); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.rand_vec[
                        self.perm_x[(i + di) & 255]
                        ^ self.perm_y[(j + dj) & 255]
                        ^ self.perm_z[(k + dk) & 255]
                    ]
                }
            }
        }

        self.perlin_interp(c, u, v, w)
    }

    pub fn turb(&self, p: &Point3, depth: i32) -> f64 {
        let mut accum: f64 = 0.0;
        let mut temp_p: Point3 = *p;
        let mut weight: f64 = 1.0;
        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }
        accum.abs()
    }

    fn perlin_generate_perm(&mut self, axis: i32) -> () {
        let mut perm: Vec<usize> = Vec::from_iter(0..self.point_count);
        perm = self.permute(&mut perm);
        match axis {
            0 => self.perm_x = perm,
            1 => self.perm_y = perm,
            _ => self.perm_z = perm,
        }
    }

    fn permute(&self, perm: &mut Vec<usize>) -> Vec<usize> {
        for i in (0..self.point_count).rev() {
            let target: usize = random_int(0, i as i32) as usize;
            let tmp: usize = perm[i];
            perm[i] = perm[target];
            perm[target] = tmp;
        }
        perm.clone()
    }

    fn perlin_interp(&self, c: Vec<Vec<Vec<Vec3>>>, u: f64, v: f64, w: f64) -> f64 {
        let uu: f64 = u * u * (3.0 - 2.0 * u);
        let vv: f64 = v * v * (3.0 - 2.0 * v);
        let ww: f64 = w * w * (3.0 - 2.0 * w);
        let mut accum: f64 = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_vector = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += (i as f64 * uu + (1.0 - i as f64) * (1.0 - uu))
                    * (j as f64 * vv + (1.0 - j as f64) * (1.0 - vv))
                    * (k as f64 * ww + (1.0 - k as f64) * (1.0 - ww))
                    * dot(&c[i][j][k], &weight_vector);
                }
            }
        }
        accum
    }
}

impl Default for Perlin {
    fn default() -> Self {
        let point_count: usize = 256;
        Self { 
            point_count, 
            rand_vec: vec![Vec3::default(); point_count], 
            perm_x: vec![0; point_count], 
            perm_y: vec![0; point_count], 
            perm_z: vec![0; point_count], 
        }
    }
}