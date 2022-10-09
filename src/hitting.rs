use std::f64::consts::PI;
use crate::ray_class::Ray;
use crate::vec_class::{Vec3, Point3, dot};
use crate::materials::Material;
use crate::bvh::AABB;
use libm::{acos, atan2};

///Helper struct to store records of ray collisions between surfaces.
#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    pub p : Point3,
    pub normal : Vec3,
    pub mat : Material,
    pub t : f32,
    pub u : f32,
    pub v : f32,
    pub front_facing : bool,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord{
            p : Point3::new(0.0, 0.0, 0.0), 
            normal : Vec3::new(0.0, 0.0, 0.0), 
            t : 0.0, front_facing : false, 
            mat : Material::Lambertian(usize::MAX),
            u : 0.0,
            v : 0.0,
        }
    }

    pub fn set_front_face_normal(&mut self, r : Ray, outward_normal : Vec3) {
        self.front_facing = dot(r.direction, outward_normal) < 0.0;
        if self.front_facing {
            self.normal = outward_normal;
        } else {
            self.normal = -outward_normal;
        }
    }
}

///Representation of objects within scenes. Possible object types include:
/// 
/// Sphere: a 3-dimensional sphere with uniform radius.
/// 
/// XYRect: a 2-dimensional rectangle positioned at a specific z-coordinate.
/// 
/// XZRect: a 2-dimensional rectangle positioned at a specific y-coordinate.
/// 
/// YZRect: a 2-dimensional rectangle positioned at a specific x-coordinate.
/// 
/// Medium: a constant medium that produces a fog-like effect.
#[derive(Debug, Clone)]
pub enum Hittable {
    Sphere(Material, Point3, f32),
    XYRect(Material, f32, f32, f32, f32, f32),
    XZRect(Material, f32, f32, f32, f32, f32),
    YZRect(Material, f32, f32, f32, f32, f32),
    Box(Material, Point3, Point3),
    Medium(Material, Box<Hittable>, f32),
}

impl Hittable {

