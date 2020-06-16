use super::math::{Matrix, Point3, Ray, Vector3};
use super::render::{fresnel_reflection, fresnel_refraction, get_light_energy, reflect_ray, refract_ray};
use super::scene::{
    colors::BLACK, Color, Intersection, Material, Renderable, Scene,
};

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
                (1., i.material.refraction_index())
            } else {
                (i.material.refraction_index(), 1.)
            };

            let ambient = (i.material.ambient(i.tex_coord)) * scene.ambient();

            let lights: Color = get_light_energy(scene, &i)
                .iter()
                .map(|(ldir, lenergy)| {
                    let fresnel = fresnel_reflection(&ldir, &i.normal, n1, n2);
                    fresnel * i.material.get_reflected_energy(&lenergy, &ldir, &i)
                })
                .sum();

            let reflected = if i.material.reflectivity() > EPSILON {
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

            let refracted = if i.material.refraction_index() > EPSILON {
                let refract_ray = refract_ray(ray, &i, n1, n2);
                (i.material.diffuse(i.tex_coord))
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
