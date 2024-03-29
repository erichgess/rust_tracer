use super::math::{Point3, Ray, Vector3};
use super::scene::colors::*;
use super::scene::{Color, Intersection, Renderable, Scene};

pub struct RenderBuffer {
    pub w: usize,
    pub h: usize,
    pub buf: Vec<Vec<Color>>,
}

impl RenderBuffer {
    pub fn new(w: usize, h: usize) -> RenderBuffer {
        RenderBuffer {
            w,
            h,
            buf: vec![vec![BLACK; h]; w],
        }
    }
}

#[cfg(target_os = "linux")]
pub fn draw_to_terminal(scene: &Scene) {
    let x_res = 100;
    let y_res = 50;
    let camera = Camera::new(x_res, y_res);
    let mut buffer = RenderBuffer::new(x_res, y_res);
    render(&camera, scene, &mut buffer, 5);
    terminal::draw(&buffer);
}

pub fn render(camera: &Camera, scene: &Scene, buffer: &mut RenderBuffer, depth: usize) {
    for v in 0..camera.y_res {
        for u in 0..camera.x_res {
            let ray = camera.get_ray(u, v);
            buffer.buf[u][v] = trace_ray(scene, &ray, depth);
        }
    }
}

fn trace_ray(scene: &Scene, ray: &Ray, depth: usize) -> Color {
    use std::f32::EPSILON;

    if depth == 0 {
        return BLACK;
    }

    let hit = scene.intersect(&ray);
    match hit {
        None => BLACK,
        Some(i) => {
            let (n1, n2) = if i.entering {
                (1., i.material.borrow().refraction_index())
            } else {
                (i.material.borrow().refraction_index(), 1.)
            };

            let ambient = (i.material.borrow().ambient(i.tex_coord)) * scene.ambient();

            let lights: Color = get_light_energy(scene, &i)
                .iter()
                .map(|(ldir, lenergy)| {
                    let fresnel = fresnel_reflection(&ldir, &i.normal, n1, n2);
                    fresnel
                        * i.material
                            .borrow()
                            .get_reflected_energy(&lenergy, &ldir, &i)
                })
                .sum();

            let reflected = if i.material.borrow().reflectivity() > EPSILON {
                // compute reflection vector
                let reflect_ray = reflect_ray(ray, &i);
                // compute incoming energy from the direction of the reflected ray
                let energy = trace_ray(scene, &reflect_ray, depth - 1);
                let fresnel = fresnel_reflection(&reflect_ray.direction(), &i.normal, n1, n2);
                fresnel
                    * i.material.borrow().get_reflected_energy(
                        &energy,
                        &reflect_ray.direction(),
                        &i,
                    )
            } else {
                BLACK
            };

            let refracted = if i.material.borrow().refraction_index() > EPSILON {
                let refract_ray = refract_ray(ray, &i, n1, n2);
                (i.material.borrow().diffuse(i.tex_coord))
                    * refract_ray
                        .map(|r| {
                            let fresnel =
                                fresnel_refraction(&r.direction(), &i.normal.neg(), n1, n2);
                            fresnel * trace_ray(scene, &r, depth - 1)
                        })
                        .unwrap_or(BLACK)
            } else {
                BLACK
            };

            ambient + lights + reflected + refracted
        }
    }
}

pub fn reflect_ray(ray: &Ray, i: &Intersection) -> Ray {
    // compute reflection vector
    let reflected_dir = -ray.direction().reflect(&i.normal).norm();
    let p = i.point + 0.0002 * reflected_dir;
    Ray::new(&p, &reflected_dir)
}

pub fn refract_ray(ray: &Ray, i: &Intersection, n1: f32, n2: f32) -> Option<Ray> {
    let ratio = n1 / n2;
    let m_dot_r = -ray.direction().dot(&i.normal);
    let cos_theta_sqrd = 1. - ratio * ratio * (1. - m_dot_r * m_dot_r);

    if cos_theta_sqrd > 0. {
        let cos_theta = cos_theta_sqrd.sqrt();
        let refract_dir = ray.direction() * ratio + i.normal * (ratio * m_dot_r - cos_theta);
        let p = i.point + 0.0002 * refract_dir;
        Some(Ray::new(&p, &refract_dir))
    } else {
        None
    }
}

