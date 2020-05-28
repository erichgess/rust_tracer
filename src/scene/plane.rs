/// A basic plane

use crate::math::{Matrix, Point3, Ray, Vector3};
use super::{Intersection, Phong, Renderable};

pub struct Plane {
    origin: Point3,
    normal: Vector3,
    material: Phong,
    transform: Matrix,
    inv_transform: Matrix,

    // axes for the texture coordinates
    u: Vector3,
    v: Vector3
}

impl Plane {
    pub fn new(origin: &Point3, normal: &Vector3, material: &Phong) -> Plane {
        let w = if normal.dot(&Vector3::new(1., 0., 0.)).abs() <= std::f32::EPSILON {
            Vector3::new(1., 0., 0.)
        } else {
            Vector3::new(1., 0., 0.)
        };

        let u = Vector3::new(1., 0., 0.);
        let v = Vector3::new(0., 0., 1.);

        Plane {
            origin: *origin,
            normal: *normal,
            material: *material,
            transform: Matrix::identity(),
            inv_transform: Matrix::identity(),
            u,
            v,
        }
    }
}

impl Renderable for Plane {
    fn set_transform(&mut self, m: &Matrix) {
        self.transform = *m;
        self.inv_transform = m.inverse();
    }

    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let transformed_ray = self.inv_transform * ray;
        let denom = -self.normal.dot(&transformed_ray.direction());
        if denom > std::f32::EPSILON {
            let dir = self.origin - transformed_ray.origin();
            let t = -dir.dot(&self.normal)/denom;

            let point = t * ray;
            let u = self.u.dot(&Vector3::from(point));
            let v = self.v.dot(&Vector3::from(point));
            let i = Intersection{
                t: t,
                material: self.material,
                point: point,
                eye_dir: -ray.direction().norm(),
                normal: (self.transform * self.normal),
                entering: t >= 0.,
                tex_coord: (u, v),
            };
            Some(i)
        } else {
            // Ray is parallel with the plane
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::math::Vector3;
    use crate::scene::color::Color;
    use crate::scene::TextureCoords;

    fn white(_: TextureCoords) -> Color {
        Color::white()
    }

    #[test]
    fn texture_coords() {
        let phong = Phong::new(white, white, white, 60., 0., 0.);
        let normal = Vector3::new(0., 1., 0.);
        let plane = Plane::new(&Point3::new(0., 0., 0.), &normal, &phong);

        assert_eq!(0., normal.dot(&plane.u));
    }
}
