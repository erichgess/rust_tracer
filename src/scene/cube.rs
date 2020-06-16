/// Render a unit cube
use std::rc::Rc;

use crate::math::{Matrix, Point3, Ray};

use super::Color;
use super::Intersection;
use super::Phong;
use super::Renderable;
use super::Triangle;
use super::Scene;

pub struct Cube {
    triangles: Scene,
    transform: Matrix,
    inv_transform: Matrix,
}

impl Cube {
    pub fn new(
        ambient: Color,
        diffuse: Color,
        specular: Color,
        power: f32,
        reflectivity: f32,
        refraction_idx: f32,
    ) -> Cube {
        let material = Rc::new(Phong::new(
            ambient,
            diffuse,
            specular,
            power,
            reflectivity,
            refraction_idx,
        ));
        let v0 = Point3::new(0.5, 0.5, -0.5);
        let v1 = Point3::new(0.5, -0.5, -0.5);
        let v2 = Point3::new(-0.5, -0.5, -0.5);
        let v3 = Point3::new(-0.5, 0.5, -0.5);

        let v4 = Point3::new(0.5, 0.5, 0.5);
        let v5 = Point3::new(-0.5, 0.5, 0.5);
        let v6 = Point3::new(-0.5, -0.5, 0.5);
        let v7 = Point3::new(0.5, -0.5, 0.5);

        // front
        let tf1 = Triangle::new(&v1, &v2, &v3, material.clone());
        let tf2 = Triangle::new(&v0, &v1, &v3, material.clone());

        // back
        let tk1 = Triangle::new(&v7, &v5, &v4, material.clone());
        let tk2 = Triangle::new(&v5, &v7, &v6, material.clone());

        // right side
        let tr1 = Triangle::new(&v0, &v4, &v7, material.clone());
        let tr2 = Triangle::new(&v7, &v1, &v0, material.clone());

        // left side
        let tl1 = Triangle::new(&v5, &v3, &v6, material.clone());
        let tl2 = Triangle::new(&v6, &v3, &v2, material.clone());

        // bottom
        let tb1 = Triangle::new(&v1, &v7, &v6, material.clone());
        let tb2 = Triangle::new(&v6, &v2, &v1, material.clone());

        // top
        let tt1 = Triangle::new(&v5, &v4, &v0, material.clone());
        let tt2 = Triangle::new(&v0, &v3, &v5, material.clone());

        //let tris = vec![tf1, tf2, tk1, tk2, tb1, tb2, tr1, tr2, tl1, tl2, tt1, tt2];
        let mut scene = Scene::new();
        scene.add_shape(Box::new(tf1));
        scene.add_shape(Box::new(tf2));
        scene.add_shape(Box::new(tk1));
        scene.add_shape(Box::new(tk2));
        scene.add_shape(Box::new(tr1));
        scene.add_shape(Box::new(tr2));
        scene.add_shape(Box::new(tl1));
        scene.add_shape(Box::new(tl2));
        scene.add_shape(Box::new(tt1));
        scene.add_shape(Box::new(tt2));
        scene.add_shape(Box::new(tb1));
        scene.add_shape(Box::new(tb2));


        Cube {
            transform: Matrix::identity(),
            inv_transform: Matrix::identity(),
            triangles: scene,
        }
    }
}

impl Renderable for Cube {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        // apply transformation to the ray
        let transformed_ray = self.inv_transform * ray;

        match self.triangles.intersect(&transformed_ray) {
            None => None,
            Some(mut i) => {
                i.point = i.t * ray;
                i.eye_dir = -(ray.direction().norm());
                i.normal = (self.inv_transform.transpose() * i.normal).norm(); // TODO: am I doing the right matrix op?
                Some(i)
            },
        }
    }

    fn set_transform(&mut self, mat: &Matrix) {
        self.transform = *mat;
        self.inv_transform = self.transform.inverse();
    }

    fn to_string(&self) -> String {
        "Cube".into()
    }
}
