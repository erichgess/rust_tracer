use std::cell::*;
use std::collections::HashSet;
use std::rc::Rc;

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

    pub fn size(&self) -> usize {
       RayTree:: traverse_nodes(&self.root)
    }

    fn traverse_nodes(node: &RayTreeNode) -> usize {
        match node {
            RayTreeNode::None =>  0,
            RayTreeNode::Branch(_, _, l, r) => 1 + RayTree::traverse_nodes(&l) + RayTree::traverse_nodes(&r),
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

    pub fn size(&self) -> usize {
        self.forest.iter().map(|f| f.len()).sum()
    }

    /// Return the number of trees which contain the shape
    /// with the given id.
    pub fn trees_with(&self, shape_id: i32) -> usize {
        self.forest.iter().flatten().filter(|t| t.shapes.contains(&shape_id)).count()
    }

    // Compute stats about the Ray Forest
    pub fn stats(&self) -> RayForestStats {
        // compute number of intersections

        // Number of trees
        RayForestStats{
            num_trees: self.forest.iter().map(|t| t.len()).sum(),
        }
    }
}

pub struct RayForestStats {
    pub num_trees: usize,
}

pub fn render(camera: &Camera, scene: &Scene, buffer: &mut RenderBuffer, depth: usize) {
    //let mut ray_forest: Vec<Vec<RayTree>> = vec![vec![RayTree::None; buffer.h]; buffer.w];
    let start = std::time::Instant::now();
    let ray_forest = generate_ray_forest(camera, scene, buffer.w, buffer.h, depth);
    let build_time = start.elapsed();

    let start = std::time::Instant::now();
    render_forest(&ray_forest, buffer, scene.ambient());
    let render_time = start.elapsed();

    println!("generate_ray_forest: {}", build_time.as_millis());
    println!("render_forest: {}", render_time.as_millis());
}

pub fn render_forest(forest: &RayForest, buffer: &mut RenderBuffer, ambient: &Color) {
    for u in 0..buffer.w {
        for v in 0..buffer.h {
            buffer.buf[u][v] = render_ray_tree(&forest.forest[u][v].root, ambient).0;
        }
    }
}

pub fn render_forest_filter(
    forest: &RayForest,
    buffer: &mut RenderBuffer,
    ambient: &Color,
    mutated_shapes: Rc<RefCell<HashSet<i32>>>,
) {
    let mutated_shapes = mutated_shapes.borrow();
    for u in 0..buffer.w {
        for v in 0..buffer.h {
            let i = forest.forest[u][v].shapes.intersection(&mutated_shapes);
            let i: HashSet<_> = i.collect();
            if !i.is_empty() {
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

fn build_ray_tree(
    scene: &Scene,
    ray: &Ray,
    depth: usize,
    shapes: &mut HashSet<i32>,
) -> RayTreeNode {
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

#[cfg(test)]
mod tests {
    use super::*;
    use super::Intersection;
    use super::super::scene::*;
    use super::super::math::*;

    #[test]
    pub fn ray_tree_size() {
        let mut t = RayTree::new();
        t.root = RayTreeNode::None;

        assert_eq!(0, t.size());

        t.root = RayTreeNode::Branch(new_int(), vec![], Box::new(RayTreeNode::None), Box::new(RayTreeNode::None));
        assert_eq!(1, t.size());

        let r = RayTreeNode::Branch(new_int(), vec![], Box::new(RayTreeNode::None), Box::new(RayTreeNode::None));
        t.root = RayTreeNode::Branch(new_int(), vec![], Box::new(RayTreeNode::None), Box::new(r));
        assert_eq!(2, t.size());

        let l = RayTreeNode::Branch(new_int(), vec![], Box::new(RayTreeNode::None), Box::new(RayTreeNode::None));
        let r = RayTreeNode::Branch(new_int(), vec![], Box::new(RayTreeNode::None), Box::new(RayTreeNode::None));
        t.root = RayTreeNode::Branch(new_int(), vec![], Box::new(l), Box::new(r));
        assert_eq!(3, t.size());

        let ll = RayTreeNode::Branch(new_int(), vec![], Box::new(RayTreeNode::None), Box::new(RayTreeNode::None));
        let lr = RayTreeNode::Branch(new_int(), vec![], Box::new(RayTreeNode::None), Box::new(RayTreeNode::None));
        let l = RayTreeNode::Branch(new_int(), vec![], Box::new(ll), Box::new(lr));
        let rl = RayTreeNode::Branch(new_int(), vec![], Box::new(RayTreeNode::None), Box::new(RayTreeNode::None));
        let rr = RayTreeNode::Branch(new_int(), vec![], Box::new(RayTreeNode::None), Box::new(RayTreeNode::None));
        let r = RayTreeNode::Branch(new_int(), vec![], Box::new(rl), Box::new(rr));
        t.root = RayTreeNode::Branch(new_int(), vec![], Box::new(l), Box::new(r));
        assert_eq!(7, t.size());
    }

    fn new_int() -> Intersection {
        let mat = Phong::new(Color::new(0., 0., 0.), Color::new(0., 0., 0.), Color::new(0., 0., 0.), 1., 1., 1.);
        let mat = Rc::new(RefCell::new(mat));
        Intersection{
            id: 0,
            t: 0.,
            entering: false,
            material: mat,
            point: Point3::new(0., 0., 0.),
            eye_dir: Vector3::new(0., 0., 0.),
            normal: Vector3::new(0., 0., 0.),
            tex_coord: (0., 0.),
        }
    }
}
