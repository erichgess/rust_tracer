use super::math::{Ray, Vector3};
use super::render::{
    fresnel_reflection, fresnel_refraction, get_light_energy, reflect_ray, refract_ray, Camera,
    RenderBuffer,
};
use super::scene::{colors::BLACK, Color, Intersection, Renderable, Scene};

#[derive(Clone)]
enum RayTree {
    None,
    Branch(
        Intersection,
        Vec<(Vector3, Color)>,
        Box<RayTree>,
        Box<RayTree>,
    ),
}

struct RayForest<'a> {
    ambient: &'a Color,
    forest: Vec<Vec<RayTree>>,
}

impl<'a> RayForest<'a> {
    pub fn new(w: usize, h: usize, ambient: &'a Color) -> RayForest {
        RayForest {
            ambient,
            forest: vec![vec![RayTree::None; h]; w],
        }
    }
}

pub fn render(camera: &Camera, scene: &Scene, buffer: &mut RenderBuffer, depth: usize) {
    //let mut ray_forest: Vec<Vec<RayTree>> = vec![vec![RayTree::None; buffer.h]; buffer.w];
    let start = std::time::Instant::now();
    let ray_forest = generate_ray_forest(camera, scene, buffer.w, buffer.h, depth);
    let build_time = start.elapsed();

    let start = std::time::Instant::now();
    reduce_forest(&ray_forest, buffer, scene.ambient());
    let render_time = start.elapsed();

    println!("Total Time Building: {}", build_time.as_millis());
    println!("Total Time Rendering: {}", render_time.as_millis());
}

fn reduce_forest(forest: &RayForest, buffer: &mut RenderBuffer, ambient: &Color) {
    for u in 0..buffer.w {
        for v in 0..buffer.h {
            buffer.buf[u][v] = render_ray_tree(&forest.forest[u][v], ambient).0;
        }
    }
}

fn generate_ray_forest<'a>(
    camera: &'a Camera,
    scene: &'a Scene,
    w: usize,
    h: usize,
    depth: usize,
) -> RayForest<'a> {
    let mut ray_forest = RayForest::new(w, h, scene.ambient());
    for v in 0..camera.y_res {
        for u in 0..camera.x_res {
            let ray = camera.get_ray(u, v);
            let tree = build_ray_tree(scene, &ray, depth);
            ray_forest.forest[u][v] = tree;
        }
    }
    ray_forest
}

fn build_ray_tree(scene: &Scene, ray: &Ray, depth: usize) -> RayTree {
    use std::f32::EPSILON;

    if depth == 0 {
        return RayTree::None;
    }

    let hit = scene.intersect(&ray);
    match hit {
        None => RayTree::None,
        Some(i) => {
            let (n1, n2) = if i.entering {
                (1., i.material.refraction_index())
            } else {
                (i.material.refraction_index(), 1.)
            };

            let lights = get_light_energy(scene, &i);

            let reflected = if i.material.reflectivity() > EPSILON {
                // compute reflection vector
                let reflect_ray = reflect_ray(ray, &i);
                // compute incoming energy from the direction of the reflected ray
                build_ray_tree(scene, &reflect_ray, depth - 1)
            } else {
                RayTree::None
            };

            let refracted = if i.material.refraction_index() > EPSILON {
                let refract_ray = refract_ray(ray, &i, n1, n2);
                refract_ray
                    .map(|r| build_ray_tree(scene, &r, depth - 1))
                    .unwrap_or(RayTree::None)
            } else {
                RayTree::None
            };

            RayTree::Branch(i, lights, Box::new(reflected), Box::new(refracted))
        }
    }
}

fn render_ray_tree(tree: &RayTree, ambient: &Color) -> (Color, Vector3) {
    match tree {
        RayTree::None => (BLACK, Vector3::new(0., 0., 0.)),
        RayTree::Branch(ref i, lights, reflected, refracted) => {
            let (n1, n2) = if i.entering {
                (1., i.material.refraction_index())
            } else {
                (i.material.refraction_index(), 1.)
            };

            let lights: Color = lights
                .iter()
                .map(|(ldir, lenergy)| {
                    let fresnel = fresnel_reflection(&ldir, &i.normal, n1, n2);
                    fresnel * i.material.get_reflected_energy(&lenergy, &ldir, &i)
                })
                .sum();

            let reflected = {
                // compute incoming energy from the direction of the reflected ray
                let (energy, dir) = render_ray_tree(reflected, ambient);
                let fresnel = fresnel_reflection(&dir, &i.normal, n1, n2);
                fresnel * i.material.get_reflected_energy(&energy, &i.eye_dir, &i)
            };

            let refracted = {
                let (energy, dir) = render_ray_tree(refracted, ambient);
                let fresnel = fresnel_refraction(&dir, &i.normal.neg(), n1, n2);
                fresnel * energy
            };

            let ambient = (i.material.ambient(i.tex_coord)) * ambient;
            (ambient + lights + reflected + refracted, -i.eye_dir)
        }
    }
}
