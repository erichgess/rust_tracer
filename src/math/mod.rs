mod matrix;
mod point;
mod ray;

use matrix::Matrix;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3{
            x, y, z
        }
    }

    pub fn neg(&self) -> Vector3 {
        Vector3{
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    // Scalar multiply
    pub fn mul(&self, a: f32) -> Vector3 {
        Vector3{
            x: self.x * a,
            y: self.y * a,
            z: self.z * a,
        }
    }

    // Scalar multiply
    pub fn div(&self, d: f32) -> Vector3 {
        Vector3{
            x: self.x / d,
            y: self.y / d,
            z: self.z / d,
        }
    }

    // Vector Add
    pub fn add(&self, v: &Vector3) -> Vector3 {
        Vector3{
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }

    // Vector Subtract
    pub fn sub(&self, v: &Vector3) -> Vector3 {
        Vector3{
            x: self.x - v.x,
            y: self.y - v.y,
            z: self.z - v.z,
        }
    }

    // length squared
    pub fn len2(&self) -> f32 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }

    // length
    pub fn len(&self) -> f32 {
        let len2 = self.len2();
        len2.sqrt()
    }

    // Dot product
    pub fn dot(&self, v: &Vector3) -> f32 {
        self.x*v.x + self.y*v.y + self.z*v.z
    }

    // Normalize
    pub fn norm(&self) -> Vector3 {
        let len = self.len();
        self.div(len)
    }

    // Cross product
    pub fn cross(&self, v: &Vector3) -> Vector3 {
        Vector3{
            x: self.y*v.z - self.z*v.y,
            y: self.z*v.x - self.x*v.z,
            z: self.x*v.y - self.y*v.x,
        }
    }

    pub fn mat_mul(&self, mat: &Matrix) -> Vector3 {
        Vector3 {
            x: self.x*mat.get(0,0) + self.y*mat.get(1,0) + self.z*mat.get(2,0),
            y: self.x*mat.get(0,1) + self.y*mat.get(1,1) + self.z*mat.get(2,1),
            z: self.x*mat.get(0,2) + self.y*mat.get(1,2) + self.z*mat.get(2,2),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Vector4 {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Vector4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vector4 {
        Vector4 {
            x, y, z, w,
        }
    }

    pub fn from(v: &Vector3) -> Vector4 {
        Vector4 {
            x: v.x, 
            y: v.y, 
            z: v.z,
            w: 1.,
        }
    }

    pub fn vec3(&self) -> Vector3 {
        Vector3::new(self.x, self.y, self.z)
    }

    pub fn neg(&self) -> Vector4 {
        Vector4{
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }

    pub fn scalar_mul(&self, a: f32) -> Vector4 {
        Vector4{
            x: self.x * a,
            y: self.y * a,
            z: self.z * a,
            w: self.w * a,
        }
    }

    pub fn scalar_div(&self, a: f32) -> Vector4 {
        Vector4{
            x: self.x / a,
            y: self.y / a,
            z: self.z / a,
            w: self.w / a,
        }
    }

    pub fn negate(&self) -> Vector4 {
        Vector4{
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }

    pub fn add(&self, v: &Vector4) -> Vector4 {
        Vector4 {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
            w: self.w + v.w,
        }
    }

    pub fn sub(&self, v: &Vector4) -> Vector4 {
        Vector4 {
            x: self.x - v.x,
            y: self.y - v.y,
            z: self.z - v.z,
            w: self.w - v.w,
        }
    }

    pub fn len2(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    pub fn len(&self) -> f32 {
        self.len2().sqrt()
    }

    pub fn dot(&self, v: &Vector4) -> f32 {
        self.x * v.x + self.y * v.y + self.z*v.z + self.w*v.w
    }

    pub fn norm(&self) -> Vector4 {
        self.scalar_div(self.len())
    }

    pub fn mat_mul(&self, mat: &Matrix) -> Vector4 {
        Vector4 {
            x: self.x*mat.get(0,0) + self.y*mat.get(1,0) + self.z*mat.get(2,0) + self.w*mat.get(3,0),
            y: self.x*mat.get(0,1) + self.y*mat.get(1,1) + self.z*mat.get(2,1) + self.w*mat.get(3,1),
            z: self.x*mat.get(0,2) + self.y*mat.get(1,2) + self.z*mat.get(2,2) + self.w*mat.get(3,2),
            w: self.x*mat.get(0,3) + self.y*mat.get(1,3) + self.z*mat.get(2,3) + self.w*mat.get(3,3),
        }
    }
}

#[cfg(test)]
mod vector3_tests {
    use super::*;
    // Test that two vectors differ by no more than
    // f32::EPSILON in each dimension
    fn assert_within_eps(a: &Vector3, b: &Vector3) {
        let diff = a.sub(b);
        assert_eq!(true, diff.x.abs() < f32::EPSILON);
        assert_eq!(true, diff.y.abs() < f32::EPSILON);
        assert_eq!(true, diff.z.abs() < f32::EPSILON);
    }

    #[test]
    fn basic() {
        let v1 = Vector3::new(1., 1., 1.);
        let v2 = Vector3::new(2., 0., 2.);

        assert_eq!(Vector3::new(2., 2., 2.), v1.mul(2.));
        assert_eq!(Vector3::new(0.5, 0.5, 0.5), v1.div(2.));
        assert_eq!(Vector3::new(3., 1., 3.), v1.add(&v2));
        assert_eq!(Vector3::new(-1., 1., -1.), v1.sub(&v2));
        assert_eq!(4., v1.dot(&v2));
        let len = 3f32;
        assert_eq!(len.sqrt(), v1.len());

        let norm = v1.norm();
        let diff = 1.0 - norm.len();
        assert_eq!(true, diff.abs() < std::f32::EPSILON);

        let x = Vector3::new(1., 0., 0.);
        let y = Vector3::new(0., 1., 0.);
        let z = x.cross(&y);
        assert_eq!(Vector3::new(0., 0., 1.), z);
    }

    #[test]
    fn scale() {
        let v1 = Vector3::new(1., 1., 1.);
        let scale = Matrix::scale(2., 3., 4.);

        let r = v1.mat_mul(&scale);
        assert_eq!(Vector3::new(2., 3., 4.), r);
    }

    #[test]
    fn translate() {
        // Should do nothing to a Vector3
        let v1 = Vector3::new(1., 1., 1.);
        let translate = Matrix::translate(2., 3., 4.);

        let r = v1.mat_mul(&translate);
        assert_eq!(Vector3::new(1., 1., 1.), r);
    }

    #[test]
    fn rotate() {
        let p = Vector3::new(1., 1., 1.);
        {
            let rotx = Matrix::rotate_x(90.);
            let r = p.mat_mul(&rotx);
            assert_within_eps(&Vector3::new(1., 1., -1.), &r);
        }
        {
            let roty = Matrix::rotate_y(90.);
            let r = p.mat_mul(&roty);
            assert_within_eps(&Vector3::new(-1., 1.,1.), &r);
        }
        {
            let rotz = Matrix::rotate_z(90.);
            let r = p.mat_mul(&rotz);
            assert_within_eps(&Vector3::new(1., -1., 1.), &r);
        }
    }
}

#[cfg(test)]
mod vector4_tests {
    use super::*;

    #[test]
    fn basic() {
        let v1 = Vector4::new(1., 1., 1., 1.);
        let v2 = Vector4::new(2., 0., 2., 1.);

        assert_eq!(Vector4::new(2., 2., 2., 2.), v1.scalar_mul(2.));
        assert_eq!(Vector4::new(0.5, 0.5, 0.5, 0.5), v1.scalar_div(2.));
        assert_eq!(Vector4::new(3., 1., 3., 2.), v1.add(&v2));
        assert_eq!(Vector4::new(-1., 1., -1., 0.), v1.sub(&v2));
        assert_eq!(5., v1.dot(&v2));
        let len = 4f32;
        assert_eq!(len.sqrt(), v1.len());

        let norm = v1.norm();
        let diff = 1.0 - norm.len();
        assert_eq!(true, diff.abs() < std::f32::EPSILON);
    }

    #[test]
    fn scale() {
        let v1 = Vector4::new(1., 1., 1., 1.);
        let scale = Matrix::scale(2., 3., 4.);

        let r = v1.mat_mul(&scale);
        assert_eq!(Vector4::new(2., 3., 4., 1.), r);
    }

    #[test]
    fn translate() {
        let v1 = Vector4::new(1., 1., 1., 1.);
        let translate = Matrix::translate(2., 3., 4.);

        let r = v1.mat_mul(&translate);
        assert_eq!(Vector4::new(1., 1., 1., 10.), r);
    }
}
