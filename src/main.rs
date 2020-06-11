#![feature(test)]

mod bmp;
mod math;
mod scene;

use clap::{App, Arg, ArgMatches};

use math::{Matrix, Point3, Ray, Vector3};
use scene::colors::*;
use scene::Sphere;
use scene::{
    Color, Cube, Intersection, Material, Phong, Plane, PointLight, Renderable, Scene, TextureCoords,
};

#[derive(Debug)]
struct Config {
    width: usize,
    height: usize,
    to_terminal: bool,
    depth: usize,
}

fn main() {
    let args = configure_cli().get_matches();
    let config = parse_args(&args);
    println!("{:?}", config);
    let x_res = config.width;
    let y_res = config.height;
    let camera = Camera::new(x_res, y_res);
    let mut buffer = RenderBuffer::new(x_res, y_res);

    let mut scene = Scene::new();
    create_scene(&mut scene);
    let start = std::time::Instant::now();
    render(&camera, &scene, &mut buffer, config.depth);
    let duration = start.elapsed();

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Invalid time");
    bmp::save_to_bmp(
        "./output/",
        &format!("{}.png", timestamp.as_secs()),
        &buffer,
    )
    .expect("Failed to save image to disk");
    println!("Render and draw time: {}ms", duration.as_millis());

    if config.to_terminal {
        draw_to_terminal(&scene);
    }
}

fn configure_cli<'a, 'b>() -> App<'a, 'b> {
    App::new("Rust Tracer")
        .version(option_env!("CARGO_PKG_VERSION").unwrap_or(""))
        .about("Ray Tracer")
        .arg(
            Arg::with_name("width")
                .long("width")
                .short("w")
                .takes_value(true)
                .default_value("512")
                .help("Set the width in pixels of the rendered image")
        )
        .arg(
            Arg::with_name("height")
                .long("height")
                .short("h")
                .takes_value(true)
                .default_value("512")
                .help("Set the height in pixels of the rendered image")
        )
        .arg(
            Arg::with_name("depth")
                .long("depth")
                .short("d")
                .takes_value(true)
                .default_value("8")
                .help("Set the maximum depth of reflections and transmissions which the ray tracer will follow when tracing a ray through the scene.")
        )
        .arg(
            Arg::with_name("to-terminal")
                .long("to-terminal")
                .short("t")
                .help("Render the scene as ASCII art to the terminal")
        )
}

fn parse_args(args: &ArgMatches) -> Config {
    let width = args.value_of("width").map(|s| s.parse::<usize>().expect("Expected integer for width")).unwrap();
    let height = args.value_of("height").map(|s| s.parse::<usize>().expect("Expected integer for height")).unwrap();
    let depth = args.value_of("depth").map(|s| s.parse::<usize>().expect("Expected integer for depth")).unwrap();
    let to_terminal = args.is_present("to-terminal");
    Config{
        width,
        height,
        depth,
        to_terminal,
    }
}

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

fn black(_: TextureCoords) -> Color {
    BLACK
}

fn red(_: TextureCoords) -> Color {
    RED
}

fn blue(_: TextureCoords) -> Color {
    0.8 * BLUE
}

fn dim_blue(_: TextureCoords) -> Color {
    0.1 * BLUE
}

fn white(_: TextureCoords) -> Color {
    WHITE
}

fn bright_gray(_: TextureCoords) -> Color {
    0.8 * WHITE
}

fn dim_white(_: TextureCoords) -> Color {
    0.1 * WHITE
}

fn checkerboard(tx: TextureCoords) -> Color {
    let u = (tx.0).abs() as i32;
    let v = (tx.1).abs() as i32;

    if tx.0 < 0. && tx.1 < 0. || tx.0 > 0. && tx.1 > 0. {
        if u % 2 == v % 2 {
            WHITE
        } else {
            0.5 * WHITE
        }
    } else {
        if u % 2 != v % 2 {
            WHITE
        } else {
            0.5 * WHITE
        }
    }
}

