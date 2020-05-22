use crate::math::{Matrix, Point3, Ray, Vector3};

mod color;
mod sphere;

pub use color::Color;
pub use sphere::Sphere;

pub struct Scene {
    lights: Vec<Box<dyn LightSource>>,
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

    pub fn add_light(&mut self, light: Box<dyn LightSource>) {
        self.lights.push(light);
    }

    pub fn get_incoming_energy(&self, intersection: &Intersection) -> Color {
        // Move slightly away from the surface of intersection because rounding
        // errors in floating point arithmetic can easily cause the ray to intersect
        // with its surface.  This would cause random points to be colored as if
        // they are in shadow even though they are visible to the light source.
        let p = intersection.point + 0.0002 * intersection.normal;
        let mut total_energy = Color::black();
        for light in self.lights.iter() {
            total_energy += light.get_energy(&self, &p, &intersection.normal);
        }
        total_energy
    }
}

impl Renderable for Scene {
    fn set_transform(&mut self, _: &Matrix) {}

    fn set_color(&mut self, _: &Color) {}

    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let mut nearest_intersection = None;
        for shape in self.shapes.iter() {
            match shape.intersect(ray) {
                None => (),
                Some(intersection) => {
                    if nearest_intersection.is_none() {
                        nearest_intersection = Some(intersection);
                    } else if intersection.t < nearest_intersection.unwrap().t {
                        nearest_intersection = Some(intersection);
                    }
                }
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

pub trait LightSource {
    fn get_energy(&self, scene: &Scene, point: &Point3, normal: &Vector3) -> Color;
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
    fn get_energy(&self, scene: &Scene, point: &Point3, normal: &Vector3) -> Color {
        let dir_to_light = (self.pos - point).norm();
        let ray = Ray::new(&point, &dir_to_light);
        let total_energy = match scene.intersect(&ray) {
            Some(_) => Color::black(),
            None => dir_to_light.dot(normal) * self.color,
        };
        total_energy
    }
}

/// Ambient light that radiates all points in a scene with a constant
/// amount of energy.
pub struct AmbientLight {
    color: Color,
}

impl AmbientLight {
    pub fn new(c: &Color) -> AmbientLight {
        AmbientLight { color: *c }
    }
}

impl LightSource for AmbientLight {
    fn get_energy(&self, _: &Scene, _: &Point3, _: &Vector3) -> Color {
        self.color
    }
}
