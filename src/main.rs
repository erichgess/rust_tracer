#![feature(test)]
//#![allow(unused_imports)]
#![allow(dead_code)]

extern crate cairo;
extern crate gio;
extern crate gtk;

mod bmp;
mod gui;
mod math;
mod my_scene;
mod render;
mod render_tree;
mod scene;

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

use clap::{App, Arg, ArgMatches};
use gio::prelude::*;
use gtk::prelude::*;

use my_scene::*;
use render::*;
use render_tree::RayForest;
use scene::Scene;

#[derive(Debug, Clone, Copy)]
struct Config {
    width: usize,
    height: usize,
    depth: usize,
    to_terminal: bool,
    gui: bool,
}

fn main() {
    let cargs = configure_cli().get_matches();
    let config = parse_args(&cargs);
    println!("Rendering configuration: {:?}", config);

    println!("Create Scene");
    let mut scene = Scene::new();
    create_scene(&mut scene);
    println!("Done Creating Scene");
    println!("Generate Forest");
    let forest = generate_forest(&config, &scene);
    println!("Done Generating Forest");
    let forest = Rc::new(forest);
    let scene = Rc::new(RefCell::new(scene));
    let mutated_shapes = Rc::new(RefCell::new(HashSet::new()));

    if config.gui {
        let app =
            gtk::Application::new(Some("com.github.erichgess.rust-tracer"), Default::default())
                .expect("Initialization failed...");
        app.connect_activate(move |app| {
            let scene = Rc::clone(&scene);
            let forest = Rc::clone(&forest);
            let mutated_shapes = Rc::clone(&mutated_shapes);
            build_gui(app, config, scene, forest, mutated_shapes);
        });

        app.run(&vec![]); // Give an empty list of args bc we already processed the args above.
    } else {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Invalid time");
        let file = format!("{}.png", timestamp.as_secs());
        render_to_file(&config, &scene.borrow(), "./output/", &file);
    }
}

fn build_gui<'a>(
    app: &gtk::Application,
    config: Config,
    scene: Rc<RefCell<Scene>>,
    forest: Rc<RayForest>,
    mutated_shapes: Rc<RefCell<HashSet<i32>>>,
) {
    let window = gtk::ApplicationWindow::new(app);
    window.set_title("Rust Tracer");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::CenterOnParent);
    window.set_default_size(config.width as i32, config.height as i32);

    let mut notebook = gui::Notebook::new();
    window.add(&notebook.notebook);

    let render_box = build_render_view(config, Rc::clone(&scene), forest, mutated_shapes);
    let title = "Render";
    notebook.create_tab(title, render_box.upcast());

    let scene_desc = build_scene_description_view(&scene.borrow());
    let title = "Scene";
    notebook.create_tab(title, scene_desc.upcast());

    window.show_all();
}

fn build_scene_description_view(scene: &Scene) -> gtk::TextView {
    let text = gtk::TextView::new();
    text.set_editable(false);
    match text.get_buffer() {
        None => panic!("Could not get buffer from TextView for Scene Description"),
        Some(buffer) => {
            let mut text;
            buffer.set_text("Put Scene Shit Here");
            // Print Ambient Light
            text = format!("Ambient Light: {:?}\n", scene.ambient());

            // Print lights
            for light in scene.lights() {
                text = text + &format!("Light: {}\n", light.to_string());
            }

            // Print shapes
            for shape in scene.shapes() {
                text = text + &format!("Shape: {}\n", shape.to_string());
            }

            buffer.set_text(&text);
        }
    }
    text
}

fn build_render_view<'a>(
    config: Config,
    scene: Rc<RefCell<Scene>>,
    forest: Rc<RayForest>,
    mutated_shapes: Rc<RefCell<HashSet<i32>>>,
) -> gtk::Box {
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);

    let scrolled_box = gtk::ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
    scrolled_box.set_size_request(config.width as i32, config.height as i32);
    vbox.pack_start(&scrolled_box, true, true, 0);

    let img = gtk::Image::new();
    img.set_size_request(config.width as i32, config.height as i32);
    scrolled_box.add(&img);

    let btn = gtk::Button::new();
    btn.set_label("Render");
    vbox.pack_start(&btn, false, false, 0);

    // Setup rendering configuration controls
    let wbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    vbox.pack_start(&wbox, false, false, 0);

    let label = gtk::Label::new(Some("Width"));
    wbox.pack_start(&label, false, false, 0);
    let w_input = gtk::Entry::new();
    w_input.set_text(&format!("{}", config.width));
    wbox.pack_start(&w_input, false, false, 4);

    let label = gtk::Label::new(Some("Height"));
    wbox.pack_start(&label, false, false, 0);
    let h_input = gtk::Entry::new();
    h_input.set_text(&format!("{}", config.height));
    wbox.pack_start(&h_input, false, false, 4);

    let label = gtk::Label::new(Some("Depth"));
    wbox.pack_start(&label, false, false, 0);
    let d_input = gtk::Entry::new();
    d_input.set_text(&format!("{}", config.depth));
    wbox.pack_start(&d_input, false, false, 4);

    {
        let mutated_shapes = Rc::clone(&mutated_shapes);
        let cbox = create_shape_editor(Rc::clone(&scene), mutated_shapes);
        vbox.pack_start(&cbox, false, false, 0);
    }

    // Setup Render button to render and display the scene
    {
        let img = img.clone();
        let scene = Rc::clone(&scene);
        let forest = Rc::new(forest);
        let mutated_shapes = Rc::clone(&mutated_shapes);
        btn.connect_clicked(move |_btn| {
            let width = w_input
                .get_text()
                .map(|v| v.parse::<usize>().unwrap_or(config.width))
                .unwrap();
            let height = h_input
                .get_text()
                .map(|v| v.parse::<usize>().unwrap_or(config.height))
                .unwrap();
            let depth = d_input
                .get_text()
                .map(|v| v.parse::<usize>().unwrap_or(config.depth))
                .unwrap();
            let config = Config {
                width,
                height,
                depth,
                ..config
            };

            println!("Rendering...");
            println!("Mutated Shapes: {:?}", mutated_shapes.borrow());
            let is = render_forest_to_image_surface(&config, &forest, scene.borrow().ambient());
            img.set_from_surface(Some(&is));
            mutated_shapes.borrow_mut().clear();
        });
    }

    vbox
}

