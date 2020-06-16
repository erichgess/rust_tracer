use super::math::{Ray, Vector3};
use super::render::{fresnel_reflection, fresnel_refraction, get_light_energy, reflect_ray, refract_ray};
use super::scene::{
    colors::BLACK, Color, Intersection, PointLight, Renderable, Scene,
};

enum RayTree {
    None,
    Branch(Intersection, Vec<(Vector3,Color)>, Box<RayTree>, Box<RayTree>)
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
                trace_ray(scene, &reflect_ray, depth - 1)
            } else {
                RayTree::None
            };

            let refracted = if i.material.refraction_index() > EPSILON {
                let refract_ray = refract_ray(ray, &i, n1, n2);
                refract_ray
                        .map(|r| {
                            trace_ray(scene, &r, depth - 1)
                        })
                        .unwrap_or(RayTree::None)
            } else {
                RayTree::None
            };

            RayTree::Branch(i, lights, Box::new(reflected), Box::new(refracted))
        }
    }
}
