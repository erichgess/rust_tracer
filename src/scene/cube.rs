/// Render a unit cube
use std::rc::Rc;

use crate::math::{Matrix, Point3, Ray};

use super::Intersection;
use super::Material;
use super::Renderable;
use super::Scene;
use super::Triangle;

pub struct Cube {
    triangles: Scene,
    transform: Matrix,
    inv_transform: Matrix,
}

impl Cube {
    pub fn new(material: Rc<dyn Material>) -> Cube {
        let v0 = Point3::new(0.5, 0.5, -0.5);
        let v1 = Point3::new(0.5, -0.5, -0.5);
        let v2 = Point3::new(-0.5, -0.5, -0.5);
        let v3 = Point3::new(-0.5, 0.5, -0.5);

        let v4 = Point3::new(0.5, 0.5, 0.5);
        let v5 = Point3::new(-0.5, 0.5, 0.5);
        let v6 = Point3::new(-0.5, -0.5, 0.5);
        let v7 = Point3::new(0.5, -0.5, 0.5);

        // front
        let tf1 = Triangle::new(&v1, &v2, &v3, Rc::clone(&material));
        let tf2 = Triangle::new(&v0, &v1, &v3, Rc::clone(&material));

        // back
        let tk1 = Triangle::new(&v7, &v5, &v4, Rc::clone(&material));
        let tk2 = Triangle::new(&v5, &v7, &v6, Rc::clone(&material));

        // right side
        let tr1 = Triangle::new(&v0, &v4, &v7, Rc::clone(&material));
        let tr2 = Triangle::new(&v7, &v1, &v0, Rc::clone(&material));

        // left side
        let tl1 = Triangle::new(&v5, &v3, &v6, Rc::clone(&material));
        let tl2 = Triangle::new(&v6, &v3, &v2, Rc::clone(&material));

        // bottom
        let tb1 = Triangle::new(&v1, &v7, &v6, Rc::clone(&material));
        let tb2 = Triangle::new(&v6, &v2, &v1, Rc::clone(&material));

        // top
        let tt1 = Triangle::new(&v5, &v4, &v0, Rc::clone(&material));
        let tt2 = Triangle::new(&v0, &v3, &v5, Rc::clone(&material));

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
            }
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
