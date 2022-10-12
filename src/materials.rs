//Module to store the 'material' enum and its related methods

use crate::ray_class::Ray;
use crate::vec_class::{Color, Point3, dot,  random_in_unit_sphere};
use crate::hitting::HitRecord;
use rand::Rng;
use super::TEXTURE_LIST;

#[derive(Debug, Clone, Copy)]
///Represent the material of a particular object. This determines how rays and light interact with objects.
pub enum Material {
    Lambertian(usize),
    Metal(Color, f32),
    Dielectric(Color, f32),
    Light(usize),
    Isotropic(usize),
}

impl Material {
    ///Scatters the input ray according to an object's material, as well as where it landed.
    pub fn scatter(&self, r_in : Ray, rec : &HitRecord, attenuation : &mut Color, scattered : &mut Ray) -> bool {
        match self {
            Material::Lambertian(texture_id) => {
                let mut scatter_dir = rec.normal + random_in_unit_sphere();
                if scatter_dir.near_zero() {
                    scatter_dir = rec.normal;
                }
                *scattered = Ray::new(rec.p, scatter_dir);
                unsafe {
                    *attenuation = TEXTURE_LIST[*texture_id].value(rec.u, rec.v, rec.p);
                }
                true
            },
            Material::Metal(albedo, fuzz) => {
                let reflected = r_in.direction.unit_vector().reflect(rec.normal);
                *scattered = Ray::new(rec.p, reflected + random_in_unit_sphere() * (*fuzz));
                *attenuation = *albedo;
                dot(scattered.direction, rec.normal) > 0.0
            },
            Material::Dielectric(c, ir) => {
                *attenuation = *c;
                let refraction_ratio = if rec.front_facing {1.0 / *ir} else {*ir};
                let mut rng = rand::thread_rng();

                //Schlick's approximation for reflectance
                let reflectance = |cosine : f32, ref_idx : f32| {
                    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
                    r0 *= r0;
                    r0 + (1.0 - r0) * ((1.0 - cosine) as f32).powf(5.0)
                };

                let unit_direction = r_in.direction.unit_vector();
                let cos = if dot(-unit_direction, rec.normal) < 1.0 {dot(-unit_direction, rec.normal)} else {1.0};
                let sin = (1.0 - cos*cos).sqrt();
                let dir = if refraction_ratio * sin > 1.0 || reflectance(cos, refraction_ratio) > rng.gen::<f32>() {
                    unit_direction.reflect(rec.normal)
                } else {
                    unit_direction.refract(rec.normal, refraction_ratio)
                };

                *scattered = Ray::new(rec.p, dir);
                true
            },
            Material::Isotropic(texture_id) => {
                *scattered = Ray::new(rec.p, random_in_unit_sphere());
                unsafe {
                    *attenuation = TEXTURE_LIST[*texture_id].value(rec.u, rec.v, rec.p);
                }
                true
            },
            _ => false,
        }
    }

    pub fn emitted(&self, u : f32, v : f32, p : Point3) -> Color {
        match self {
            Material::Light(texture_id) => unsafe {TEXTURE_LIST[*texture_id].value(u, v, p)},
            _ => Color::new(0.0, 0.0, 0.0),
        }
    }
}