fn create_shape_editor(scene: Rc<RefCell<Scene>>, mutated_shapes: Rc<RefCell<HashSet<i32>>>) -> gtk::Box {
    let cbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);

    let mut ss = scene.borrow_mut();

    let shape_names = ss.shapes().iter().map(|sh| sh.get_name());
    let shape_list = gtk::ComboBoxText::new();
    for (i, n) in shape_names.enumerate() {
        shape_list.insert_text(i as i32, &n);
    }
    shape_list.set_active(Some(0));
    cbox.pack_start(&shape_list, false, false, 10);

    let shape = shape_list.get_active_text().unwrap().to_string();
    let sphere = ss.find_shape_mut(&shape).unwrap();
    let m = sphere.get_material_mut();
    let m = m.unwrap();
    let orig_c = m.diffuse((0., 0.));

    // Setup material adjuster slider
    let label = gtk::Label::new(Some("R"));
    cbox.pack_start(&label, false, false, 0);
    let r_slider = gtk::Scale::new(gtk::Orientation::Horizontal, None::<&gtk::Adjustment>);
    r_slider.set_range(0., 1.);
    r_slider.set_value(orig_c.r as f64);

    let shape_list = Rc::new(shape_list);
    {
        let scene = Rc::clone(&scene);
        let shape_list = Rc::clone(&shape_list);
        let mutated_shapes = Rc::clone(&mutated_shapes);
        let f = move |slider: &gtk::Scale| {
            let v = slider.get_value() as f32;
            println!("Set Red: {}", v);
            let shape = shape_list.get_active_text().unwrap().to_string();
            let mut ss = scene.borrow_mut();
            let sphere = ss.find_shape_mut(&shape).unwrap();
            mutated_shapes.borrow_mut().insert(sphere.id());
            let m = sphere.get_material_mut();
            let mut m = m.unwrap();
            let mut c = m.diffuse((0., 0.));
            c.r = v;
            m.set_diffuse(c);
        };
        r_slider.connect_value_changed(f);
        cbox.pack_start(&r_slider, true, true, 0);
    }

    // Setup material adjuster slider
    let label = gtk::Label::new(Some("G"));
    cbox.pack_start(&label, false, false, 0);
    let g_slider = gtk::Scale::new(gtk::Orientation::Horizontal, None::<&gtk::Adjustment>);
    g_slider.set_range(0., 1.);
    g_slider.set_value(orig_c.g as f64);
    {
        let scene = Rc::clone(&scene);
        let shape_list = Rc::clone(&shape_list);
        let mutated_shapes = Rc::clone(&mutated_shapes);
        let f = move |slider: &gtk::Scale| {
            let v = slider.get_value() as f32;
            println!("Set Green: {}", v);
            let shape = shape_list.get_active_text().unwrap().to_string();
            let mut ss = scene.borrow_mut();
            let sphere = ss.find_shape_mut(&shape).unwrap();
            mutated_shapes.borrow_mut().insert(sphere.id());
            let m = sphere.get_material_mut();
            let mut m = m.unwrap();
            let mut c = m.diffuse((0., 0.));
            c.g = v;
            m.set_diffuse(c);
        };
        g_slider.connect_value_changed(f);
        cbox.pack_start(&g_slider, true, true, 5);
    }

    // Setup material adjuster slider
    let label = gtk::Label::new(Some("B"));
    cbox.pack_start(&label, false, false, 0);
    let b_slider = gtk::Scale::new(gtk::Orientation::Horizontal, None::<&gtk::Adjustment>);
    b_slider.set_range(0., 1.);
    b_slider.set_value(orig_c.b as f64);
    {
        let scene = Rc::clone(&scene);
        let shape_list = Rc::clone(&shape_list);
        let mutated_shapes = Rc::clone(&mutated_shapes);
        let f = move |slider: &gtk::Scale| {
            let v = slider.get_value() as f32;
            println!("Set Blue: {}", v);
            let shape = shape_list.get_active_text().unwrap().to_string();
            let mut ss = scene.borrow_mut();
            let sphere = ss.find_shape_mut(&shape).unwrap();
            mutated_shapes.borrow_mut().insert(sphere.id());
            let m = sphere.get_material_mut();
            let mut m = m.unwrap();
            let mut c = m.diffuse((0., 0.));
            c.b = v;
            m.set_diffuse(c);
        };
        b_slider.connect_value_changed(f);
        cbox.pack_start(&b_slider, true, true, 0);
    }

    let scene = Rc::clone(&scene);
    shape_list.connect_changed(move |list| {
        let color = {
            let shape = list.get_active_text().unwrap().to_string();
            let ss = scene.borrow();
            let sphere = ss.find_shape(&shape).unwrap();
            println!("Selected: {}", sphere.to_string());
            let m = sphere.get_material();
            let m = m.unwrap();
            m.diffuse((0., 0.))
        };
        println!("Changed");
        r_slider.set_value(color.r as f64);
        g_slider.set_value(color.g as f64);
        b_slider.set_value(color.b as f64);
    });

    cbox
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
    Config {
        width,
        height,
        depth,
        to_terminal,
        gui,
    }
}

