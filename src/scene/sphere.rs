use crate::math::{Matrix, Point3, Ray, Vector3};

use super::Color;
use super::Intersection;
use super::Material;
use super::Renderable;

pub struct Sphere {
    transform: Matrix,
    inv_transform: Matrix,
    color: Color,
    material: Material,
}

impl Sphere {
    pub fn new(color: Color, reflectivity: f32, refraction_idx: f32) -> Sphere {
        Sphere {
            transform: Matrix::identity(),
            inv_transform: Matrix::identity(),
            color: color,
            material: Material::new(&color, reflectivity, refraction_idx),
        }
    }
}

impl Renderable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        // apply transformation to the ray
        //let transformed_ray = ray.mat_mul(&self.inv_transform);
        let transformed_ray = self.inv_transform * ray;

        let l = transformed_ray.origin() - Point3::new(0., 0., 0.);
        let a = transformed_ray.direction().len2();
        let b = 2. * transformed_ray.direction().dot(&l);
        let c = l.len2() - 1.;
        match solve_quadratic(a, b, c) {
            None => None,
            Some((mut t0, mut t1)) => {
                if t0 > t1 {
                    let tmp = t1;
                    t1 = t0;
                    t0 = tmp;
                }
                if t0 < 0. && t1 < 0. {
                    return None;
                }

                let t = if t0 < 0. { t1 } else { t0 };
                let entering = t0 > 0.;
                let point = t * ray;
                let normal = t * transformed_ray;
                let mut normal = (self.inv_transform.transpose() * Vector3::from(normal)).norm();
                if !entering {normal = -normal;}
                let eye_dir = -ray.direction().norm();
                Some(Intersection {
                    t,
                    material: self.material,
                    point,
                    eye_dir,
                    normal,
                    entering,
                })
            }
        }
    }

    fn set_transform(&mut self, mat: &Matrix) {
        self.transform = *mat;
        self.inv_transform = self.transform.inverse();
    }

    fn set_color(&mut self, color: &Color) {
        self.color = *color;
    }
}

fn solve_quadratic(a: f32, b: f32, c: f32) -> Option<(f32, f32)> {
    use std::f32::EPSILON;

    let discr = b * b - 4. * a * c;
    if discr < 0. {
        None
    } else if discr.abs() < EPSILON {
        let x = -0.5 * b / a;
        Some((x, x))
    } else {
        let q = if b > 0. {
            -0.5 * (b + discr.sqrt())
        } else {
            -0.5 * (b - discr.sqrt())
        };
        let x0 = q / a;
        let x1 = c / q;
        Some((x0, x1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::Vector3;

    #[test]
    fn basic() {
        let mut sph = Sphere::new(Color::red(), 1., 0.);

        assert_eq!(
            Matrix::identity(),
            sph.transform,
            "A new sphere should have the identity matrix for its transform"
        );

        sph.set_transform(&Matrix::scale(2., 2., 2.));
        assert_eq!(
            Matrix::scale(2., 2., 2.),
            sph.transform,
            "Sphere did not scale as expected"
        );
    }

    #[test]
    fn intersection_no_transform() {
        let sph = Sphere::new(Color::white(), 1., 0.);
        let ray = Ray::new(&Point3::new(0., 0., 2.), &Vector3::new(0., 0., -1.));
        let intersect = sph.intersect(&ray);
        assert_ne!(None, intersect);
        assert_eq!(1., intersect.unwrap().t);

        let miss = Ray::new(&Point3::new(0., 0., 2.), &Vector3::new(0., 1., 0.));
        let intersect = sph.intersect(&miss);
        assert_eq!(None, intersect);

        let edge = Ray::new(&Point3::new(0., 1., 2.), &Vector3::new(0., 0., -1.));
        let intersect = sph.intersect(&edge);
        assert_ne!(None, intersect);
        assert_eq!(2., intersect.unwrap().t);
    }

    #[test]
    fn intersection_transform() {
        let mut sph = Sphere::new(Color::white(), 1., 0.);
        let transform = Matrix::translate(0., 2., -2.) * Matrix::scale(2., 2., 2.);
        sph.set_transform(&transform);

        let ray = Ray::new(&Point3::new(0., 0., 2.), &Vector3::new(0., 0., -1.));
        let intersect = sph.intersect(&ray);
        assert_ne!(None, intersect);
        assert_eq!(4., intersect.unwrap().t);

        let miss = Ray::new(&Point3::new(0., 0., 2.), &Vector3::new(0., 1., 0.));
        let intersect = sph.intersect(&miss);
        assert_eq!(None, intersect);

        let edge = Ray::new(&Point3::new(0., 2., 2.), &Vector3::new(0., 0., -1.));
        let intersect = sph.intersect(&edge);
        assert_ne!(None, intersect);
        assert_eq!(2., intersect.unwrap().t);
    }
}

#[cfg(test)]
mod benchmarks {
    extern crate test;
    use super::*;
    use crate::math::Vector3;

    #[bench]
    fn intersection(b: &mut test::Bencher) {
        let sph = Sphere::new(Color::white(), 1., 0.);
        let ray = Ray::new(&Point3::new(0., 0., 2.), &Vector3::new(0., 0., -1.));

        b.iter(|| sph.intersect(&ray));
    }

    #[bench]
    fn solve_quadratic(bch: &mut test::Bencher) {
        let a = 1.;
        let b = 4.;
        let c = 1.;
        bch.iter(|| super::solve_quadratic(a, b, c));
    }
}
