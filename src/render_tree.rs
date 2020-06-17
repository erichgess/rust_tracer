use super::math::{Ray, Vector3};
use super::render::{Camera, fresnel_reflection, fresnel_refraction, get_light_energy, RenderBuffer, reflect_ray, refract_ray};
use super::scene::{
    colors::BLACK, Color, Intersection, PointLight, Renderable, Scene,
};

enum RayTree {
    None,
    Branch(Intersection, Vec<(Vector3,Color)>, Box<RayTree>, Box<RayTree>)
}

pub fn render(camera: &Camera, scene: &Scene, buffer: &mut RenderBuffer, depth: usize) {
    let mut build_time = std::time::Duration::default();
    let mut render_time = std::time::Duration::default();
    for v in 0..camera.y_res {
        for u in 0..camera.x_res {
            let start = std::time::Instant::now();
            let ray = camera.get_ray(u, v);
            let tree = build_ray_tree(scene, &ray, depth);
            let duration = start.elapsed();
            build_time += duration;

            let start = std::time::Instant::now();
            buffer.buf[u][v] = render_ray(&tree, scene.ambient()).0;
            let duration = start.elapsed();
            render_time += duration;
        }
    }
    println!("Total Time Building: {}", build_time.as_millis());
    println!("Total Time Rendering: {}", render_time.as_millis());
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
                        .map(|r| {
                            build_ray_tree(scene, &r, depth - 1)
                        })
                        .unwrap_or(RayTree::None)
            } else {
                RayTree::None
            };

            RayTree::Branch(i, lights, Box::new(reflected), Box::new(refracted))
        }
    }
}

//TODO: I think using `i.eye_dir` for the reflection and refration directions when
// computing the fresnel coefficients is wrong and is causing colors to be much brighter
// than they should be
fn render_ray(tree: &RayTree, ambient: &Color) -> (Color, Vector3) {
    use std::f32::EPSILON;

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

            let reflected = if i.material.reflectivity() > EPSILON {
                // compute incoming energy from the direction of the reflected ray
                let (energy, dir) = render_ray(reflected, ambient);
                let fresnel = fresnel_reflection(&dir, &i.normal, n1, n2);
                fresnel * i.material.get_reflected_energy(&energy, &i.eye_dir, &i)
            } else {
                BLACK
            };

            let refracted = if i.material.refraction_index() > EPSILON {
                let (energy, dir) = render_ray(refracted, ambient);
                let fresnel = fresnel_refraction(&dir, &i.normal.neg(), n1, n2);
                fresnel * energy
            } else {
                BLACK
            };

            let ambient = (i.material.ambient(i.tex_coord)) * ambient;
            (ambient + lights + reflected + refracted, -i.eye_dir)
        }
    }
}
