use std::collections::HashSet;
use super::math::{Ray, Vector3};
use super::render::{
    fresnel_reflection, fresnel_refraction, get_light_energy, reflect_ray, refract_ray, Camera,
    RenderBuffer,
};
use super::scene::{colors::BLACK, Color, Intersection, Renderable, Scene};

#[derive(Clone)]
enum RayTreeNode {
    None,
    Branch(
        Intersection,
        Vec<(Vector3, Color)>,
        Box<RayTreeNode>,
        Box<RayTreeNode>,
    ),
}

#[derive(Clone)]
struct RayTree {
    dirty: bool,
    shapes: HashSet<i32>,
    root: RayTreeNode,
}

impl RayTree {
    pub fn new() -> RayTree {
        RayTree {
            dirty: false,
            shapes: HashSet::new(),
            root: RayTreeNode::None,
        }
    }
}

pub struct RayForest {
    forest: Vec<Vec<RayTree>>,
}

impl RayForest {
    pub fn new(w: usize, h: usize) -> RayForest {
        RayForest {
            forest: vec![vec![RayTree::new(); h]; w],
        }
    }
}

pub fn render(camera: &Camera, scene: &Scene, buffer: &mut RenderBuffer, depth: usize) {
    //let mut ray_forest: Vec<Vec<RayTree>> = vec![vec![RayTree::None; buffer.h]; buffer.w];
    let start = std::time::Instant::now();
    let ray_forest = generate_ray_forest(camera, scene, buffer.w, buffer.h, depth);
    let build_time = start.elapsed();

    let start = std::time::Instant::now();
    render_forest(&ray_forest, buffer, scene.ambient());
    let render_time = start.elapsed();

    println!("Total Time Building: {}", build_time.as_millis());
    println!("Total Time Rendering: {}", render_time.as_millis());
}

pub fn render_forest(forest: &RayForest, buffer: &mut RenderBuffer, ambient: &Color) {
    for u in 0..buffer.w {
        for v in 0..buffer.h {
            if forest.forest[u][v].dirty {
                buffer.buf[u][v] = render_ray_tree(&forest.forest[u][v].root, ambient).0;
            }
        }
    }
}

pub fn generate_ray_forest(
    camera: &Camera,
    scene: &Scene,
    w: usize,
    h: usize,
    depth: usize,
) -> RayForest {
    let mut ray_forest = RayForest::new(w, h);
    for v in 0..camera.y_res {
        for u in 0..camera.x_res {
            let ray = camera.get_ray(u, v);
            let tree = build_ray_tree(scene, &ray, depth, &mut ray_forest.forest[u][v].shapes);
            ray_forest.forest[u][v].root = tree;
            ray_forest.forest[u][v].dirty = true;
        }
    }
    ray_forest
}

fn build_ray_tree(scene: &Scene, ray: &Ray, depth: usize, shapes: &mut HashSet<i32>) -> RayTreeNode {
    use std::f32::EPSILON;

    if depth == 0 {
        return RayTreeNode::None;
    }

    let hit = scene.intersect(&ray);
    match hit {
        None => RayTreeNode::None,
        Some(i) => {
            shapes.insert(i.id);
            let (n1, n2) = if i.entering {
                (1., i.material.borrow().refraction_index())
            } else {
                (i.material.borrow().refraction_index(), 1.)
            };

            let lights = get_light_energy(scene, &i);

            let reflected = if i.material.borrow().reflectivity() > EPSILON {
                // compute reflection vector
                let reflect_ray = reflect_ray(ray, &i);
                // compute incoming energy from the direction of the reflected ray
                build_ray_tree(scene, &reflect_ray, depth - 1, shapes)
            } else {
                RayTreeNode::None
            };

            let refracted = if i.material.borrow().refraction_index() > EPSILON {
                let refract_ray = refract_ray(ray, &i, n1, n2);
                refract_ray
                    .map(|r| build_ray_tree(scene, &r, depth - 1, shapes))
                    .unwrap_or(RayTreeNode::None)
            } else {
                RayTreeNode::None
            };

            RayTreeNode::Branch(i, lights, Box::new(reflected), Box::new(refracted))
        }
    }
}

fn render_ray_tree(tree: &RayTreeNode, ambient: &Color) -> (Color, Vector3) {
    match tree {
        RayTreeNode::None => (BLACK, Vector3::new(0., 0., 0.)),
        RayTreeNode::Branch(ref i, lights, reflected, refracted) => {
            let (n1, n2) = if i.entering {
                (1., i.material.borrow().refraction_index())
            } else {
                (i.material.borrow().refraction_index(), 1.)
            };

            let lights: Color = lights
                .iter()
                .map(|(ldir, lenergy)| {
                    let fresnel = fresnel_reflection(&ldir, &i.normal, n1, n2);
                    fresnel
                        * i.material
                            .borrow()
                            .get_reflected_energy(&lenergy, &ldir, &i)
                })
                .sum();

            let reflected = {
                // compute incoming energy from the direction of the reflected ray
                let (energy, dir) = render_ray_tree(reflected, ambient);
                let fresnel = fresnel_reflection(&dir, &i.normal, n1, n2);
                fresnel
                    * i.material
                        .borrow()
                        .get_reflected_energy(&energy, &i.eye_dir, &i)
            };

            let refracted = {
                let (energy, dir) = render_ray_tree(refracted, ambient);
                let fresnel = fresnel_refraction(&dir, &i.normal.neg(), n1, n2);
                fresnel * energy
            };

            let ambient = (i.material.borrow().ambient(i.tex_coord)) * ambient;
            (ambient + lights + reflected + refracted, -i.eye_dir)
        }
    }
}