/// Use Schlick's approximation to compute the Fresnel coeffection for the amount of energy
/// reflected off of a surface.
pub fn fresnel_reflection(light_dir: &Vector3, normal: &Vector3, n1: f32, n2: f32) -> f32 {
    let m_dot_r = light_dir.dot(&normal);
    let r0 = ((n1 - n2) / (n1 + n2)) * ((n1 - n2) / (n1 + n2));

    r0 + (1. - r0) * (1. - m_dot_r).powi(5)
}

/// Use Schlick's approximation to compute the amount of energy transmitted through a material
/// (this is the energy which is not reflected)
pub fn fresnel_refraction(light_dir: &Vector3, normal: &Vector3, n1: f32, n2: f32) -> f32 {
    1. - fresnel_reflection(light_dir, normal, n1, n2)
}

pub fn get_light_energy(scene: &Scene, i: &Intersection) -> Vec<(Vector3, Color)> {
    // Move slightly away from the surface of intersection because rounding
    // errors in floating point arithmetic can easily cause the ray to intersect
    // with its surface.  This would cause random points to be colored as if
    // they are in shadow even though they are visible to the light source.
    let p = i.point + 0.0002 * i.normal;
    scene
        .lights()
        .iter()
        .map(|l| l.get_energy(scene, &p))
        .collect()
}

pub struct Camera {
    pub origin: Point3,
    pub x_min: f32,
    pub x_max: f32,
    pub y_min: f32,
    pub y_max: f32,
    pub x_res: usize,
    pub y_res: usize,
}

impl Camera {
    pub fn new(x_res: usize, y_res: usize) -> Camera {
        Camera {
            origin: Point3::new(0., 0., -8.),
            x_min: -3.,
            x_max: 3.,
            y_min: -3.,
            y_max: 3.,
            x_res,
            y_res,
        }
    }

    pub fn get_ray(&self, u: usize, v: usize) -> Ray {
        let x_delta = (self.x_max - self.x_min) / self.x_res as f32;
        let y_delta = (self.y_max - self.y_min) / self.y_res as f32;
        let x = self.x_min as f32 + u as f32 * x_delta;
        let y = self.y_max as f32 - v as f32 * y_delta;
        let viewpoint = Point3::new(x, y, 0.);
        Ray::new(&self.origin, &(viewpoint - self.origin).norm())
    }
}

#[cfg(target_os = "linux")]
mod terminal {
    extern crate termion;

    use super::super::scene::{colors::*, Color};
    use super::RenderBuffer;
    use termion::{color, color::Rgb};

    #[allow(dead_code)]
    fn to_rgb(c: &Color) -> Rgb {
        let r = 255. * c.r;
        let g = 255. * c.g;
        let b = 255. * c.b;

        Rgb(r as u8, g as u8, b as u8)
    }

    #[allow(dead_code)]
    pub fn draw(buffer: &RenderBuffer) {
        for v in 0..buffer.h {
            for u in 0..buffer.w {
                match buffer.buf[u][v] {
                    c if c == BLACK => print!("{}.", color::Fg(color::White)),
                    c => {
                        print!("{}X", color::Fg(to_rgb(&c)));
                    }
                }
            }
            println!();
        }
    }
}

#[cfg(test)]
mod benchmarks {
    extern crate test;
    use test::Bencher;

    use std::cell::RefCell;
    use std::rc::Rc;

    use super::super::math::Matrix;
    use super::super::scene::{Phong, Sphere};
    use super::*;

    #[bench]
    fn render_128x128(b: &mut Bencher) {
        let x_res = 128;
        let y_res = 128;
        let camera = Camera::new(x_res, y_res);
        let mut buffer = RenderBuffer::new(x_res, y_res);

        let mut scene = Scene::new();
        let phong = Rc::new(RefCell::new(Phong::new(WHITE, RED, WHITE, 60., 1., 0.)));
        let mut sph = Sphere::new(phong);
        let transform = Matrix::scale(1.0, 2.25, 1.0);
        sph.set_transform(&transform);

        scene.add_shape(Box::new(sph));

        b.iter(|| super::render(&camera, &scene, &mut buffer, 5));
    }
}
