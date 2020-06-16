use super::*;

pub type ColorFun = fn((f32, f32)) -> Color;

pub trait ColorTrait {
    fn color(tx: TextureCoords) -> Color;
}

pub trait Material {
    fn get_reflected_energy(&self, incoming: &Color, light_dir: &Vector3, i: &Intersection) -> Color;
    fn diffuse(&self, tx: TextureCoords) -> Color;
    fn ambient(&self, tx: TextureCoords) -> Color;
    fn reflectivity(&self) -> f32;
    fn refraction_index(&self) -> f32;
    fn to_string(&self) -> String;
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Phong {
    pub specular: Color,
    pub ambient: Color,
    pub diffuse: Color,
    pub power: f32,
    pub reflectivity: f32,
    pub refraction_index: f32,
}

impl Phong {
    pub fn new(
        ambient: Color,
        diffuse: Color,
        specular: Color,
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
    fn diffuse(&self, _: TextureCoords) -> Color {
        self.diffuse
    }

    fn ambient(&self, _: TextureCoords) -> Color {
        self.ambient
    }

    fn refraction_index(&self) -> f32 {
        self.refraction_index
    }

    fn reflectivity(&self) -> f32 {
        self.reflectivity
    }

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
            &self.diffuse,
        );
        let specular = phong(
            self.power,
            &i.eye_dir,
            &light_dir,
            &i.normal,
            &incoming,
            &self.specular,
        );
        diffuse + specular
    }

    fn to_string(&self) -> String {
        format!("Phong(Ambient: {}, Diffuse: {}, Specular: {})", self.ambient, self.diffuse, self.specular)
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct TexturePhong {
    pub specular: ColorFun,
    pub ambient: ColorFun,
    pub diffuse: ColorFun,
    pub power: f32,
    pub reflectivity: f32,
    pub refraction_index: f32,
}

impl TexturePhong {
    pub fn new(
        ambient: ColorFun,
        diffuse: ColorFun,
        specular: ColorFun,
        power: f32,
        reflectivity: f32,
        refraction_index: f32,
    ) -> TexturePhong {
        TexturePhong {
            ambient,
            diffuse,
            specular,
            power,
            reflectivity,
            refraction_index,
        }
    }

    pub fn at(&self, tx: TextureCoords) -> Phong {
        Phong::new(
            (self.ambient)(tx),
            (self.diffuse)(tx),
            (self.specular)(tx),
            self.power,
            self.reflectivity,
            self.refraction_index,
            )
    }
}

impl Material for TexturePhong {
    fn diffuse(&self, tx: TextureCoords) -> Color {
        (self.diffuse)(tx)
    }

    fn ambient(&self, tx: TextureCoords) -> Color {
        (self.ambient)(tx)
    }

    fn refraction_index(&self) -> f32 {
        self.refraction_index
    }

    fn reflectivity(&self) -> f32 {
        self.reflectivity
    }

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
