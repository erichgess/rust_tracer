use super::math::{Matrix, Point3, Ray, Vector3};
use super::scene::colors::*;
use super::scene::Sphere;
use super::scene::{
    Color, Cube, Intersection, Material, TexturePhong, Phong, Plane, PointLight, Renderable, Scene, TextureCoords,
};

fn black(_: TextureCoords) -> Color {
    BLACK
}

fn red(_: TextureCoords) -> Color {
    RED
}

fn blue(_: TextureCoords) -> Color {
    0.8 * BLUE
}

fn dim_blue(_: TextureCoords) -> Color {
    0.1 * BLUE
}

fn white(_: TextureCoords) -> Color {
    WHITE
}

fn bright_gray(_: TextureCoords) -> Color {
    0.8 * WHITE
}

fn dim_white(_: TextureCoords) -> Color {
    0.1 * WHITE
}

const DIM_WHITE:Color = Color{r: 0.1, g: 0.1, b: 0.1};
const DIM_BLUE:Color = Color{r: 0., g: 0., b: 0.1};

fn checkerboard(tx: TextureCoords) -> Color {
    let u = (tx.0).abs() as i32;
    let v = (tx.1).abs() as i32;

    if tx.0 < 0. && tx.1 < 0. || tx.0 > 0. && tx.1 > 0. {
        if u % 2 == v % 2 {
            WHITE
        } else {
            0.5 * WHITE
        }
    } else {
        if u % 2 != v % 2 {
            WHITE
        } else {
            0.5 * WHITE
        }
    }
}

pub fn create_scene(scene: &mut Scene) {
    let mut sph = Sphere::new(DIM_WHITE, RED, WHITE, 60., 0.5, 0.);
    let transform =
        Matrix::translate(-1.0, 0., 0.) * Matrix::rotate_z(75.) * Matrix::scale(1.0, 0.25, 1.0);
    sph.set_transform(&transform);
    scene.add_shape(Box::new(sph));

    let mut sph2 = Sphere::new(BLACK, BLUE, DIM_BLUE, 600., 0.4, 0.);
    let transform = Matrix::translate(1., -1., 0.);
    sph2.set_transform(&transform);
    scene.add_shape(Box::new(sph2));

    let mut sph4 = Sphere::new(BLACK, WHITE, WHITE, 60., 0.7, 1.333);
    let transform = Matrix::translate(0., -0.5, -3.) * Matrix::scale(0.6, 0.6, 0.6);
    sph4.set_transform(&transform);
    scene.add_shape(Box::new(sph4));

    let plane_material = TexturePhong::new(dim_white, checkerboard, dim_white, 60., 0., 0.);
    let plane = Plane::new(
        &Point3::new(0., -2., 2.),
        &Vector3::new(0., 0., -1.),
        &plane_material,
    );
    scene.add_shape(Box::new(plane));

    let plane_material = TexturePhong::new(dim_white, checkerboard, dim_white, 60., 0., 0.);
    let plane = Plane::new(
        &Point3::new(0., -2., 0.),
        &Vector3::new(0., 1., 0.),
        &plane_material,
    );
    scene.add_shape(Box::new(plane));

    let mut cube = Cube::new(BLACK, WHITE, WHITE, 60., 0., 1.333);
    let transform = Matrix::translate(-1., -1.0, -4.) * Matrix::rotate_x(-45.0);
    cube.set_transform(&transform);
    scene.add_shape(Box::new(cube));
    let light = PointLight::new(Point3::new(4., 4.0, 0.), Color::new(1., 0., 0.));
    scene.add_light(Box::new(light));

    let light = PointLight::new(Point3::new(-1., 2.0, -4.), Color::new(0., 1., 0.));
    scene.add_light(Box::new(light));

    let light = PointLight::new(Point3::new(0., 8.0, -4.), Color::new(0., 0., 1.));
    scene.add_light(Box::new(light));

    let ambient = Color::new(0.1, 0.1, 0.1);
    scene.set_ambient(&ambient);
}
