#![feature(test)]

mod bmp;
mod math;
mod scene;

use math::{Matrix, Point3, Ray};
use scene::Sphere;
use scene::{AmbientLight, Color, PointLight, Renderable, Scene};

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
    let mut sph = Sphere::new();
    sph.set_color(&Color::red());
    let transform = //Matrix::rotate_y(45.)
        Matrix::translate(-1.0, 0., 0.)
        * Matrix::rotate_z(75.)
        * Matrix::scale(1.0, 0.25, 1.0);
    sph.set_transform(&transform);
    scene.add_shape(Box::new(sph));

    let mut sph2 = Sphere::new();
    sph2.set_color(&Color::blue());
    let transform = Matrix::translate(1., 0., 0.);
    sph2.set_transform(&transform);
    scene.add_shape(Box::new(sph2));

    let mut sph3 = Sphere::new();
    sph3.set_color(&(0.8 * Color::green()));
    let transform = Matrix::translate(0., -2., 0.) * Matrix::scale(10., 1., 10.);
    sph3.set_transform(&transform);
    scene.add_shape(Box::new(sph3));

    let light = PointLight::new(Point3::new(1., 4.0, 0.), Color::new(1., 1., 1.));
    scene.add_light(Box::new(light));

    let ambient = AmbientLight::new(&Color::new(0.2, 0.2, 0.2));
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
            buffer.buf[u][v] = get_energy(scene, &ray, 4);
        }
    }
}

fn get_energy(scene: &Scene, ray: &Ray, reflections: usize) -> Color {
    let hit = scene.intersect(&ray);
    let diffuse = match hit {
        None => Color::black(),
        Some(i) => {
            scene.get_incoming_energy(&i)
            //let color = energy * i.material.color;
        }
    };

    let reflected = if reflections > 0 {
        match hit {
            None => Color::black(),
            Some(i) => {
                // compute reflection vector
                let reflected_dir = -ray.direction().reflect(&i.normal);
                let p = i.point + 0.0002 * i.normal;
                let reflect_ray = Ray::new(&p, &reflected_dir);
                // compute incoming energy from the direction of the reflected ray
                i.material.reflectivity * (Color::white() - i.material.color) * get_energy(scene, &reflect_ray, reflections - 1)
            }
        }
    } else {
        Color::black()
    };

    0.4 * diffuse + reflected
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

    fn to_rgb(c: &Color) -> Rgb {
        let r = 255. * c.r;
        let g = 255. * c.g;
        let b = 255. * c.b;

        Rgb(r as u8, g as u8, b as u8)
    }

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
        let mut sph = Sphere::new();
        sph.set_color(&Color::red());
        let transform = Matrix::scale(1.0, 2.25, 1.0);
        sph.set_transform(&transform);

        scene.add_shape(Box::new(sph));

        b.iter(|| super::render(&camera, &scene, &mut buffer));
    }
}
