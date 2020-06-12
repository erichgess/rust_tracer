use super::*;

pub type ColorFun = fn((f32, f32)) -> Color;

pub trait ColorTrait {
    fn color(tx: TextureCoords) -> Color;
}

pub trait Material {
    fn get_reflected_energy(&self, incoming: &Color, light_dir: &Vector3, i: &Intersection) -> Color;
    fn to_string(&self) -> String;
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Phong {
    pub specular: ColorFun,
    pub ambient: ColorFun,
    pub diffuse: ColorFun,
    pub power: f32,
    pub reflectivity: f32,
    pub refraction_index: f32,
}

impl Phong {
    pub fn new(
        ambient: ColorFun,
        diffuse: ColorFun,
        specular: ColorFun,
        power: f32,
        reflectivity: f32,
        refraction_index: f32,
    ) -> Phong {
        Phong {
            ambient,
            diffuse,
            specular,
            power,
            reflectivity,
            refraction_index,
        }
    }
}

impl Material for Phong {
    /// Use Phong reflection model to compute the intensity of light reflected
    /// in the direction of the eye
    fn get_reflected_energy(
        &self,
        incoming: &Color,
        light_dir: &Vector3,
        i: &Intersection,
    ) -> Color {
        let diffuse = lambert(
            &light_dir,
            &i.normal,
            &incoming,
            &(self.diffuse)(i.tex_coord),
        );
        let specular = phong(
            self.power,
            &i.eye_dir,
            &light_dir,
            &i.normal,
            &incoming,
            &(self.specular)(i.tex_coord),
        );
        diffuse + specular
    }

    fn to_string(&self) -> String {
        format!("Phong")
    }
}

fn lambert(light_dir: &Vector3, normal: &Vector3, light: &Color, surface: &Color) -> Color {
    light_dir.dot(normal) * light * surface
}

fn phong(
    power: f32,
    eye_dir: &Vector3,
    light_dir: &Vector3,
    normal: &Vector3,
    light: &Color,
    surface: &Color,
) -> Color {
    let h = (eye_dir.norm() + light_dir.norm()).norm();
    let m_dot_h = normal.dot(&h);

    if m_dot_h < 0. {
        colors::BLACK
    } else {
        m_dot_h.powf(power) * light * surface
    }
}
