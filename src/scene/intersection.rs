use std::cell::RefCell;
use std::rc::Rc;

use super::{Material, TextureCoords};
use crate::math::{Point3, Vector3};

#[derive(Clone)]
pub struct Intersection {
    pub id: i32,
    pub t: f32,
    pub material: Rc<RefCell<dyn Material>>,
    pub point: Point3,
    pub eye_dir: Vector3,
    pub normal: Vector3,
    pub entering: bool,
    pub tex_coord: TextureCoords,
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Intersection) -> bool {
        (self.t - other.t).abs() < std::f32::EPSILON
    }
}

impl PartialOrd for Intersection {
    fn partial_cmp(&self, other: &Intersection) -> Option<std::cmp::Ordering> {
        let diff = self.t - other.t;
        if diff.abs() < std::f32::EPSILON {
            Some(std::cmp::Ordering::Equal)
        } else if self.t < other.t {
            Some(std::cmp::Ordering::Less)
        } else {
            Some(std::cmp::Ordering::Greater)
        }
    }
}

impl Eq for Intersection {}

impl Ord for Intersection {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let diff = self.t - other.t;
        if diff.abs() < std::f32::EPSILON {
            std::cmp::Ordering::Equal
        } else if self.t < other.t {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    }
}
