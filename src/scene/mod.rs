use crate::math::{Point3, Ray, Matrix, Vector3};

mod sphere;

pub use sphere::Sphere;

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
    pub normal: Vector3
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color {r, g, b}
    }

    pub fn red() -> Color {
        Color {
            r: 1.,
            g: 0.,
            b: 0.,
        }
    }
}

impl std::ops::Add for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl std::ops::Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl std::ops::Mul<Color> for f32 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color {
            r: self * rhs.r,
            g: self * rhs.g,
            b: self * rhs.b,
        }
    }
}
