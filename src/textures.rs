use crate::vec_class::{Vec3, Color, Point3, dot};
use rand::Rng;

///Stores the different variants of solid textures. Variants include
/// 
/// Solid: simply renders a given color.
/// 
/// Checker: renders two different colors in a cherk-like pattern.
/// 
/// Noise: uses Perlin noise to render a pseudo-random texture of black and white.
/// 
/// Image: Renders an image onto a surface.
#[derive(Debug, Clone)]
pub enum Texture {
    Solid(Color),
    Checker(Color, Color),
    Noise(Perlin, f32),
    Image(Vec<u8>, u32, u32),
}

impl Texture {
    pub fn value(&self, u : f32, v : f32, p : Point3) -> Color {
        match self {
            Texture::Solid(c) => *c,
            Texture::Checker(odd, even) => {
                let sines = (p.x * 10.0).sin() * (p.y * 10.0).sin() * (p.z * 10.0).sin();
                if sines < 0.0 {
                    return *odd;
                } else {
                    return *even;
                }
            },
            Texture::Noise(per, scale) => Color::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + (*scale * p.z + 10.0*per.turb(p, 7)).sin()),
            Texture::Image(bytes, w, h) => {
                let width = *w;
                let height = *h;

                let u_bounded = if u < 0.0 {0.0} else if u > 1.0 {1.0} else {u};
                let v_bounded = if v < 0.0 {1.0} else if v > 1.0 {0.0} else {1.0 - v};
                let mut i = (u_bounded * width as f32) as u32;
                let mut j = (v_bounded * height as f32) as u32;

                i = i.min(width - 1);
                j = j.min(height - 1);

                let index = 3*j*width + 3*i;
                Color::new(bytes[index as usize] as f32 / 255.0, bytes[(index+1) as usize] as f32 / 255.0, bytes[(index+2) as usize] as f32 / 255.0)
            },
        }
    }
}

///Implements the concept of Perlin noise, a type of gradient noise developed
/// by Kevin Perlin to make procedural generation easier.
#[derive(Debug, Clone, Copy)]
pub struct Perlin {
    pub ranvec : [Vec3 ; 256],
    pub perm_x : [i32 ; 256],
    pub perm_y : [i32 ; 256],
    pub perm_z : [i32 ; 256],
}

impl Perlin {

    pub fn new() -> Perlin {
        let mut p = Perlin {ranvec : [Vec3::random() ; 256], perm_x : [0 ; 256], perm_y : [0 ; 256], perm_z : [0 ; 256]};
        for i in 0..256 {
            p.ranvec[i] = Vec3::random_range(-1.0,1.0).unit_vector();
        }
        Perlin::initialize(&mut p.perm_x);
        Perlin::initialize(&mut p.perm_y);
        Perlin::initialize(&mut p.perm_z);
        p
    }

    fn noise(&self, p : Point3) -> f32 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;
        let mut c : [[[Vec3 ; 2] ; 2] ; 2] = [[[Vec3::random() ; 2] ; 2] ; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di as usize][dj as usize][dk as usize] = self.ranvec[
                       (self.perm_x[((i + di) & 255) as usize] ^ 
                        self.perm_y[((j + dj) & 255) as usize] ^ 
                        self.perm_z[((k + dk) & 255) as usize]) 
                        as usize
                    ];
                }
            }
        }

        Perlin::trilinear_interp(&c, u, v, w)
    }

    fn trilinear_interp(c : &[[[Vec3 ; 2] ; 2] ; 2], u : f32, v : f32, w : f32) -> f32 {
        //Hermite's cubic to smooth noise
        let uu = u*u*(3.0 - 2.0*u);
        let vv = v*v*(3.0 - 2.0*v);
        let ww = w*w*(3.0 - 2.0*w);

        let mut accum = 0.0;
        for di in 0..=1 {
            for dj in 0..=1 {
                for dk in 0..=1 {
                    let i = di as f32;
                    let j = dj as f32;
                    let k = dk as f32;
                    let weight_v = Vec3::new(u-i, v-j, w-k);
                    let i_fac = i * uu + (1.0 - i)*(1.0 - uu);
                    let j_fac = j * vv + (1.0 - j)*(1.0 - vv);
                    let k_fac = k * ww + (1.0 - k)*(1.0 - ww);
                    accum += i_fac * j_fac * k_fac * dot(c[di][dj][dk], weight_v);
                }
            }
        }
        accum
    }

    pub fn turb(&self, p : Point3, depth : i32) -> f32 {
        let mut accum = 0.0;
        let mut temp_p = p;
        let mut weight = 1.0;
        for _i in 0..depth {
            accum += self.noise(temp_p) * weight;
            weight *= 0.5;
            temp_p *= 2.0;
        }
        accum.abs()
    }

    fn initialize(arr : &mut[i32 ; 256]) {
        for i in 0..256 {
            arr[i as usize] = i;
        }
        let mut rng = rand::thread_rng();
        for i in (1..=255).rev() {
            let target = rng.gen_range(0..(i+1)) as usize;
            let tmp = arr[i as usize];
            arr[i as usize] = arr[target];
            arr[target] = tmp;
        }
    }
}