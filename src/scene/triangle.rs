use super::{Intersection, Phong, Renderable};
/// Render a single triangle
use crate::math::{Matrix, Point3, Ray, Vector3};

pub struct Triangle {
    verts: Vec<Point3>,
    normal: Vector3,
    transform: Matrix,
    inv_transform: Matrix,
    material: Phong,
}

impl Triangle {
    pub fn new(v0: &Point3, v1: &Point3, v2: &Point3, material: &Phong) -> Triangle {
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
            material: *material,
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
        if v < 0. || v > 1. {
            return None;
        }

        let t = v0v2.dot(&qvec) * inv_det;

        Some(Intersection {
            t,
            material: self.material,
            point: t * ray,
            eye_dir: ray.direction().norm().neg(),
            normal: self.normal,
            entering: det > 0.,
            tex_coord: (u, v),
        })
    }

    fn set_transform(&mut self, m: &Matrix) {
        self.transform = *m;
        self.inv_transform = m.inverse();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scene::color::Color;
    use crate::scene::TextureCoords;

    fn white(_: TextureCoords) -> Color {
        Color::white()
    }
    #[test]
    fn creation() {
        let material = Phong::new(white, white, white, 60., 0., 0.);
        // CCW defined triangle the normal should point in the +Z axis
        let tri = Triangle::new(
            &Point3::new(0., 0., 0.),
            &Point3::new(1., 0., 0.),
            &Point3::new(0., 1., 0.),
            &material,
        );
        assert_eq!(Vector3::new(0., 0., 1.), tri.normal);

        // CW defined triangle the normal should point in the -Z axis
        let tri = Triangle::new(
            &Point3::new(1., 0., 0.),
            &Point3::new(0., 0., 0.),
            &Point3::new(0., 1., 0.),
            &material,
        );
        assert_eq!(Vector3::new(0., 0., -1.), tri.normal);
    }
}
