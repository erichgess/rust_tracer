use crate::math::{Matrix, Point3, Ray, Vector3};

mod color;
mod sphere;

pub use color::Color;
pub use sphere::Sphere;

pub struct Scene {
    lights: Vec<Light>,
    shapes: Vec<Box<dyn Renderable>>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            lights: vec![],
            shapes: vec![],
        }
    }

    pub fn add_shape(&mut self, shape: Box<dyn Renderable>) {
        self.shapes.push(shape);
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    pub fn shapes(&self) -> &Vec<Box<dyn Renderable>> {
        &self.shapes
    }

    pub fn lights(&self) -> &Vec<Light> {
        &self.lights
    }
}

impl Renderable for Scene {
    fn set_transform(&mut self, mat: &Matrix) {
        
    }

    fn set_color(&mut self, color: &Color) {
        
    }

    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let mut nearest_intersection = None;
        for shape in self.shapes.iter() {
            match shape.intersect(ray){
                None => (),
                Some(intersection) => {
                    if nearest_intersection.is_none() {
                        nearest_intersection = Some(intersection);
                    } else if intersection.t < nearest_intersection.unwrap().t{
                        nearest_intersection = Some(intersection);
                    }
                },
            }
        }
        nearest_intersection
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
    // Tests if a ray will intersect the object and if it does
    // returns where the intersection occurred.
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;

    // Set the transformation matrix which will be used to position
    // and scale the sphere within the scene
    fn set_transform(&mut self, mat: &Matrix);

    // Sets the color of the object
    fn set_color(&mut self, color: &Color);
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Intersection {
    pub t: f32,
    pub color: Color,
    pub point: Point3,
    pub normal: Vector3,
}

/**
 A single point light in the scene: it radiates energy with the
 intensity of `Color` equally in all directions.
 */
pub struct Light {
    pos: Point3,
    color: Color,
}

impl Light {
    pub fn new(pos: Point3, color: Color) -> Light {
        Light { pos, color }
    }
    pub fn pos(&self) -> Point3 {
        self.pos
    }

    pub fn color(&self) -> Color {
        self.color
    }
}
