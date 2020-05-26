use super::*;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Material {
    pub specular: ColorFun,
    pub ambient: ColorFun,
    pub diffuse: ColorFun,
    pub power: f32,
    pub reflectivity: f32,
    pub refraction_index: f32,
}

impl Material {
    pub fn new(
        ambient: ColorFun,
        diffuse: ColorFun,
        specular: ColorFun,
        power: f32,
        reflectivity: f32,
        refraction_index: f32,
    ) -> Material {
        Material {
            ambient,
            diffuse,
            specular,
            power,
            reflectivity,
            refraction_index,
        }
    }

    /// Use Phong reflection model to compute the intensity of light reflected
    /// in the direction of the eye
    pub fn get_reflected_energy(
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
}
