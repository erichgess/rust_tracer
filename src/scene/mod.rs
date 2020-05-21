use crate::math::{Ray, Vector3, Point3, Matrix};

mod sphere;

#[derive(Debug, PartialEq)]
pub struct Intersection {
    t: f32,
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
}
