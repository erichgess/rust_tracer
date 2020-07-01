#[cfg(target_os = "linux")]
pub mod gtk_gui {
    extern crate cairo;
    extern crate gio;
    extern crate gtk;

    use std::cell::*;
    use std::collections::HashSet;
    use std::rc::Rc;

    use gio::prelude::*;
    use gtk::prelude::*;

    use super::super::render::*;
    use super::super::render_tree;
    use super::super::render_tree::*;
    use super::super::scene::Scene;
    use super::super::Config;

    pub struct Notebook {
        pub notebook: gtk::Notebook,
        tabs: Vec<gtk::Box>,
    }

    impl Notebook {
        pub fn new() -> Notebook {
            Notebook {
                notebook: gtk::Notebook::new(),
                tabs: Vec::new(),
            }
        }

        pub fn create_tab(&mut self, title: &str, widget: gtk::Widget) -> u32 {
            let label = gtk::Label::new(Some(title));
            let tab = gtk::Box::new(gtk::Orientation::Horizontal, 0);

            tab.pack_start(&label, false, false, 0);
            tab.show_all();

            let index = self.notebook.append_page(&widget, Some(&tab));

            self.tabs.push(tab);
            index
        }
    }

    pub fn start_gui(
        config: Config,
        scene: Rc<RefCell<Scene>>,
        forest: Rc<RayForest>,
        mutated_shapes: Rc<RefCell<HashSet<i32>>>,
        buffer: Rc<RefCell<RenderBuffer>>,
    ) {
        let app =
            gtk::Application::new(Some("com.github.erichgess.rust-tracer"), Default::default())
                .expect("Initialization failed...");
        app.connect_activate(move |app| {
            build_gui(
                app,
                config,
                scene.clone(),
                forest.clone(),
                mutated_shapes.clone(),
                buffer.clone(),
            );
        });

        app.run(&vec![]); // Give an empty list of args bc we already processed the args above.
    }

    fn build_gui<'a>(
        app: &gtk::Application,
        config: Config,
        scene: Rc<RefCell<Scene>>,
        forest: Rc<RayForest>,
        mutated_shapes: Rc<RefCell<HashSet<i32>>>,
        buffer: Rc<RefCell<RenderBuffer>>,
    ) {
        let window = gtk::ApplicationWindow::new(app);
        window.set_title("Rust Tracer");
        window.set_border_width(10);
        window.set_position(gtk::WindowPosition::CenterOnParent);
        window.set_default_size(config.width as i32, config.height as i32);

        let mut notebook = Notebook::new();
        window.add(&notebook.notebook);

        let buffer = Rc::clone(&buffer);
        let render_box =
            build_render_view(config, Rc::clone(&scene), forest, mutated_shapes, buffer);
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
        buffer: Rc<RefCell<RenderBuffer>>,
    ) -> gtk::Box {
        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);

        let scrolled_box =
            gtk::ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
        scrolled_box.set_size_request(config.width as i32, config.height as i32);
        vbox.pack_start(&scrolled_box, true, true, 0);

        let img = gtk::Image::new();
        img.set_size_request(config.width as i32, config.height as i32);
        scrolled_box.add(&img);

        let btn = gtk::Button::new();
        btn.set_label("Render");
        vbox.pack_start(&btn, false, false, 0);

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
            let buffer = Rc::clone(&buffer);
            btn.connect_clicked(move |_btn| {
                println!("Rendering...");
                println!("Mutated Shapes: {:?}", mutated_shapes.borrow());

                let start = std::time::Instant::now();
                render_tree::render_forest_filter(
                    &forest,
                    &mut buffer.borrow_mut(),
                    scene.borrow().ambient(),
                    mutated_shapes.clone(),
                );
                let duration = start.elapsed();
                println!("render_forest_filter: {}ms", duration.as_millis());

                let surface = render_buffer_to_image_surface(&buffer.borrow());
                img.set_from_surface(Some(&surface));
                mutated_shapes.borrow_mut().clear();
            });
        }

        vbox
    }

    fn create_shape_editor(
        scene: Rc<RefCell<Scene>>,
        mutated_shapes: Rc<RefCell<HashSet<i32>>>,
    ) -> gtk::Box {
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
                let shape = shape_list.get_active_text().unwrap().to_string();
                let mut ss = scene.borrow_mut();
                let sphere = ss.find_shape_mut(&shape).unwrap();
                mutated_shapes.borrow_mut().insert(sphere.id());
                let m = sphere.get_material_mut();
                let mut m = match m {
                    None => return,
                    Some(m) => m,
                };
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
                let shape = shape_list.get_active_text().unwrap().to_string();
                let mut ss = scene.borrow_mut();
                let sphere = ss.find_shape_mut(&shape).unwrap();
                mutated_shapes.borrow_mut().insert(sphere.id());
                let m = sphere.get_material_mut();
                let mut m = match m {
                    None => return,
                    Some(m) => m,
                };
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
                let shape = shape_list.get_active_text().unwrap().to_string();
                let mut ss = scene.borrow_mut();
                let sphere = ss.find_shape_mut(&shape).unwrap();
                mutated_shapes.borrow_mut().insert(sphere.id());
                let m = sphere.get_material_mut();
                let mut m = match m {
                    None => return,
                    Some(m) => m,
                };
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
                let m = match m {
                    None => return,
                    Some(m) => m,
                };
                m.diffuse((0., 0.))
            };
            r_slider.set_value(color.r as f64);
            g_slider.set_value(color.g as f64);
            b_slider.set_value(color.b as f64);
        });

        cbox
    }

    fn render_buffer_to_image_surface(buf: &RenderBuffer) -> cairo::ImageSurface {
        use cairo::{Format, ImageSurface};
        let mut surface = ImageSurface::create(Format::Rgb24, buf.w as i32, buf.h as i32)
            .expect("Failed to crate ImageSurface");
        {
            let mut sd = surface.get_data().expect("Could not get SurfaceData");
            for y in 0..buf.h {
                for x in 0..buf.w {
                    let sd_idx = 4 * buf.w * y + 4 * x;
                    let (r, g, b) = buf.buf[x][y].as_u8();
                    sd[sd_idx + 0] = b;
                    sd[sd_idx + 1] = g;
                    sd[sd_idx + 2] = r;
                }
            }
        }

        surface
    }
}
