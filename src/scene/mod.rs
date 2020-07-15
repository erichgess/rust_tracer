use crate::math::{Matrix, Point3, Ray, Vector3};

mod color;
mod cube;
mod intersection;
mod material;
mod plane;
mod sphere;
mod triangle;

use std::cell::*;

pub use color::colors;
pub use color::Color;
pub use cube::Cube;
pub use intersection::Intersection;
pub use material::{ColorFun, Material, Phong, TexturePhong};
pub use plane::Plane;
pub use sphere::Sphere;
pub use triangle::Triangle;

pub struct Scene {
    id: i32,
    ambient: Color,
    lights: Vec<Box<dyn LightSource>>,
    shapes: Vec<Box<dyn Renderable>>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            id: 0,
            ambient: colors::BLACK,
            lights: vec![],
            shapes: vec![],
        }
    }

    /// Adds `shape` to the scene so that it will be rendered.
    pub fn add_shape(&mut self, shape: Box<dyn Renderable>) {
        let mut shape = shape;
        shape.set_id(self.shapes.len() as i32);
        self.shapes.push(shape);
    }

    pub fn add_light(&mut self, light: Box<dyn LightSource>) {
        self.lights.push(light);
    }

    pub fn set_ambient(&mut self, ambient: &Color) {
        self.ambient = *ambient;
    }

    pub fn ambient(&self) -> &Color {
        &self.ambient
    }

    pub fn lights(&self) -> &Vec<Box<dyn LightSource>> {
        &self.lights
    }

    pub fn shapes(&self) -> &Vec<Box<dyn Renderable>> {
        &self.shapes
    }

    pub fn find_shape_mut(&mut self, name: &str) -> Option<&mut dyn Renderable> {
        for i in 0..self.shapes.len() {
            if self.shapes[i].get_name() == name {
                return Some(&mut (*self.shapes[i]))
            }
        }

        None
    }

    pub fn find_shape(&self, name: &str) -> Option<&dyn Renderable> {
        for i in 0..self.shapes.len() {
            if self.shapes[i].get_name() == name {
                return Some(&(*self.shapes[i]))
            }
        }

        None
    }
}

impl Renderable for Scene {
    fn id(&self) -> i32 {
        self.id
    }

    fn set_id(&mut self ,id: i32) {
        self.id = id;
    }

    fn set_transform(&mut self, _: &Matrix) {}

    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let mut nearest = 0.;
        let mut nearest_intersection = None;
        for shape in self.shapes.iter() {
            match shape.intersect(ray) {
                None => (),
                Some(intersection) => {
                    if nearest_intersection.is_none() {
                        nearest = intersection.t;
                        nearest_intersection = Some(intersection);
                    } else if intersection.t < nearest {
                        nearest = intersection.t;
                        nearest_intersection = Some(intersection);
                    }
                }
            }
        }
        nearest_intersection
    }

    fn get_name(&self) -> String {
        self.to_string()
    }

    fn size(&self) -> usize {
        self.shapes.iter().map(|s| s.size()).sum()
    }

    fn get_material_mut(&mut self) -> Option<RefMut<dyn Material>> {
        None
    }

    fn get_material(&self) -> Option<Ref<dyn Material>> {
        None
    }

    fn to_string(&self) -> String {
        "The Scene".into()
    }
}

/**
 * A `Renderable` is anything which exists as an actual entity on the scene
 * that will be rendered in the final image.  For example: a sphere or a
 * cube.
 *
 * This trait defines a set of methods which every object must implement
 * and which are required for the object to be rendered.
 */
pub trait Renderable {
    fn id(&self) -> i32;
    fn set_id(&mut self, id: i32);

    // Tests if a ray will intersect the object and if it does
    // returns where the intersection occurred.
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;

    // Set the transformation matrix which will be used to position
    // and scale the sphere within the scene
    fn set_transform(&mut self, mat: &Matrix);

    fn get_material_mut(&mut self) -> Option<RefMut<dyn Material>>;
    fn get_material(&self) -> Option<Ref<dyn Material>>;

    fn get_name(&self) -> String;
    fn to_string(&self) -> String;
    fn size(&self) -> usize;
}

pub type TextureCoords = (f32, f32);

pub trait LightSource {
    fn get_energy(&self, scene: &Scene, point: &Point3) -> (Vector3, Color);
    fn to_string(&self) -> String;
}

/**
A single point light in the scene: it radiates energy with the
intensity of `Color` equally in all directions.
*/
pub struct PointLight {
    pos: Point3,
    color: Color,
}

impl PointLight {
    pub fn new(pos: Point3, color: Color) -> PointLight {
        PointLight { pos, color }
    }
}

impl LightSource for PointLight {
    fn get_energy(&self, scene: &Scene, point: &Point3) -> (Vector3, Color) {
        let dir_to_light = (self.pos - point).norm();
        let ray = Ray::new(&point, &dir_to_light);
        let total_energy = match scene.intersect(&ray) {
            // If there is an intersection: make sure it happens between the light and the
            // surface point.
            Some(i) => {
                if (i.point - point).len2() < (self.pos - point).len2() {
                    colors::BLACK
                } else {
                    self.color
                }
            }
            None => self.color,
        };
        (dir_to_light, total_energy)
    }

    fn to_string(&self) -> String {
        format!(
            "Position: ({}, {}, {}), Color: ({}, {}, {})",
            self.pos.x(),
            self.pos.y(),
            self.pos.z(),
            self.color.r,
            self.color.g,
            self.color.b
        )
    }
}

/// Ambient light that radiates all points in a scene with a constant
/// amount of energy.
#[derive(Copy, Clone)]
pub struct AmbientLight {
    color: Color,
}

impl AmbientLight {
    #[allow(dead_code)]
    pub fn new(c: &Color) -> AmbientLight {
        AmbientLight { color: *c }
    }
}

impl LightSource for AmbientLight {
    fn get_energy(&self, _: &Scene, _: &Point3) -> (Vector3, Color) {
        (Vector3::new(0., 0., 0.), self.color)
    }

    fn to_string(&self) -> String {
        format!(
            "Color: ({}, {}, {})",
            self.color.r, self.color.g, self.color.b
        )
    }
}
