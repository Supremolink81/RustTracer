use crate::hitting::{Hittable, HitRecord};
use crate::bvh::{AABB, surrounding_box};
use crate::ray_class::Ray;
use std::cmp::Ordering;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Node {
    left : Option<usize>,
    right : Option<usize>,
    aabb : Option<AABB>,
    data : Option<Hittable>,
}

impl Node {
    fn new(left : Option<usize>, right : Option<usize>, aabb : Option<AABB>, data : Option<Hittable>) -> Node {
        Node {
            left, 
            right,
            aabb,
            data,
        }
    }
}

///Represents a Bounding Volume Hierarchy of the objects in the scene. Allows
/// 
/// ray collisions to be detected in O(log2 n) time.
#[derive(Debug, Clone)]
pub struct Tree {
    pub items : Vec<Node>,
    pub root : usize,
}

impl Tree {
    ///Builds a Bounding Volume Hierarchy from a list of Hittavle objects.
    pub fn build(lst : &mut Vec<Hittable>) -> Tree {
        let mut t = Tree{items : vec![], root : 0};
        t.root = t.con(lst);
        t
    }

    ///Creates a new node with two children.
    fn new_node(& mut self, aabb : Option<AABB>, left : Option<usize>, right : Option<usize>) -> usize {
        let next = self.items.len();
        self.items.push(Node::new(left, right, aabb, None));
        next
    }

    ///Creates a new leaf node containing a Hittable object.
    fn new_leaf(&mut self, item : &Hittable) -> usize {
        let next = self.items.len();
        self.items.push(Node::new(None, None, Some(item.bounding_box()), Some(item.clone())));
        next
    }

    ///Recursive helper function that constructs a new Bounding Volume Hierarchy from the input slice.
    fn con(&mut self, objects : &mut [Hittable]) -> usize {
        let axis = rand::thread_rng().gen_range(0..3) as usize;
        objects.sort_by(|a : &Hittable, b : &Hittable| cmp(a, b, axis));

        let left : usize;
        let right : usize;

        if objects.len() == 1 {
            return self.new_leaf(&objects[0]);
        } else if objects.len() == 2 {
            left = self.new_leaf(&objects[0]);
            right = self.new_leaf(&objects[1]);
        } else {
            let mid = objects.len() / 2;
            let (left_l, right_l) = objects.split_at_mut(mid);
            left = self.con(left_l);
            right = self.con(right_l);
        }

        if let Some(r_box) = self.items[left].aabb {
            if let Some(l_box) = self.items[right].aabb {
                return self.new_node(Some(surrounding_box(r_box, l_box)), Some(left), Some(right));
            }
        }

        self.new_node(None, None, None)
    }

    ///Determines if a ray hits any object in the Bounding Volume Hierarchy.
    pub fn hit(& self, r : Ray, t_min : f32, t_max : f32, rec : &mut HitRecord, index : usize) -> bool {
        let node = &self.items[index];
        if let Some(aabb) = node.aabb {
            if aabb.hit(r, t_min, t_max) {
                if let Some(d) = &node.data {
                    return d.hit(r, t_min, t_max, rec);
                }

                let mut rec_l : HitRecord = rec.clone();
                let mut rec_r : HitRecord = rec.clone();

                let hit_l = match node.left {
                    Some(left) => self.hit(r, t_min, t_max, &mut rec_l, left),
                    None => false,
                };

                let hit_r = match node.right {
                    Some(right) => self.hit(r, t_min, if hit_l {rec_l.t} else {t_max}, &mut rec_r, right),
                    None => false,
                };
                
                if hit_l && !hit_r {
                    *rec = rec_l.clone();
                } else if !hit_l && hit_r {
                    *rec = rec_r.clone();
                } else {
                    if rec_l.t < rec_r.t {
                        *rec = rec_l.clone();
                    } else {
                        *rec = rec_r.clone();
                    }
                }

                return hit_l || hit_r;
            }
        }
        false
    }
}

///Custom comparator function for two Hittable objects (based on location).
pub fn cmp(a : &Hittable, b : &Hittable, index : usize) -> Ordering {
    if a.bounding_box().minimum[index] < b.bounding_box().minimum[index] {
        return Ordering::Less;
    } else if a.bounding_box().minimum[index] > b.bounding_box().minimum[index] {
        return Ordering::Greater;
    } 
    Ordering::Equal
}