fn render_to_file(config: &Config, scene: &Scene, dir: &str, file: &str) {
    let start = std::time::Instant::now();
    let buffer = render_scene(config, scene);
    let duration = start.elapsed();
    println!("Render and draw time: {}ms", duration.as_millis());

    bmp::save_to_bmp(dir, file, &buffer).expect("Failed to save image to disk");
}

fn render_to_image_surface(config: &Config, scene: &Scene) -> cairo::ImageSurface {
    use cairo::{Format, ImageSurface};

    let start = std::time::Instant::now();
    let buffer = render_scene(config, scene);
    let duration = start.elapsed();
    println!("Render and draw time: {}ms", duration.as_millis());

    let mut surface =
        ImageSurface::create(Format::Rgb24, config.width as i32, config.height as i32)
            .expect("Failed to crate ImageSurface");
    {
        let mut sd = surface.get_data().expect("Could not get SurfaceData");
        for y in 0..config.height {
            for x in 0..config.width {
                let sd_idx = 4 * config.width * y + 4 * x;
                let (r, g, b) = buffer.buf[x][y].as_u8();
                sd[sd_idx + 0] = b;
                sd[sd_idx + 1] = g;
                sd[sd_idx + 2] = r;
            }
        }
    }

    surface
}

fn render_forest_to_image_surface(
    config: &Config,
    forest: &RayForest,
    ambient: &crate::scene::Color,
) -> cairo::ImageSurface {
    use cairo::{Format, ImageSurface};

    let start = std::time::Instant::now();
    let buffer = render_forest(config, forest, ambient);
    let duration = start.elapsed();
    println!("Render and draw time: {}ms", duration.as_millis());

    let mut surface =
        ImageSurface::create(Format::Rgb24, config.width as i32, config.height as i32)
            .expect("Failed to crate ImageSurface");
    {
        let mut sd = surface.get_data().expect("Could not get SurfaceData");
        for y in 0..config.height {
            for x in 0..config.width {
                let sd_idx = 4 * config.width * y + 4 * x;
                let (r, g, b) = buffer.buf[x][y].as_u8();
                sd[sd_idx + 0] = b;
                sd[sd_idx + 1] = g;
                sd[sd_idx + 2] = r;
            }
        }
    }

    surface
}

fn render_scene(config: &Config, scene: &Scene) -> RenderBuffer {
    let x_res = config.width;
    let y_res = config.height;
    let camera = Camera::new(x_res, y_res);
    let mut buffer = RenderBuffer::new(x_res, y_res);

    render_tree::render(&camera, &scene, &mut buffer, config.depth);

    if config.to_terminal {
        draw_to_terminal(&scene);
    }

    buffer
}

fn generate_forest(config: &Config, scene: &Scene) -> RayForest {
    let x_res = config.width;
    let y_res = config.height;
    let camera = Camera::new(x_res, y_res);

    //render_tree::render(&camera, &scene, &mut buffer, config.depth);
    render_tree::generate_ray_forest(&camera, scene, x_res, y_res, config.depth)
}

fn render_forest(
    config: &Config,
    scene: &RayForest,
    ambient: &crate::scene::Color,
) -> RenderBuffer {
    let x_res = config.width;
    let y_res = config.height;
    let mut buffer = RenderBuffer::new(x_res, y_res);

    //render_tree::render(&camera, &scene, &mut buffer, config.depth);
    render_tree::render_forest(scene, &mut buffer, ambient);

    buffer
}
