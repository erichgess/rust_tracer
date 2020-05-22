#![feature(test)]

mod math;
mod scene;

use math::{Matrix, Point3, Ray};
use scene::{Color, Intersection, Renderable};
use scene::Sphere;

fn main() {
    let x_res = 50;
    let y_res = 25;
    let camera = Camera::new(x_res, y_res);
    let mut buffer = vec![vec![None; y_res]; x_res];

    render(&camera, x_res, y_res, &mut buffer);

    terminal::draw(x_res, y_res, &buffer);
}

fn render(camera: &Camera, x_res: usize, y_res: usize, buffer: &mut Vec<Vec<Option<Intersection>>>) {
    let mut sph = Sphere::new();
    sph.set_color(&Color::red());
    let transform = Matrix::scale(1.0, 2.25, 1.0);
    sph.set_transform(&transform);

    for v in 0..y_res {
        for u in 0..x_res {
            let ray = camera.get_ray(u, v);
            let hit = sph.intersect(&ray);
            if hit.is_some() {
                println!("{:?}", hit);
            }
            buffer[u][v] = hit;
        }
    }
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

    use termion::{color, color::Rgb};
    use super::scene::{Color, Intersection};

    fn to_rgb(c: &Color) -> Rgb {
        let r = 255. * c.r;
        let g = 255. * c.g;
        let b = 255. * c.b;

        Rgb(r as u8, g as u8, b as u8)
    }

    pub fn draw(x_res: usize, y_res: usize, buffer: &Vec<Vec<Option<Intersection>>>) {
        for v in 0..y_res {
            for u in 0..x_res {
                match buffer[u][v] {
                    None => print!("{}.", color::Fg(color::White)),
                    Some(i) => {
                        print!("{}x", color::Fg(to_rgb(&i.color)));
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
    fn render(b: &mut Bencher) {
        let x_res = 128;
        let y_res = 128;
        let camera = Camera::new(x_res, y_res);

        let mut buffer = vec![vec![None; y_res]; x_res];

        b.iter(||super::render(&camera, x_res, y_res, &mut buffer));
    }
}
