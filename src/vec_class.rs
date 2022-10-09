use std::{ops::{Add, Sub, Mul, Div, AddAssign, MulAssign, DivAssign, IndexMut, Index, Neg}, f32::consts::PI};
use rand::Rng;

 ///Used to keep track of 3-dimensional vector data.
#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x : f32,
    pub y : f32,
    pub z : f32,
}

///Represents a 3D point in vector space.
pub type Point3 = Vec3;

///Represents an RGB-based color value.
pub type Color = Vec3;

//These methods make the most sense to use when a vec3 instance is being used as a vector, rather than a point or color.
impl Vec3 {

    ///Creates a new 3-dimensional vector, point, or color.
    pub fn new(x : f32, y : f32, z : f32) -> Vec3 {
        Vec3 {
            x,
            y,
            z,
        }
    }

    ///Returns the square of the length of this vector.
    pub fn length_squared(&self) -> f32 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    ///Returns the length of this vector.
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    ///Returns the unit vector of this vector.
    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }

    ///Returns a random vector, point or color, with all 3 parameters being random numbers between 0 and 1 non-inclusive.
    pub fn random() -> Vec3 {
        Vec3 {
            x : rand::random::<f32>(),
            y : rand::random::<f32>(),
            z : rand::random::<f32>(),
        }
    }

    ///Returns a random vector, point or color, with all 3 parameters being random numbers between a minimum and a maximum non-inclusive.
    pub fn random_range(minimum : f32, maximum : f32) -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3 {
            x : rng.gen_range(minimum..maximum),
            y : rng.gen_range(minimum..maximum),
            z : rng.gen_range(minimum..maximum),
        }
    }

    ///Returns whether the vector's values are all close to 0. Near zero is needed due to floating point error.
    pub fn near_zero(&self) -> bool {
        let s : f32 = 0.0001;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }

    pub fn reflect(&self, n : Vec3) -> Vec3 {
        *self -  n * 2.0 * dot(*self, n)
    }

    pub fn refract(&self, n : Vec3, etai_over_etat : f32) -> Vec3 {
        let cos = if dot(-(*self), n) < 1.0 {dot(-(*self), n)} else {1.0};
        let r_perp = (*self + n * cos) * etai_over_etat;
        let r_parallel = n * -((1.0 - r_perp.length_squared()).abs().sqrt());
        r_perp + r_parallel
    }

}

impl Index<usize> for Vec3 {
    type Output = f32;
    fn index(&self, index : usize) -> &f32 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds!: {}", index),
        }
    }
} 

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index : usize) -> &mut f32 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of bounds!: {}", index),
        }
    }
} 

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other : Self) -> Self::Output {
        Vec3 {
            x : self.x + other.x,
            y : self.y + other.y,
            z : self.z + other.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other : Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other : Self) -> Self::Output {
        Vec3 {
            x : self.x - other.x, 
            y : self.y - other.y, 
            z : self.z - other.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, other : Self) -> Self::Output {
        Vec3 {
            x : self.x * other.x, 
            y : self.y * other.y, 
            z : self.z * other.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, other : f32) -> Self::Output {
        Vec3 {
            x : self.x * other, 
            y : self.y * other, 
            z : self.z * other,
        }
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, other : f32) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, other : f32) -> Self::Output {
        Vec3 {
            x : self.x / other, 
            y : self.y / other, 
            z : self.z / other,
        }
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, other : f32) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Vec3 {
            x : -self.x,
            y : -self.y,
            z : -self.z,
        }
    }
}

pub fn dot(v1 : Vec3, v2 : Vec3) -> f32 {
    (v1.x * v2.x) + (v1.y * v2.y) + (v1.z * v2.z)
}

pub fn cross(v1 : Vec3, v2 : Vec3) -> Vec3 {
    Vec3 {
        x : (v1.y * v2.z) - (v1.z * v2.y),
        y : (v1.z * v2.x) - (v1.x * v2.z),
        z : (v1.x * v2.y) - (v1.y * v2.x),
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    let r1 = rng.gen::<f32>();
    let r2 = rng.gen::<f32>();
    Vec3::new((2.0 * PI * r1).cos() * 2.0 * (r2 * (1.0 - r2)).sqrt(), (2.0 * PI * r1).sin() * 2.0 * (r2 * (1.0 - r2)).sqrt(), 1.0 - (2.0 * r2))
}

pub fn random_in_unit_disk() -> Vec3 {
    let mut p;
    loop {
        p = Vec3::random_range(-1.0, 1.0);
        p.z = 0.0;
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}