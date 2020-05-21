mod matrix;
mod point;
mod ray;
mod vector3;

use std::ops;

use matrix::Matrix;
use vector3::Vector3;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vector4 {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Vector4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vector4 {
        Vector4 { x, y, z, w }
    }

    pub fn from(v: &Vector3) -> Vector4 {
        Vector4 {
            x: v.x(),
            y: v.y(),
            z: v.z(),
            w: 1.,
        }
    }

    pub fn vec3(&self) -> Vector3 {
        Vector3::new(self.x, self.y, self.z)
    }

    pub fn neg(&self) -> Vector4 {
        Vector4 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }

    pub fn scalar_mul(&self, a: f32) -> Vector4 {
        Vector4 {
            x: self.x * a,
            y: self.y * a,
            z: self.z * a,
            w: self.w * a,
        }
    }

    pub fn scalar_div(&self, a: f32) -> Vector4 {
        Vector4 {
            x: self.x / a,
            y: self.y / a,
            z: self.z / a,
            w: self.w / a,
        }
    }

    pub fn negate(&self) -> Vector4 {
        Vector4 {
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
        self.x * v.x + self.y * v.y + self.z * v.z + self.w * v.w
    }

    pub fn norm(&self) -> Vector4 {
        self.scalar_div(self.len())
    }

    pub fn mat_mul(&self, mat: &Matrix) -> Vector4 {
        Vector4 {
            x: self.x * mat.get(0, 0)
                + self.y * mat.get(1, 0)
                + self.z * mat.get(2, 0)
                + self.w * mat.get(3, 0),
            y: self.x * mat.get(0, 1)
                + self.y * mat.get(1, 1)
                + self.z * mat.get(2, 1)
                + self.w * mat.get(3, 1),
            z: self.x * mat.get(0, 2)
                + self.y * mat.get(1, 2)
                + self.z * mat.get(2, 2)
                + self.w * mat.get(3, 2),
            w: self.x * mat.get(0, 3)
                + self.y * mat.get(1, 3)
                + self.z * mat.get(2, 3)
                + self.w * mat.get(3, 3),
        }
    }
}

impl ops::Neg for Vector4 {
    type Output = Vector4;

    fn neg(self) -> Self::Output {
        Vector4::neg(&self)
    }
}

impl ops::Add for Vector4 {
    type Output = Vector4;

    fn add(self, rhs: Vector4) -> Self::Output {
        Vector4::add(&self, &rhs)
    }
}

impl ops::Sub for Vector4 {
    type Output = Vector4;

    fn sub(self, rhs: Vector4) -> Self::Output {
        Vector4::sub(&self, &rhs)
    }
}

impl ops::Mul<f32> for Vector4 {
    type Output = Vector4;

    fn mul(self, rhs: f32) -> Self::Output {
        self.scalar_mul(rhs)
    }
}

impl ops::Mul<Vector4> for f32 {
    type Output = Vector4;

    fn mul(self, rhs: Vector4) -> Self::Output {
        rhs.scalar_mul(self)
    }
}

impl ops::Mul<Matrix> for Vector4 {
    type Output = Vector4;

    fn mul(self, _rhs: Matrix) -> Self::Output {
        self.mat_mul(&_rhs)
    }
}

impl ops::Mul<Vector4> for Matrix {
    type Output = Vector4;

    fn mul(self, _rhs: Vector4) -> Self::Output {
        self.vec_mul(&_rhs)
    }
}

#[cfg(test)]
mod vector4_tests {
    use super::*;
    // f32::EPSILON in each dimension
    fn assert_within_eps(a: &Vector4, b: &Vector4) {
        let diff = a.sub(b);
        assert_eq!(true, diff.x.abs() < f32::EPSILON, "X differs by more than epsilon");
        assert_eq!(true, diff.y.abs() < f32::EPSILON, "Y differs by more than epsilon");
        assert_eq!(true, diff.z.abs() < f32::EPSILON, "Z differs by more than epsilon");
        assert_eq!(true, diff.w.abs() < f32::EPSILON, "W differs by more than epsilon");
    }

    #[test]
    fn basic() {
        let v1 = Vector4::new(1., 1., 1., 1.);
        let v2 = Vector4::new(2., 0., 2., 1.);

        assert_eq!(Vector4::new(2., 2., 2., 2.), v1 * 2.);
        assert_eq!(Vector4::new(0.5, 0.5, 0.5, 0.5), v1.scalar_div(2.));
        assert_eq!(Vector4::new(3., 1., 3., 2.), v1 + v2);
        assert_eq!(Vector4::new(-1., 1., -1., 0.), v1 - v2);
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

        let r = v1 * scale;
        assert_eq!(Vector4::new(2., 3., 4., 1.), r);
    }

    #[test]
    fn translate() {
        let v1 = Vector4::new(1., 1., 1., 1.);
        let translate = Matrix::translate(2., 3., 4.);

        let r = v1 * translate;
        assert_eq!(Vector4::new(1., 1., 1., 10.), r);
    }

    #[test]
    fn rotate() {
        {
            let p = Vector4::new(1., 1., 1., 1.);
            let rotx = Matrix::rotate_x(90.);
            let r = p * rotx;
            assert_within_eps(&Vector4::new(1., 1., -1., 1.), &r);

            let p = Vector4::new(1., 1., 1., 1.);
            let rotx = Matrix::rotate_x(90.);
            let r = rotx * p;
            assert_within_eps(&Vector4::new(1., -1., 1., 1.), &r);
        }
        {
            let p = Vector4::new(1., 1., 1., 1.);
            let roty = Matrix::rotate_y(90.);
            let r = p * roty;
            assert_within_eps(&Vector4::new(-1., 1., 1., 1.), &r);

            let p = Vector4::new(1., 1., 1., 1.);
            let roty = Matrix::rotate_y(90.);
            let r = roty * p;
            assert_within_eps(&Vector4::new(1., 1., -1., 1.), &r);
        }
        {
            let p = Vector4::new(1., 1., 1., 1.);
            let rotz = Matrix::rotate_z(90.);
            let r = p * rotz;
            assert_within_eps(&Vector4::new(1., -1., 1., 1.), &r);

            let p = Vector4::new(1., 1., 1., 1.);
            let rotz = Matrix::rotate_z(90.);
            let r = rotz * p;
            assert_within_eps(&Vector4::new(-1., 1., 1., 1.), &r);
        }
    }
}
