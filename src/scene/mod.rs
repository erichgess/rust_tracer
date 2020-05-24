use crate::math::{Matrix, Point3, Ray, Vector3};

mod color;
mod sphere;

pub use color::Color;
pub use sphere::Sphere;

pub struct Scene {
    ambient: Color,
    lights: Vec<Box<dyn LightSource>>,
    shapes: Vec<Box<dyn Renderable>>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            ambient: Color::black(),
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

    pub fn set_ambient(&mut self, ambient: &Color) {
        self.ambient = *ambient;
    }

    pub fn get_ambient(&self) -> &Color {
        &self.ambient
    }

    pub fn lights(&self) -> &Vec<Box<dyn LightSource>> {
        &self.lights
    }
}

fn lambert(light_dir: &Vector3, normal: &Vector3, color: &Color) -> Color {
    light_dir.dot(normal) * color
}

fn phong(
    power: f32,
    eye_dir: &Vector3,
    light_dir: &Vector3,
    normal: &Vector3,
    color: &Color,
) -> Color {
    let h = (eye_dir.norm() + light_dir.norm()).norm();
    let m_dot_h = normal.dot(&h);

    if m_dot_h < 0. {
        0. * color
    } else {
        m_dot_h.powf(power) * color
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
    pub material: Material,
    pub point: Point3,
    pub eye_dir: Vector3,
    pub normal: Vector3,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Material {
    pub color: Color,
    pub specular_intensity: Color,
    pub reflectivity: f32,
    pub refraction_index: f32,
}

impl Material {
    pub fn new(color: &Color, reflectivity: f32, refraction_index: f32) -> Material {
        Material {
            color: *color,
            specular_intensity: Color::white(),
            reflectivity,
            refraction_index,
        }
    }

    pub fn get_reflected_energy(
        &self,
        eye_dir: &Vector3,
        light_dir: &Vector3,
        normal: &Vector3,
        incoming: &Color,
    ) -> Color {
        let diffuse = lambert(&light_dir, &normal, &incoming) * self.color;
        let specular = phong(600., &eye_dir, &light_dir, &normal, &incoming) * self.specular_intensity;
        (1. - self.reflectivity) * diffuse + self.reflectivity * specular
    }
}

pub trait LightSource {
    fn get_energy(
        &self,
        scene: &Scene,
        point: &Point3,
    ) -> (Vector3, Color);
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
    fn get_energy(
        &self,
        scene: &Scene,
        point: &Point3,
    ) -> (Vector3, Color) {
        let dir_to_light = (self.pos - point).norm();
        let ray = Ray::new(&point, &dir_to_light);
        let total_energy = match scene.intersect(&ray) {
            Some(_) => Color::black(),
            None => self.color,
        };
        (dir_to_light, total_energy)
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
}
