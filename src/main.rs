#![feature(test)]
//#![allow(unused_imports)]
#![allow(dead_code)]

mod bmp;
mod cli;
mod gui;
mod math;
mod my_scene;
mod render;
mod render_tree;
mod scene;

use std::{cell::RefCell, io, io::prelude::*, rc::Rc};


#[cfg(target_os = "linux")]
use {gui::gtk_gui::start_gui, std::collections::HashSet};

use cli::*;

use my_scene::*;
use render::*;
use render_tree::RayForest;
use scene::{Renderable, Scene};

fn main() {
    let cargs = configure_cli().get_matches();
    let config = parse_args(&cargs);
    println!("Rendering configuration: {:?}", config);

    println!("Create Scene");
    let mut scene = Scene::new();
    create_scene(&mut scene);
    let scene = Rc::new(RefCell::new(scene));
    println!("Done Creating Scene");

    if config.subcommand == Subcommand::Normal {
        handle_normal_mode(config, scene.clone());
    } else if let Subcommand::Benchmark(bench_config) = config.subcommand {
        handle_benchmark_mode(config, scene.clone(), bench_config.runs, bench_config.filter_mode);
    }
}

fn handle_normal_mode(config: Config, scene: Rc<RefCell<Scene>>) {
    if config.interactive {
        enter_to_proceed();
    }

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

                if config.print_forest_stats {
                    let stats = forest.stats();
                    println!("Number of Trees: {}", stats.num_trees);
                    println!("Min Tree Size: {}", stats.smallest_tree);
                    println!("Max Tree Size: {}", stats.largest_tree);
                    println!("Median Size: {}", stats.median);
                    println!("p90 Size: {}", stats.p90);
                    println!("p95 Size: {}", stats.p95);
                    println!("p99 Size: {}", stats.p99);

                    println!("Number of Intersections: {}", stats.num_intersections);

                    let num_shapes = scene.borrow().size();
                    println!("Number of Shapes: {}", num_shapes);
                    println!("Number of Intersection Tests: {}", num_shapes * stats.num_intersections);
                }

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

    fn enter_to_proceed() {
        let stdin = io::stdin();
        print!("Enter To Proceed: ");
        io::stdout().flush().unwrap();
        let mut _buf = String::new();
        stdin.read_line(&mut _buf).unwrap();
    }
}

fn handle_benchmark_mode(config: Config, scene: Rc<RefCell<Scene>>, runs: i32, filter: bool) {
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

            let duration;
            if !filter {
                println!("Render full forest");
                let start = std::time::Instant::now();
                for _ in 0..runs {
                    render_forest(&config, &forest.clone(), scene.borrow().ambient());
                }
                duration = start.elapsed();
            } else {
                println!("Render partial forest");

                // Get a shape who's pixels will be re-rendered
                let shape_id = scene.borrow().find_shape("blue").unwrap().id();
                let mut mutated_shapes = std::collections::HashSet::new();
                mutated_shapes.insert(shape_id);
                let mutated_shapes = Rc::new(RefCell::new(mutated_shapes));


                // Create render buffer
                let buffer = RenderBuffer::new(config.width, config.height);
                let buffer = Rc::new(RefCell::new(buffer));

                // Benchmark execution
                let start = std::time::Instant::now();
                for _ in 0..runs {
                    render_tree::render_forest_filter(
                        &forest,
                        &mut buffer.borrow_mut(),
                        &scene.borrow().ambient(),
                        mutated_shapes.clone(),
                    );
                }
                duration = start.elapsed();

                let tree_count = forest.size();
                let trees_with = forest.trees_with(shape_id);
                println!("Forest Size: {}", tree_count);
                println!("Trees Evaluated: {}", trees_with);
                // Print some basic facts
                println!(
                    "% evaluated: {}",
                    100. * trees_with as f32 / tree_count as f32
                );
            }

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
