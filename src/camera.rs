// Module to store the camera.

use crate::ray_class::Ray;
use crate::vec_class::{Vec3, Point3, cross, random_in_unit_disk};
use core::f32::consts::PI;

fn degrees_to_radians(degrees : f32) -> f32 {
    degrees * PI / 180.0
}

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub origin : Point3,
    pub lower_left_corner : Point3,
    pub horizontal : Vec3,
    pub vertical : Vec3,
    pub u : Vec3,
    pub v : Vec3,
    pub w : Vec3,
    pub lens_radius : f32,
}

impl Camera {
    pub fn new(lookfrom : Point3, lookat : Point3, vup : Vec3, vfov : f32, aspect_ratio : f32, aperture : f32, focus_dist : f32) -> Camera {
        let theta = degrees_to_radians(vfov);
        let h = (theta/2.0).tan();

        let viewport_height = 2.0*h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit_vector();
        let u = cross(vup, w).unit_vector();
        let v = cross(w, u);

        let hor = u * viewport_width * focus_dist;
        let ver = v * viewport_height * focus_dist;
        let llc = lookfrom - hor/2.0 - ver/2.0 - w*focus_dist;
        Camera {
            origin : lookfrom,
            lower_left_corner : llc,
            horizontal : hor, 
            vertical : ver,
            u,
            v,
            w,
            lens_radius : aperture / 2.0,
        }
    }

    pub fn get_ray(&self, u : f32, v : f32) -> Ray {
        let rd = random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(self.origin + offset, self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin - offset)
    }
}