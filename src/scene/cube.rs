/// Render a unit cube
use crate::math::{Matrix, Point3, Ray, Vector3};

use super::ColorFun;
use super::Intersection;
use super::Phong;
use super::Renderable;
use super::TextureCoords;

struct Cube {
    transform: Matrix,
    inv_transform: Matrix,
    material: Phong,
}

impl Cube {
    pub fn new(
        ambient: ColorFun,
        diffuse: ColorFun,
        specular: ColorFun,
        power: f32,
        reflectivity: f32,
        refraction_idx: f32,
    ) -> Cube {
        Cube {
            transform: Matrix::identity(),
            inv_transform: Matrix::identity(),
            material: Phong::new(
                ambient,
                diffuse,
                specular,
                power,
                reflectivity,
                refraction_idx,
            ),
        }
    }
}

impl Renderable for Cube {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        // apply transformation to the ray
        let transformed_ray = self.inv_transform * ray;

        let mut normal = Vector3::new(0., 1., 0.);
        let mut tmin = (-0.5 - ray.origin().x())/ ray.direction().x();
        let mut tmax = (0.5 - ray.origin().x())/ ray.direction().x();

        if tmin > tmax {
            let tmp = tmin;
            tmin = tmax;
            tmax = tmin;
        }

        if tmin < 0. {
             normal = -normal;
        }

        let mut tymin = (-0.5 - ray.origin().y()) / ray.direction().y();
        let mut tymax = (0.5 - ray.origin().y()) / ray.direction().y();
        if tymin > tymax {
            let tmp = tymin;
            tymin = tymax;
            tymax = tmp;
        }

        if tmin > tymax || tymin > tmax {
            return None;
        }

        if tymin > tmin {
            tmin = tymin;
        }

        if tymax < tmax {
            tmax = tymax;
        }

        let mut tzmin = (-0.5 - ray.origin().z()) / ray.direction().z();
        let mut tzmax = (0.5 - ray.origin().z()) / ray.direction().z();
        if tzmin > tzmax {
            let tmp = tzmin;
            tzmin = tzmax;
            tzmax = tmp;
        }

        if (tmin > tzmax) || (tzmin > tmax) {
            return None;
        }

        if tzmin > tmin {
            tmin = tzmin;
        }

        if tzmax < tmax {
            tmax = tzmax;
        }

        None
    }

    fn set_transform(&mut self, mat: &Matrix) {
        self.transform = *mat;
        self.inv_transform = self.transform.inverse();
    }

}
