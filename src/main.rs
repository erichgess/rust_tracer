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

use std::{cell::RefCell, io, io::prelude::*, rc::Rc};

use clap::{App, Arg, ArgMatches};

#[cfg(target_os = "linux")]
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
    interactive: bool,
    subcommand: Subcommand,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Method {
    Basic,
    RayForest,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Subcommand {
    Normal,
    Benchmark(i32, bool),
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
    if config.interactive {
        enter_to_proceed();
    }

    if config.subcommand == Subcommand::Normal {
        if config.gui {
            #[cfg(target_os = "linux")]
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

                    if config.interactive {
                        enter_to_proceed();
                    }

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
    } else if let Subcommand::Benchmark(runs, filter) = config.subcommand {
        match config.method {
            Method::Basic => {
                let start = std::time::Instant::now();
                for _ in 0..runs {
                    render_scene_basic(&config, &scene.borrow());
                }
                let duration = start.elapsed();
                println!(
                    "Total Time: {}ms | {}ns",
                    duration.as_millis(),
                    duration.as_nanos()
                );
                println!(
                    "Avg Per Op: {}ms | {}ns",
                    duration.as_millis() as f32 / runs as f32,
                    duration.as_nanos() as f32 / runs as f32
                );
            }
            Method::RayForest => {
                println!("This will benchmark evaluating the complete forest");

                println!("Rendering in RayForest Mode");
                println!("Generate Forest");
                let forest = generate_forest(&config, &scene.borrow());
                let forest = Rc::new(forest);
                println!("Done Generating Forest");

                let start = std::time::Instant::now();
                if !filter {
                    println!("Render full forest");
                    for _ in 0..runs {
                        render_forest(&config, &forest.clone(), scene.borrow().ambient());
                    }
                } else {
                    println!("Render partial forest");

                    // Get a shape who's pixels will be re-rendered
                    let shape_id = scene.borrow().find_shape("blue").unwrap().id();
                    let mut mutated_shapes = std::collections::HashSet::new();
                    mutated_shapes.insert(shape_id);
                    let mutated_shapes = Rc::new(RefCell::new(mutated_shapes));

                    let buffer = RenderBuffer::new(config.width, config.height);
                    let buffer = Rc::new(RefCell::new(buffer));

                    for _ in 0..runs {
                        render_tree::render_forest_filter(
                            &forest,
                            &mut buffer.borrow_mut(),
                            &scene.borrow().ambient(),
                            mutated_shapes.clone(),
                        );
                    }
                }
                let duration = start.elapsed();
                println!(
                    "Total Time: {}ms | {}ns",
                    duration.as_millis(),
                    duration.as_nanos()
                );
                println!(
                    "Avg Per Op: {}ms | {}ns",
                    duration.as_millis() as f32 / runs as f32,
                    duration.as_nanos() as f32 / runs as f32
                );
            }
        }
    }
}

fn enter_to_proceed() {
    let stdin = io::stdin();
    print!("Enter To Proceed: ");
    io::stdout().flush().unwrap();
    let mut _buf = String::new();
    stdin.read_line(&mut _buf).unwrap();
}

fn configure_cli<'a, 'b>() -> App<'a, 'b> {
    let app = App::new("Rust Tracer")
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
        .arg(
            Arg::with_name("interactive")
            .long("interactive")
            .short("i")
            .help("When in CLI mode, this will block after each stage of the rendering pipeline and wait for the user before proceeding.  Useful for performance analysis and debugging.")
        )
        .subcommand(
            App::new("bench")
            .about("Runs benchmark tests to aid with performance testing and analysis")
            .arg(
                Arg::with_name("runs")
                .help("How many times to run the test")
                .long("runs")
                .short("-n")
                .default_value("10")
            )
            .arg(
                Arg::with_name("filter")
                .help("Test the ray forest render filter method")
                .long("filter")
                .short("f")
            )
        );

    #[cfg(target_os = "linux")]
    {
        app.arg(
            Arg::with_name("gui")
                .long("gui")
                .short("g")
                .help("Open a GUI for interacting with the ray tracer."),
        )
    }

    #[cfg(not(target_os = "linux"))]
    {
        app
    }
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
    let interactive = args.is_present("interactive");
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

    let subcommand = args
        .subcommand_matches("bench")
        .map_or(Subcommand::Normal, |sub| {
            let n = sub
                .value_of("runs")
                .map(|n| {
                    n.parse::<i32>()
                        .expect("Expected integer for number of runs")
                })
                .unwrap();
            let filter = sub.is_present("filter");
            Subcommand::Benchmark(n, filter)
        });

    Config {
        width,
        height,
        depth,
        to_terminal,
        gui,
        method,
        interactive,
        subcommand,
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

    #[cfg(target_os = "linux")]
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