    ///Determines if a ray hits this Hittable object.
    /// 
    /// 
    /// 
    /// A mutable HitRecord reference is also passed as argument,
    /// so that if the function returns true, there is data regarding the details of the collision.
    /// (Note: medium collision is still under construction and doesn't fully work)
    pub fn hit(&self, r : Ray, t_min : f32, t_max : f32, rec : &mut HitRecord) -> bool {
        match self {
            Hittable::Sphere(mat, center, radius) => {
                let oc = r.origin_point - *center;
                let a = r.direction.length_squared();
                let half_b = dot(oc, r.direction);
                let c = oc.length_squared() - radius * radius;

                let discriminant = half_b * half_b - a * c;
                if discriminant < 0.0 {
                    return false;
                }
                let mut root = (-half_b - discriminant.sqrt()) / a;
                if root < t_min || t_max < root {
                    root = (-half_b + discriminant.sqrt()) / a;
                    if root < t_min || t_max < root {
                        return false;
                    }
                }

                //Record initialization
                rec.t = root;
                rec.p = r.at(rec.t);
                let outward_normal : Vec3 = (rec.p - *center) / *radius;
                rec.set_front_face_normal(r, outward_normal);
                rec.mat = *mat;
                self.get_uv(outward_normal, &mut rec.u, &mut rec.v);
                
                true
            },
            Hittable::XYRect(mat, x0, x1, y0, y1, k) => {
                let t = (*k - r.origin_point.z) / r.direction.z;
                if t < t_min || t > t_max {
                    return false;
                }
                let x = r.origin_point.x + t*r.direction.x;
                let y = r.origin_point.y + t*r.direction.y;
                if x < *x0 || x > *x1 || y < *y0 || y > *y1 {
                    return false;
                }

                //Record initialization
                rec.u = (x - *x0) / (*x1 - *x0);
                rec.v = (y - *y0) / (*y1 - *y0);
                rec.t = t;
                rec.mat = *mat;
                rec.p = r.at(t);
                rec.set_front_face_normal(r, Vec3::new(0.0, 0.0, 1.0));
                
                true
            },
            Hittable::XZRect(mat, x0, x1, z0, z1, k) => {
                let t = (*k - r.origin_point.y) / r.direction.y;
                if t < t_min || t > t_max {
                    return false;
                }
                let x = r.origin_point.x + t*r.direction.x;
                let z = r.origin_point.z + t*r.direction.z;
                if x < *x0 || x > *x1 || z < *z0 || z > *z1 {
                    return false;
                }

                //Record initialization
                rec.u = (x - *x0) / (*x1 - *x0);
                rec.v = (z - *z0) / (*z1 - *z0);
                rec.t = t;
                rec.mat = *mat;
                rec.p = r.at(t);
                rec.set_front_face_normal(r, Vec3::new(0.0, 1.0, 0.0));

                true
            },
            Hittable::YZRect(mat, y0, y1, z0, z1, k) => {

                let t = (*k - r.origin_point.x) / r.direction.x;

                //Make sure t is valid
                if t < t_min || t > t_max {
                    return false;
                }
                let y = r.origin_point.x + t*r.direction.x;
                let z = r.origin_point.z + t*r.direction.z;

                //Check to see if the expected y and z values are valid
                if y < *y0 || y > *y1 || z < *z0 || z > *z1 {
                    return false;
                }

                //Hit record initialization
                rec.u = (y - *y0) / (*y1 - *y0);
                rec.v = (z - *z0) / (*z1 - *z0);
                rec.t = t;
                rec.mat = *mat;
                rec.p = r.at(t);
                rec.set_front_face_normal(r, Vec3::new(1.0, 0.0, 0.0));

                true
            },
            Hittable::Medium(mat, b, density) => {

                let mut rec1 = HitRecord::new();
                let mut rec2 = HitRecord::new();

                //Make sure rays are hitting object
                if !b.hit(r, -f32::MAX, f32::MAX, &mut rec1) {
                    return false;
                }
                if !b.hit(r, rec1.t+0.0001, f32::MAX, &mut rec2) {
                    return false;
                }

                //Ensure record distances are within appropriate bounds.
                rec1.t = rec1.t.max(t_min);
                rec2.t = rec2.t.min(t_max);
                if rec1.t >= rec2.t {
                    return false;
                }
                rec1.t = rec1.t.max(0.0);

                let distance_inside_boundary = (rec2.t - rec1.t) * r.direction.length();
                let hit_distance = rand::random::<f32>().ln() / -(*density);

                if hit_distance > distance_inside_boundary {
                    return false;
                }

                //Hit record initialization
                rec.t = rec1.t + hit_distance / r.direction.length();
                rec.p = r.at(rec.t);
                rec.normal = Vec3::new(1.0, 0.0 , 0.0);
                rec.front_facing = true;
                rec.mat = *mat;

                return true;
            },
            Hittable::Box(mat, minimum, maximum) => {
                
                //Initialize sides of box
                let side1 = Hittable::XYRect(*mat, minimum.x, maximum.x, minimum.y, maximum.y, minimum.z);
                let side2 = Hittable::XYRect(*mat, minimum.x, maximum.x, minimum.y, maximum.y, maximum.z);
                let side3 = Hittable::XZRect(*mat, minimum.x, maximum.x, minimum.z, maximum.z, minimum.y);
                let side4 = Hittable::XZRect(*mat, minimum.x, maximum.x, minimum.z, maximum.z, maximum.y);
                let side5 = Hittable::YZRect(*mat, minimum.y, maximum.y, minimum.z, maximum.z, minimum.x);
                let side6 = Hittable::YZRect(*mat, minimum.y, maximum.y, minimum.z, maximum.z, maximum.x);

                //Keep track of closest collision out of the sides
                let mut temp_rec = HitRecord::new();
                let mut closest = t_max;
                let mut hit_something = false;

                //Check collisions with each side
                if side6.hit(r, t_min, closest, &mut temp_rec) {
                    hit_something = true;
                    closest = temp_rec.t;
                    *rec = temp_rec;
                }
                if side4.hit(r, t_min, closest, &mut temp_rec) {
                    hit_something = true;
                    closest = temp_rec.t;
                    *rec = temp_rec;
                }
                if side2.hit(r, t_min, closest, &mut temp_rec) {
                    hit_something = true;
                    closest = temp_rec.t;
                    *rec = temp_rec;
                }
                if side5.hit(r, t_min, closest, &mut temp_rec) {
                    hit_something = true;
                    closest = temp_rec.t;
                    *rec = temp_rec;
                }
                if side3.hit(r, t_min, closest, &mut temp_rec) {
                    hit_something = true;
                    closest = temp_rec.t;
                    *rec = temp_rec;
                }
                if side1.hit(r, t_min, closest, &mut temp_rec) {
                    hit_something = true;
                    closest = temp_rec.t;
                    *rec = temp_rec;
                }

                //True if at least one side was hit
                hit_something
            },
        }
    }

    ///Gets the bounding box of this Hittable object.
    pub fn bounding_box(&self) -> AABB {
        match self {
            Hittable::Sphere(_mat, center, radius) => AABB::new(*center - Vec3::new(*radius, *radius, *radius), *center + Vec3::new(*radius, *radius, *radius)),
            Hittable::XYRect(_mat, x0, x1, y0, y1, k) => AABB::new(Point3::new(*x0, *y0, *k-0.001), Point3::new(*x1, *y1, k+0.001)),
            Hittable::XZRect(_mat, x0, x1, z0, z1, k) => AABB::new(Point3::new(*x0, *k-0.001, *z0), Point3::new(*x1, *k+0.001, *z1)),
            Hittable::YZRect(_mat, y0, y1, z0, z1, k) => AABB::new(Point3::new(*k-0.001, *y0, *z0), Point3::new(*k+0.001, *y1, *z1)),
            Hittable::Medium(_mat, b, _density) => (**b).bounding_box(),
            Hittable::Box(_mat, minimum, maximum) => AABB::new(*minimum, *maximum),
        }
    }
    
    ///Retrieves the appropriate u and v values for spheres (for use in determining color values).
    pub fn get_uv(&self, p : Point3, u : &mut f32, v : &mut f32) {
        match self {
            Hittable::Sphere(_point, _radius, _mat) => {
                let theta = acos(-p.y as f64);
                let phi = atan2(-p.z as f64, p.x as f64) + PI;

                *u = (phi / (2.0 * PI)) as f32;
                *v = (theta / PI) as f32;
            },
            _ => (),
        };
    }
}