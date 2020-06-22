/// A basic plane
use std::cell::*;
use std::rc::Rc;

use super::{Intersection, Material, Renderable};
use crate::math::{Matrix, Point3, Ray, Vector3};

pub struct Plane {
    id: i32,
    origin: Point3,
    normal: Vector3,
    material: Rc<RefCell<dyn Material>>,
    transform: Matrix,
    inv_transform: Matrix,

    // axes for the texture coordinates
    u: Vector3,
    v: Vector3,
}

impl Plane {
    pub fn new(origin: &Point3, normal: &Vector3, material: Rc<RefCell<dyn Material>>) -> Plane {
        let w = if normal.cross(&Vector3::new(1., 0., 0.)).len() <= std::f32::EPSILON {
            Vector3::new(0., 1., 0.)
        } else {
            Vector3::new(1., 0., 0.)
        };

        let u = normal.cross(&w).norm();
        let v = normal.cross(&u).norm();

        Plane {
            id: 0,
            origin: *origin,
            normal: *normal,
            material: Rc::clone(&material),
            transform: Matrix::identity(),
            inv_transform: Matrix::identity(),
            u,
            v,
        }
    }
}

impl Renderable for Plane {
    fn id(&self) -> i32 {
        self.id
    }

    fn set_id(&mut self, id: i32) {
        self.id = id;
    }

    fn set_transform(&mut self, m: &Matrix) {
        self.transform = *m;
        self.inv_transform = m.inverse();
    }

    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let transformed_ray = self.inv_transform * ray;
        let denom = -self.normal.dot(&transformed_ray.direction());
        if denom > std::f32::EPSILON {
            let dir = self.origin - transformed_ray.origin();
            let t = -dir.dot(&self.normal) / denom;

            let point = t * ray;
            let u = self.u.dot(&Vector3::from(point));
            let v = self.v.dot(&Vector3::from(point));
            let i = Intersection {
                t,
                entering: t >= 0.,
                point,
                eye_dir: -ray.direction().norm(),
                normal: (self.transform * self.normal),
                material: Rc::clone(&self.material),
                tex_coord: (u, v),
            };
            Some(i)
        } else {
            // Ray is parallel with the plane
            None
        }
    }

    fn get_name(&self) -> String {
        self.to_string()
    }

    fn get_material_mut(&mut self) -> Option<RefMut<dyn Material>> {
        Some(self.material.borrow_mut())
    }

    fn get_material(&self) -> Option<Ref<dyn Material>> {
        Some(self.material.borrow())
    }

    fn to_string(&self) -> String {
        "Plane".into()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::math::Vector3;
    use crate::scene::{
        color::{colors::WHITE, Color},
        material::TexturePhong,
        TextureCoords,
    };

    fn white(_: TextureCoords) -> Color {
        WHITE
    }

    #[test]
    fn texture_coords() {
        let phong = Rc::new(RefCell::new(TexturePhong::new(
            white, white, white, 60., 0., 0.,
        )));
        let normal = Vector3::new(0., 1., 0.);
        let plane = Plane::new(&Point3::new(0., 0., 0.), &normal, phong);

        assert_eq!(0., normal.dot(&plane.u));
    }
}
