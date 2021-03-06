use crate::vec3::{Vec3, Point3};
use rand::{self, Rng};

const POINT_COUNT: i32 = 256;

pub struct Perlin {
    ran_vec: Vec<Vec3>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}

impl Perlin {
    fn permute(p: &mut Vec<i32>) {
        let mut rng = rand::thread_rng();

        // TODO Refactor later to use more ideal way, e.g. reverse & iterator
        let mut i = POINT_COUNT - 1;

        while i > 0 {
            let target = rng.gen_range(0, i);
            p.swap(i as usize, target as usize);
            i -= 1;
        }
    }

    pub fn perlin_generate_perm() -> Vec<i32> {
        let mut p: Vec<i32> = (0..POINT_COUNT).collect();

        Self::permute(&mut p);

        p
    }

    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let ran_vec = (0..POINT_COUNT)
            .map(|_| Vec3{ x: rng.gen_range(-1., 1.),
                           y: rng.gen_range(-1., 1.),
                           z: rng.gen_range(-1., 1.)}
                       .normalize())
            .collect();

        Self {
            ran_vec: ran_vec,
            perm_x: Self::perlin_generate_perm(),
            perm_y: Self::perlin_generate_perm(),
            perm_z: Self::perlin_generate_perm(),
        }
    }

    pub fn noise(&self, p: &Point3) -> f32 {
        let mut u = p.x - p.x.floor();
        let mut v = p.y - p.y.floor();
        let mut w = p.z - p.z.floor();

        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;

        let mut c: [[[Vec3; 2];2];2] = [[[Vec3{ x: 0., y: 0., z: 0.}; 2]; 2]; 2];

        for di in 0i32..2 {
            for dj in 0i32..2 {
                for dk in 0i32..2 {
                    c[di as usize][dj as usize][dk as usize] =
                        self.ran_vec[(self.perm_x[((i + di) & 255) as usize] ^
                                      self.perm_y[((j + dj) & 255) as usize] ^
                                      self.perm_z[((k + dk) & 255) as usize]) as usize];
                }
            }
        }

        perlin_interp(c, u, v, w)
    }

    pub fn turb(&self, p: &Point3, depth: usize) -> f32 {
        let mut accum = 0.;
        let mut temp_p = *p;
        let mut weight = 1.;

        for _i in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p = 2. * temp_p;
        }

        accum.abs()
    }
}

fn trilinear_interp(c: [[[f32; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
    let mut accum: f32 = 0.;

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                accum += (i as f32 * u + ((1 - i) as f32) * (1. - u)) *
                    (j as f32 * v + ((1 - j) as f32) * (1. - v)) *
                    (k as f32 * w + ((1 - k) as f32) * (1. - w)) * c[i][j][k];
            }
        }
    }

    accum
}

fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
    let uu = u * u * (3. - 2. * u);
    let vv = v * v * (3. - 2. * v);
    let ww = w * w * (3. - 2. * w);

    let mut accum = 0.;

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let weight_v = Vec3{ x: u - i as f32, y: v - j as f32, z: w - k as f32, };
                accum += (i as f32 * uu + (1 - i) as f32 * (1. - uu))
                    * (j as f32 * vv + (1 - j) as f32 * (1. - vv))
                    * (k as f32 * ww + (1 - k) as f32 * (1. - ww))
                    * c[i][j][k].dot(weight_v);
            }
        }
    }

    accum
}
