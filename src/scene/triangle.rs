use std::cell::RefCell;
use std::rc::Rc;

use super::{Intersection, Material, Renderable};
use crate::math::{Matrix, Point3, Ray, Vector3};

pub struct Triangle {
    verts: Vec<Point3>,
    normal: Vector3,
    transform: Matrix,
    inv_transform: Matrix,
    material: Rc<RefCell<dyn Material>>,
}

impl Triangle {
    pub fn new(v0: &Point3, v1: &Point3, v2: &Point3, material: Rc<RefCell<dyn Material>>) -> Triangle {
        let verts = vec![*v0, *v1, *v2];

        let normal = {
            let v0v1 = v1 - v0;
            let v0v2 = v2 - v1;
            v0v1.cross(&v0v2).norm()
        };

        Triangle {
            verts,
            normal,
            transform: Matrix::identity(),
            inv_transform: Matrix::identity(),
            material: Rc::clone(&material),
        }
    }
}

impl Renderable for Triangle {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let v0v1 = self.verts[1] - self.verts[0];
        let v0v2 = self.verts[2] - self.verts[0];
        let pvec = ray.direction().cross(&v0v2);
        let det = v0v1.dot(&pvec);

        if det.abs() < std::f32::EPSILON {
            return None;
        }

        let inv_det = 1.0 / det;

        let tvec = ray.origin() - self.verts[0];
        let u = tvec.dot(&pvec) * inv_det;

        if u < 0. || u > 1. {
            return None;
        }

        let qvec = tvec.cross(&v0v1);
        let v = ray.direction().dot(&qvec) * inv_det;
        if v < 0. || u + v > 1. {
            return None;
        }

        let t = v0v2.dot(&qvec) * inv_det;

        if t < 0. {
            return None;
        }

        let normal = if t < 0. { -self.normal } else { self.normal };

        Some(Intersection {
            t,
            material: Rc::clone(&self.material),
            point: t * ray,
            eye_dir: -(ray.direction().norm()),
            normal,
            entering: det > 0.,
            tex_coord: (u, v),
        })
    }

    fn set_transform(&mut self, m: &Matrix) {
        self.transform = *m;
        self.inv_transform = m.inverse();
    }

    fn to_string(&self) -> String {
        "Triable".into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::Ray;
    use crate::scene::color::{colors::*, Color};
    use crate::scene::{Phong, PointLight};

    #[test]
    fn creation() {
        let material = Rc::new(RefCell::new(Phong::new(WHITE, WHITE, WHITE, 60., 0., 0.)));
        // CCW defined triangle the normal should point in the +Z axis
        let tri = Triangle::new(
            &Point3::new(0., 0., 0.),
            &Point3::new(1., 0., 0.),
            &Point3::new(0., 1., 0.),
            material.clone(),
        );
        assert_eq!(Vector3::new(0., 0., 1.), tri.normal);

        // CW defined triangle the normal should point in the -Z axis
        let tri = Triangle::new(
            &Point3::new(1., 0., 0.),
            &Point3::new(0., 0., 0.),
            &Point3::new(0., 1., 0.),
            material.clone(),
        );
        assert_eq!(Vector3::new(0., 0., -1.), tri.normal);
    }

    #[test]
    fn intersection() {
        // CW defined triangle the normal should point in the -Z axis
        let material = Rc::new(RefCell::new(Phong::new(WHITE, WHITE, WHITE, 60., 0., 0.)));
        let tri = Triangle::new(
            &Point3::new(2., -2., 0.),
            &Point3::new(-2., -2., 0.),
            &Point3::new(-2., 2., 0.),
            material,
        );

        let ray = Ray::new(&Point3::new(0., 0., -4.), &Vector3::new(0., 0., 1.));

        let i = tri.intersect(&ray);
        assert_eq!(true, i.is_some());
        let i = i.unwrap();

        assert_eq!(4.0, i.t);
        assert_eq!(Point3::new(0., 0., 0.), i.point);
        assert_eq!(Vector3::new(0., 0., -1.), i.normal);
        assert_eq!(Vector3::new(0., 0., -1.), i.eye_dir);
        assert_eq!(true, i.entering);
    }

    #[test]
    fn behind_ray_not_intersection() {
        // CW defined triangle the normal should point in the -Z axis
        let material = Rc::new(RefCell::new(Phong::new(WHITE, WHITE, WHITE, 60., 0., 0.)));
        let tri = Triangle::new(
            &Point3::new(2., -2., 0.),
            &Point3::new(-2., -2., 0.),
            &Point3::new(-2., 2., 0.),
            material,
        );

        let ray = Ray::new(&Point3::new(0., 0., -4.), &Vector3::new(0., 0., -1.));

        let i = tri.intersect(&ray);
        assert_eq!(false, i.is_some());
    }

    #[test]
    fn shading() {
        // CW defined triangle the normal should point in the -Z axis
        let material = Rc::new(RefCell::new(Phong::new(
            0.5 * WHITE,
            0.5 * WHITE,
            0.5 * WHITE,
            60.,
            0.,
            0.,
        )));
        let tri = Triangle::new(
            &Point3::new(2., -1., 0.),
            &Point3::new(-1., -1., 0.),
            &Point3::new(-1., 2., 0.),
            material,
        );

        let ray = Ray::new(&Point3::new(0., 0., -4.), &Vector3::new(0., 0., 1.));

        let i = tri.intersect(&ray);
        let i = i.unwrap();

        let light = PointLight::new(Point3::new(0., 0., -4.), Color::new(1., 1., 1.));
        let energy =
            tri.material.borrow()
                .get_reflected_energy(&light.color, &(light.pos - i.point).norm(), &i);

        assert_eq!(WHITE, energy);
    }
}
