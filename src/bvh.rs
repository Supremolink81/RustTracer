/*
Module to store Bounding Volume Hierarchies (BVH) for collision optimization (from O(n) to O(log2 n))
*/

use crate::ray_class::Ray;
use crate::vec_class::Point3;

#[derive(Debug, Clone, Copy)]
pub struct AABB {
    pub minimum : Point3,
    pub maximum : Point3,
}

impl AABB {

    pub fn new(minimum : Point3, maximum : Point3) -> AABB {
        AABB {
            minimum,
            maximum,
        }
    }

    pub fn hit(&self, r : Ray, t_min : f32, t_max : f32) -> bool {
        let mut t_mi = t_min;
        let mut t_ma = t_max;
        for i in 0..3 {
            let mut t0 = (self.minimum[i] - r.origin_point[i]) / r.direction[i];
            let mut t1 = (self.maximum[i] - r.origin_point[i]) / r.direction[i];
            if r.direction[i] < 0.0 {
                (t0, t1) = (t1, t0);
            }
            t_mi = t_mi.max(t0);
            t_ma = t_ma.min(t1);
            if t_ma <= t_mi {
                return false;
            }
        }
        return true;
    }

}

pub fn surrounding_box(box0 : AABB, box1 : AABB) -> AABB {
    let small = Point3::new(
        box0.minimum.x.min(box1.minimum.x), 
        box0.minimum.y.min(box1.minimum.y), 
        box0.minimum.z.min(box1.minimum.z)
    );
    let big = Point3::new(
        box0.maximum.x.max(box1.maximum.x), 
        box0.maximum.y.max(box1.maximum.y), 
        box0.maximum.z.max(box1.maximum.z)
    );
    AABB::new(small, big)
}