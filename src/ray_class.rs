/*
Module to store the 'ray' class and its related methods.
*/

use crate::vec_class::{Color, Point3, Vec3, dot};
use crate::hitting::HitRecord;
use crate::tree::Tree;

///Implementation of rays. Primary structure responsible for the ray tracing effects generated.
#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin_point : Point3,
    pub direction : Vec3,
}

impl Ray {

    ///Initializes a new ray, given a starting point and a direction.
    pub fn new(o : Point3, d : Vec3) -> Ray {
        Ray {
            origin_point : o,
            direction : d,
        }
    }

    /// Returns the point at which this ray would be after a certain period.
    pub fn at(&self, ti : f32) -> Point3 {
        self.origin_point + self.direction * ti
    }

    /// Determines at what point, if any, this ray would hit a sphere.
    pub fn hit_sphere(&self, center : Point3, radius : f32) -> f32 {
        let oc = self.origin_point - center;
        let a : f32 = dot(self.direction, self.direction);
        let half_b : f32 = dot(oc, self.direction);
        let c : f32 = dot(oc, oc) - radius * radius;
        let discriminant : f32 = half_b * half_b - a * c;
        if discriminant < 0.0 {
            -1.0
        } else {
            (-half_b - discriminant.sqrt()) / a
        }
    }

    /// Determines the color of the ray.
    /// 
    /// Based on a variety of factors, including:
    /// 
    /// -position of the ray
    /// 
    /// -whether the ray has hit an object
    /// 
    /// -what kind of object, if any, the ray has hit
    /// 
    /// -the lighting of the surrounding area
    pub fn ray_color(&self, objs : &Tree, depth : i32) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        let mut rec : HitRecord = HitRecord::new();
        if objs.hit(*self, 0.001, f32::INFINITY, &mut rec, objs.root) {
            let mut scattered = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
            let mut attenuation = Color::new(0.0, 0.0, 0.0);
            let emitted = rec.mat.emitted(rec.u, rec.v, rec.p);
            if !rec.mat.scatter(*self, &rec, &mut attenuation, &mut scattered) {
                return emitted;
            } 
            return emitted + attenuation * scattered.ray_color(objs, depth-1);
        }
        return Color::new(0.0, 0.0, 0.0);
    }

}