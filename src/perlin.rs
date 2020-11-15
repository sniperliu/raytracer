use crate::vec3::Point3;
use rand::{self, Rng};

const POINT_COUNT: i32 = 256;

pub struct Perlin {
    ran_float: Vec<f32>,
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
        let ran_float = (0..POINT_COUNT).map(|_| rng.gen()).collect();

        Self {
            ran_float: ran_float,
            perm_x: Self::perlin_generate_perm(),
            perm_y: Self::perlin_generate_perm(),
            perm_z: Self::perlin_generate_perm(),
        }
    }

    pub fn noise(&self, p: &Point3) -> f32 {
        // FIXME f32 -> i32 may overflow
        let i = (4. * p.x) as i32 & 255;
        let j = (4. * p.y) as i32 & 255;
        let k = (4. * p.z) as i32 & 255;

        self.ran_float[(self.perm_x[i as usize] ^
                        self.perm_y[j as usize] ^
                        self.perm_z[k as usize]) as usize]
    }
}
