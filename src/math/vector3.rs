use std::ops;

use super::matrix::Matrix;
use super::point::Point3;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn z(&self) -> f32 {
        self.z
    }

    pub fn neg(&self) -> Vector3 {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    // Scalar multiply
    pub fn scalar_mul(&self, a: f32) -> Vector3 {
        Vector3 {
            x: self.x * a,
            y: self.y * a,
            z: self.z * a,
        }
    }

    // Scalar multiply
    pub fn scalar_div(&self, d: f32) -> Vector3 {
        Vector3 {
            x: self.x / d,
            y: self.y / d,
            z: self.z / d,
        }
    }

    // Vector Add
    pub fn add(&self, v: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }

    // Vector Subtract
    pub fn sub(&self, v: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x - v.x,
            y: self.y - v.y,
            z: self.z - v.z,
        }
    }

    // length squared
    pub fn len2(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    // length
    pub fn len(&self) -> f32 {
        let len2 = self.len2();
        len2.sqrt()
    }

    // Dot product
    pub fn dot(&self, v: &Vector3) -> f32 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    // Normalize
    pub fn norm(&self) -> Vector3 {
        let len = self.len();
        self.scalar_div(len)
    }

    // Cross product
    pub fn cross(&self, v: &Vector3) -> Vector3 {
        Vector3 {
            x: self.y * v.z - self.z * v.y,
            y: self.z * v.x - self.x * v.z,
            z: self.x * v.y - self.y * v.x,
        }
    }

    pub fn mat_mul(&self, mat: &Matrix) -> Vector3 {
        Vector3 {
            x: self.x * mat.get(0, 0) + self.y * mat.get(1, 0) + self.z * mat.get(2, 0),
            y: self.x * mat.get(0, 1) + self.y * mat.get(1, 1) + self.z * mat.get(2, 1),
            z: self.x * mat.get(0, 2) + self.y * mat.get(1, 2) + self.z * mat.get(2, 2),
        }
    }

    pub fn reflect(&self, about: &Vector3) -> Vector3 {
        2. * (self.dot(about)) * about - self
    }
}

impl From<Point3> for Vector3 {
    fn from(p: Point3) -> Vector3 {
        Vector3 {
            x: p.x(),
            y: p.y(),
            z: p.z(),
        }
    }
}

impl ops::Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Self::Output {
        Vector3::neg(&self)
    }
}

impl ops::Add for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Self::Output {
        Vector3::add(&self, &rhs)
    }
}

impl ops::Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Self::Output {
        Vector3::sub(&self, &rhs)
    }
}

impl ops::Sub<&Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: &Vector3) -> Self::Output {
        Vector3::sub(&self, rhs)
    }
}

impl ops::Sub for &Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: &Vector3) -> Self::Output {
        Vector3::sub(self, rhs)
    }
}

impl ops::Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f32) -> Self::Output {
        self.scalar_mul(rhs)
    }
}

impl ops::Mul<Vector3> for f32 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        rhs.scalar_mul(self)
    }
}

impl ops::Mul<&Vector3> for f32 {
    type Output = Vector3;

    fn mul(self, rhs: &Vector3) -> Self::Output {
        rhs.scalar_mul(self)
    }
}

impl ops::Mul<Matrix> for Vector3 {
    type Output = Vector3;

    fn mul(self, _rhs: Matrix) -> Self::Output {
        self.mat_mul(&_rhs)
    }
}

impl ops::Mul<Vector3> for Matrix {
    type Output = Vector3;

    fn mul(self, _rhs: Vector3) -> Self::Output {
        self.vec3_mul(&_rhs)
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

        assert_eq!(Vector3::new(2., 2., 2.), v1 * 2.);
        assert_eq!(Vector3::new(0.5, 0.5, 0.5), v1.scalar_div(2.));
        assert_eq!(Vector3::new(3., 1., 3.), v1 + v2);
        assert_eq!(Vector3::new(-1., 1., -1.), v1 - v2);
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

        //let r = v1.mat_mul(&scale);
        let r = v1 * scale;
        assert_eq!(Vector3::new(2., 3., 4.), r);
    }

    #[test]
    fn translate() {
        // Should do nothing to a Vector3
        let v1 = Vector3::new(1., 1., 1.);
        let translate = Matrix::translate(2., 3., 4.);

        //let r = v1.mat_mul(&translate);
        let r = v1 * translate;
        assert_eq!(Vector3::new(1., 1., 1.), r);
    }

    #[test]
    fn rotate() {
        let p = Vector3::new(1., 1., 1.);
        {
            let rotx = Matrix::rotate_x(90.);
            let r = p * rotx;
            assert_within_eps(&Vector3::new(1., 1., -1.), &r);

            let rotx = Matrix::rotate_x(90.);
            let r = rotx * p;
            assert_within_eps(&Vector3::new(1., -1., 1.), &r);
        }
        {
            let roty = Matrix::rotate_y(90.);
            let r = p * roty;
            assert_within_eps(&Vector3::new(-1., 1., 1.), &r);

            let roty = Matrix::rotate_y(90.);
            let r = roty * p;
            assert_within_eps(&Vector3::new(1., 1., -1.), &r);
        }
        {
            let rotz = Matrix::rotate_z(90.);
            let r = p * rotz;
            assert_within_eps(&Vector3::new(1., -1., 1.), &r);

            let rotz = Matrix::rotate_z(90.);
            let r = rotz * p;
            assert_within_eps(&Vector3::new(-1., 1., 1.), &r);
        }
    }
}