fn create_scene(scene: &mut Scene) {
    let mut sph = Sphere::new(dim_white, red, white, 60., 0.5, 0.);
    let transform =
        Matrix::translate(-1.0, 0., 0.) * Matrix::rotate_z(75.) * Matrix::scale(1.0, 0.25, 1.0);
    sph.set_transform(&transform);
    scene.add_shape(Box::new(sph));

    let mut sph2 = Sphere::new(black, blue, dim_blue, 600., 0.4, 0.);
    let transform = Matrix::translate(1., -1., 0.);
    sph2.set_transform(&transform);
    scene.add_shape(Box::new(sph2));

    let mut sph4 = Sphere::new(black, white, white, 60., 0.7, 1.333);
    let transform = Matrix::translate(0., -0.5, -3.) * Matrix::scale(0.6, 0.6, 0.6);
    sph4.set_transform(&transform);
    scene.add_shape(Box::new(sph4));

    let plane_material = Phong::new(dim_white, checkerboard, dim_white, 60., 0., 0.);
    let plane = Plane::new(
        &Point3::new(0., -2., 2.),
        &Vector3::new(0., 0., -1.),
        &plane_material,
    );
    scene.add_shape(Box::new(plane));

    let plane_material = Phong::new(dim_white, checkerboard, dim_white, 60., 0., 0.);
    let plane = Plane::new(
        &Point3::new(0., -2., 0.),
        &Vector3::new(0., 1., 0.),
        &plane_material,
    );
    scene.add_shape(Box::new(plane));

    let mut cube = Cube::new(black, white, white, 60., 0., 1.333);
    let transform = Matrix::translate(-1., -1.0, -4.) * Matrix::rotate_x(-45.0);
    cube.set_transform(&transform);
    scene.add_shape(Box::new(cube));
    let light = PointLight::new(Point3::new(4., 4.0, 0.), Color::new(1., 0., 0.));
    scene.add_light(Box::new(light));

    let light = PointLight::new(Point3::new(-1., 2.0, -4.), Color::new(0., 1., 0.));
    scene.add_light(Box::new(light));

    let light = PointLight::new(Point3::new(0., 8.0, -4.), Color::new(0., 0., 1.));
    scene.add_light(Box::new(light));

    let ambient = Color::new(0.1, 0.1, 0.1);
    scene.set_ambient(&ambient);
}

fn draw_to_terminal(scene: &Scene) {
    let x_res = 100;
    let y_res = 50;
    let camera = Camera::new(x_res, y_res);
    let mut buffer = RenderBuffer::new(x_res, y_res);
    render(&camera, scene, &mut buffer, 5);
    terminal::draw(&buffer);
}

fn render(camera: &Camera, scene: &Scene, buffer: &mut RenderBuffer, depth: usize) {
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
                (1., i.material.refraction_index)
            } else {
                (i.material.refraction_index, 1.)
            };

            let ambient = (i.material.ambient)(i.tex_coord) * scene.ambient();

            let lights: Color = get_light_energy(scene, &i)
                .iter()
                .map(|(ldir, lenergy)| {
                    let fresnel = fresnel_reflection(&ldir, &i.normal, n1, n2);
                    fresnel * i.material.get_reflected_energy(&lenergy, &ldir, &i)
                })
                .sum();

            let reflected = if i.material.reflectivity > EPSILON {
                // compute reflection vector
                let reflect_ray = reflect_ray(ray, &i);
                // compute incoming energy from the direction of the reflected ray
                let energy = trace_ray(scene, &reflect_ray, depth - 1);
                let fresnel = fresnel_reflection(&reflect_ray.direction(), &i.normal, n1, n2);
                fresnel
                    * i.material
                        .get_reflected_energy(&energy, &reflect_ray.direction(), &i)
            } else {
                BLACK
            };

            let refracted = if i.material.refraction_index > EPSILON {
                let refract_ray = refract_ray(ray, &i, n1, n2);
                (i.material.diffuse)(i.tex_coord)
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

fn reflect_ray(ray: &Ray, i: &Intersection) -> Ray {
    // compute reflection vector
    let reflected_dir = -ray.direction().reflect(&i.normal).norm();
    let p = i.point + 0.0002 * reflected_dir;
    Ray::new(&p, &reflected_dir)
}

fn refract_ray(ray: &Ray, i: &Intersection, n1: f32, n2: f32) -> Option<Ray> {
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
fn fresnel_reflection(light_dir: &Vector3, normal: &Vector3, n1: f32, n2: f32) -> f32 {
    let m_dot_r = light_dir.dot(&normal);
    let r0 = ((n1 - n2) / (n1 + n2)) * ((n1 - n2) / (n1 + n2));

    r0 + (1. - r0) * (1. - m_dot_r).powi(5)
}

/// Use Schlick's approximation to compute the amount of energy transmitted through a material
/// (this is the energy which is not reflected)
fn fresnel_refraction(light_dir: &Vector3, normal: &Vector3, n1: f32, n2: f32) -> f32 {
    1. - fresnel_reflection(light_dir, normal, n1, n2)
}

fn get_light_energy(scene: &Scene, i: &Intersection) -> Vec<(Vector3, Color)> {
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

    use super::scene::{colors::*, Color};
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

    use super::*;

    #[bench]
    fn render_128x128(b: &mut Bencher) {
        let x_res = 128;
        let y_res = 128;
        let camera = Camera::new(x_res, y_res);
        let mut buffer = RenderBuffer::new(x_res, y_res);

        let mut scene = Scene::new();
        let mut sph = Sphere::new(white, red, white, 60., 1., 0.);
        let transform = Matrix::scale(1.0, 2.25, 1.0);
        sph.set_transform(&transform);

        scene.add_shape(Box::new(sph));

        b.iter(|| super::render(&camera, &scene, &mut buffer, 5));
    }
}
