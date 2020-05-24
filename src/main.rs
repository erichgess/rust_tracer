#![feature(test)]

mod bmp;
mod math;
mod scene;

use math::{Matrix, Point3, Ray};
use scene::Sphere;
use scene::{Color, Intersection, LightSource, PointLight, Renderable, Scene};

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
            buf: vec![vec![Color::black(); h]; w],
        }
    }
}

fn main() {
    let x_res = 512;
    let y_res = 512;
    let camera = Camera::new(x_res, y_res);
    let mut buffer = RenderBuffer::new(x_res, y_res);

    let mut scene = Scene::new();
    let mut sph = Sphere::new(Color::red(), 0.5);
    let transform = //Matrix::rotate_y(45.)
        Matrix::translate(-1.0, 0., 0.)
        * Matrix::rotate_z(75.)
        * Matrix::scale(1.0, 0.25, 1.0);
    sph.set_transform(&transform);
    scene.add_shape(Box::new(sph));

    let mut sph2 = Sphere::new(Color::blue(), 0.8);
    let transform = Matrix::translate(1., 0., 0.);
    sph2.set_transform(&transform);
    scene.add_shape(Box::new(sph2));

    let mut sph3 = Sphere::new(Color::green(), 0.2);
    let transform = Matrix::translate(0., -2., 0.) * Matrix::scale(10., 1., 10.);
    sph3.set_transform(&transform);
    scene.add_shape(Box::new(sph3));

    let mut sph4 = Sphere::new(0.7 * Color::white(), 1.);
    let transform = Matrix::translate(0., -0.5, -3.) * Matrix::scale(0.3, 0.3, 0.3);
    sph4.set_transform(&transform);
    scene.add_shape(Box::new(sph4));

    let light = PointLight::new(Point3::new(4., 4.0, 0.), Color::new(1., 1., 1.));
    scene.add_light(Box::new(light));

    let light = PointLight::new(Point3::new(-1., 2.0, -4.), Color::new(0., 1., 1.));
    scene.add_light(Box::new(light));

    //let ambient = AmbientLight::new(&Color::new(0.2, 0.2, 0.2));
    //scene.add_light(Box::new(ambient));

    let start = std::time::Instant::now();
    render(&camera, &scene, &mut buffer);
    let duration = start.elapsed();

    //terminal::draw(&buffer);
    bmp::save_to_bmp("test.png", &buffer);
    println!("Render and draw time: {}ms", duration.as_millis());
}

fn render(camera: &Camera, scene: &Scene, buffer: &mut RenderBuffer) {
    for v in 0..camera.y_res {
        for u in 0..camera.x_res {
            let ray = camera.get_ray(u, v);
            buffer.buf[u][v] = trace_ray(scene, &ray, 4);
        }
    }
}

fn trace_ray(scene: &Scene, ray: &Ray, reflections: usize) -> Color {
    let hit = scene.intersect(&ray);
    let diffuse = match hit {
        None => Color::black(),
        Some(i) => {
            calculate_light_illumination(scene, scene.lights(), &i)
        }
    };

    let reflected = if reflections > 0 {
        match hit {
            None => Color::black(),
            Some(i) => {
                // compute reflection vector
                let reflected_dir = -ray.direction().reflect(&i.normal).norm();
                let p = i.point + 0.0002 * i.normal;
                let reflect_ray = Ray::new(&p, &reflected_dir);
                // compute incoming energy from the direction of the reflected ray
                //i.material.reflectivity
                let energy = trace_ray(scene, &reflect_ray, reflections - 1);
                i.material.reflectivity
                    * i.material.get_reflected_energy(&i.eye_dir, &reflected_dir, &i.normal, &energy)
            }
        }
    } else {
        Color::black()
    };

    0.4 * diffuse + reflected
}

fn calculate_light_illumination(
    scene: &Scene,
    lights: &Vec<Box<dyn LightSource>>,
    intersection: &Intersection,
) -> Color {
    // Move slightly away from the surface of intersection because rounding
    // errors in floating point arithmetic can easily cause the ray to intersect
    // with its surface.  This would cause random points to be colored as if
    // they are in shadow even though they are visible to the light source.
    let p = intersection.point + 0.0002 * intersection.normal;
    lights
        .iter()
        .map(|l| l.get_energy(scene, &p))
        .map(|(ldir, lenergy)| {
            intersection.material.get_reflected_energy(
                &intersection.eye_dir,
                &ldir,
                &intersection.normal,
                &lenergy,
            )
        })
        .sum()
}

struct Camera {
    origin: Point3,
    x_min: f32,
    x_max: f32,
    y_min: f32,
    y_max: f32,
    x_res: usize,
    y_res: usize,
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

mod terminal {
    extern crate termion;

    use super::scene::Color;
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
                    c if c == Color::black() => print!("{}.", color::Fg(color::White)),
                    c => {
                        print!("{}x", color::Fg(to_rgb(&c)));
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

    use super::*;

    #[bench]
    fn render_128x128(b: &mut Bencher) {
        let x_res = 128;
        let y_res = 128;
        let camera = Camera::new(x_res, y_res);
        let mut buffer = RenderBuffer::new(x_res, y_res);

        let mut scene = Scene::new();
        let mut sph = Sphere::new(Color::red(), 1.);
        let transform = Matrix::scale(1.0, 2.25, 1.0);
        sph.set_transform(&transform);

        scene.add_shape(Box::new(sph));

        b.iter(|| super::render(&camera, &scene, &mut buffer));
    }
}
