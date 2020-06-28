#![feature(test)]
//#![allow(unused_imports)]
#![allow(dead_code)]

mod bmp;
mod gui;
mod math;
mod my_scene;
mod render;
mod render_tree;
mod scene;

use std::cell::RefCell;
use std::rc::Rc;

use clap::{App, Arg, ArgMatches};

#[cfg(target_os = "unix")]
use {gui::gtk_gui::start_gui, std::collections::HashSet};

use my_scene::*;
use render::*;
use render_tree::RayForest;
use scene::Scene;

#[derive(Debug, Clone, Copy)]
pub struct Config {
    width: usize,
    height: usize,
    depth: usize,
    to_terminal: bool,
    gui: bool,
    method: Method,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Method {
    Basic,
    RayForest,
}

fn main() {
    let cargs = configure_cli().get_matches();
    let config = parse_args(&cargs);
    println!("Rendering configuration: {:?}", config);

    println!("Create Scene");
    let mut scene = Scene::new();
    create_scene(&mut scene);
    let scene = Rc::new(RefCell::new(scene));
    println!("Done Creating Scene");

    if config.gui {
        #[cfg(target_os = "unix")]
        {
            println!("Generate Forest");
            let forest = generate_forest(&config, &scene.borrow());
            let forest = Rc::new(forest);
            println!("Done Generating Forest");

            let buffer = render_forest(&config, &forest, scene.borrow().ambient());
            let buffer = Rc::new(RefCell::new(buffer));

            let mutated_shapes = Rc::new(RefCell::new(HashSet::new()));

            start_gui(
                config,
                scene.clone(),
                forest.clone(),
                mutated_shapes.clone(),
                buffer.clone(),
            );
        }
    } else {
        match config.method {
            Method::Basic => {
                println!("Rendering in Basic Mode");
                let timestamp = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .expect("Invalid time");
                let file = format!("{}.png", timestamp.as_secs());
                render_basic_to_file(&config, &scene.borrow(), "./output/", &file);
            }
            Method::RayForest => {
                println!("Rendering in RayForest Mode");
                println!("Generate Forest");
                let forest = generate_forest(&config, &scene.borrow());
                let forest = Rc::new(forest);
                println!("Done Generating Forest");

                let timestamp = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .expect("Invalid time");
                let file = format!("{}.png", timestamp.as_secs());

                render_forest_to_file(
                    &config,
                    &forest.clone(),
                    scene.borrow().ambient(),
                    "./output/",
                    &file,
                );
            }
        }
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
            Arg::with_name("gui")
                .long("gui")
                .short("g")
                .help("Open a GUI for interacting with the ray tracer.")
        )
        .arg(
            Arg::with_name("method")
            .long("method")
            .takes_value(true)
            .default_value("basic")
            .help("Sets the rendering method that will be used: 1. Basic recursive rendering or 2. the RayForest method.")
            )
        .arg(
            Arg::with_name("to-terminal")
                .long("to-terminal")
                .short("t")
                .help("Render the scene as ASCII art to the terminal")
        )
}

fn parse_args(args: &ArgMatches) -> Config {
    let width = args
        .value_of("width")
        .map(|s| s.parse::<usize>().expect("Expected integer for width"))
        .unwrap();
    let height = args
        .value_of("height")
        .map(|s| s.parse::<usize>().expect("Expected integer for height"))
        .unwrap();
    let depth = args
        .value_of("depth")
        .map(|s| s.parse::<usize>().expect("Expected integer for depth"))
        .unwrap();
    let to_terminal = args.is_present("to-terminal");
    let gui = args.is_present("gui");
    let method = match args.value_of("method").map(|v| v.to_lowercase()) {
        None => Method::Basic,
        Some(x) => {
            if x == "rayforest" {
                Method::RayForest
            } else if x == "basic" {
                Method::Basic
            } else {
                panic!("Unexpected value provided for `--method`: {}", x);
            }
        }
    };
    Config {
        width,
        height,
        depth,
        to_terminal,
        gui,
        method,
    }
}

fn render_basic_to_file(config: &Config, scene: &Scene, dir: &str, file: &str) {
    let start = std::time::Instant::now();
    let buffer = render_scene_basic(config, scene);
    bmp::save_to_bmp(dir, file, &buffer).expect("Failed to save image to disk");
    let duration = start.elapsed();
    println!("render_basic_to_file: {}ms", duration.as_millis());
}

fn render_forest_to_file(
    config: &Config,
    forest: &RayForest,
    ambient: &crate::scene::Color,
    dir: &str,
    file: &str,
) {
    let start = std::time::Instant::now();
    let buffer = render_forest(config, forest, ambient);
    bmp::save_to_bmp(dir, file, &buffer).expect("Failed to save image to disk");
    let duration = start.elapsed();
    println!("render_forest_to_file: {}ms", duration.as_millis());
}

fn render_scene_basic(config: &Config, scene: &Scene) -> RenderBuffer {
    let x_res = config.width;
    let y_res = config.height;
    let camera = Camera::new(x_res, y_res);
    let mut buffer = RenderBuffer::new(x_res, y_res);

    let start = std::time::Instant::now();
    render::render(&camera, &scene, &mut buffer, config.depth);
    let duration = start.elapsed();
    println!("render_scene: {}ms", duration.as_millis());

    #[cfg(target_os = "unix")]
    if config.to_terminal {
        draw_to_terminal(&scene);
    }

    buffer
}

fn generate_forest(config: &Config, scene: &Scene) -> RayForest {
    let x_res = config.width;
    let y_res = config.height;
    let camera = Camera::new(x_res, y_res);

    let start = std::time::Instant::now();
    let forest = render_tree::generate_ray_forest(&camera, scene, x_res, y_res, config.depth);
    let duration = start.elapsed();
    println!("generate_forest: {}ms", duration.as_millis());

    forest
}

fn render_forest(
    config: &Config,
    scene: &RayForest,
    ambient: &crate::scene::Color,
) -> RenderBuffer {
    let x_res = config.width;
    let y_res = config.height;
    let mut buffer = RenderBuffer::new(x_res, y_res);

    let start = std::time::Instant::now();
    render_tree::render_forest(scene, &mut buffer, ambient);
    let duration = start.elapsed();
    println!("render_forest: {}ms", duration.as_millis());

    buffer
}
