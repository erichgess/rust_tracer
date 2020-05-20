use std::ops;

use super::matrix::Matrix;
use super::Vector3;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Point3 {
    pub fn new(x: f32, y: f32, z: f32) -> Point3 {
        Point3 { x, y, z }
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

    pub fn scalar_mul(&self, a: f32) -> Point3 {
        Point3 {
            x: self.x * a,
            y: self.y * a,
            z: self.z * a,
        }
    }

    pub fn sub(&self, q: &Point3) -> Vector3 {
        Vector3 {
            x: self.x - q.x,
            y: self.y - q.y,
            z: self.z - q.z,
        }
    }

    pub fn add(&self, v: &Vector3) -> Point3 {
        Point3 {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }

    pub fn mat_mul(&self, mat: &Matrix) -> Point3 {
        Point3 {
            x: self.x * mat.get(0, 0)
                + self.y * mat.get(1, 0)
                + self.z * mat.get(2, 0)
                + mat.get(3, 0),
            y: self.x * mat.get(0, 1)
                + self.y * mat.get(1, 1)
                + self.z * mat.get(2, 1)
                + mat.get(3, 1),
            z: self.x * mat.get(0, 2)
                + self.y * mat.get(1, 2)
                + self.z * mat.get(2, 2)
                + mat.get(3, 2),
        }
    }
}

// scalar * Point3
impl ops::Mul<Point3> for f32 {
    type Output = Point3;

    fn mul(self, _rhs: Point3) -> Self::Output {
        _rhs.scalar_mul(self)
    }
}
// Point3 * scalar
impl ops::Mul<f32> for Point3 {
    type Output = Point3;

    fn mul(self, _rhs: f32) -> Self::Output {
        self.scalar_mul(_rhs)
    }
}

// Point3 - Point3
impl ops::Sub for Point3 {
    type Output = Vector3;

    fn sub(self, _rhs: Point3) -> Self::Output {
        Point3::sub(&self, &_rhs)
    }
}

// Point3 + Vector3
impl ops::Add<Vector3> for Point3 {
    type Output = Point3;

    fn add(self, _rhs: Vector3) -> Self::Output {
        Point3::add(&self, &_rhs)
    }
}
// Vector3 + Point3
impl ops::Add<Point3> for Vector3 {
    type Output = Point3;

    fn add(self, _rhs: Point3) -> Self::Output {
        Point3::add(&_rhs, &self)
    }
}

// Matrix * Point3
impl ops::Mul<Point3> for Matrix {
    type Output = Point3;

    fn mul(self, _rhs: Point3) -> Self::Output {
        self.pt_mul(&_rhs)
    }
}

// Point3 * Matrix
impl ops::Mul<Matrix> for Point3 {
    type Output = Point3;

    fn mul(self, _rhs: Matrix) -> Self::Output {
        self.mat_mul(&_rhs)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // Test that two vectors differ by no more than
    // f32::EPSILON in each dimension
    fn pt_assert_within_eps(a: &Point3, b: &Point3) {
        let diff = a.sub(b);
        assert_eq!(true, diff.x.abs() < f32::EPSILON);
        assert_eq!(true, diff.y.abs() < f32::EPSILON);
        assert_eq!(true, diff.z.abs() < f32::EPSILON);
    }

    #[test]
    fn basic_math() {
        let p1 = Point3::new(1., 1., 1.);
        let p2 = Point3::new(0., 0., 0.);

        let d1 = p1 - p2;
        assert_eq!(Vector3::new(1., 1., 1.), d1);

        let d2 = p2 - p1;
        assert_eq!(Vector3::new(-1., -1., -1.), d2);

        let v1 = Vector3::new(1., 2., -3.);
        assert_eq!(Point3::new(2., 3., -2.), p1 + v1);
        assert_eq!(Point3::new(1., 2., -3.), v1 + p2);

        let r1 = 2. * p1;
        assert_eq!(Point3::new(2., 2., 2.), r1);
        let r1 = p1 * 2.;
        assert_eq!(Point3::new(2., 2., 2.), r1);
    }

    #[test]
    fn scale() {
        let v1 = Point3::new(1., 1., 1.);
        let scale = Matrix::scale(2., 3., 4.);

        let r = v1 * scale;
        assert_eq!(Point3::new(2., 3., 4.), r);

        let scale = Matrix::scale(2., 3., 4.);
        let r = scale * v1;
        assert_eq!(Point3::new(2., 3., 4.), r);
    }

    #[test]
    fn translate() {
        let v1 = Point3::new(1., 1., 1.);

        let translate = Matrix::translate(2., 3., 4.);
        let r = v1 * translate;
        assert_eq!(Point3::new(1., 1., 1.), r);
        
        let translate = Matrix::translate(2., 3., 4.);
        let r = translate * v1;
        assert_eq!(Point3::new(3., 4., 5.), r);
    }

    #[test]
    fn rotate() {
        let p = Point3::new(1., 1., 1.);
        {
            let rotx = Matrix::rotate_x(90.);
            let r = p * rotx;
            pt_assert_within_eps(&Point3::new(1., 1., -1.), &r);
            
            let rotx = Matrix::rotate_x(90.);
            let r = rotx * p;
            pt_assert_within_eps(&Point3::new(1., -1., 1.), &r);
        }
        {
            let roty = Matrix::rotate_y(90.);
            let r = p * roty;
            pt_assert_within_eps(&Point3::new(-1., 1., 1.), &r);
            
            let roty = Matrix::rotate_y(90.);
            let r = roty * p;
            pt_assert_within_eps(&Point3::new(1., 1., -1.), &r);
        }
        {
            let rotz = Matrix::rotate_z(90.);
            let r = p * rotz;
            pt_assert_within_eps(&Point3::new(1., -1., 1.), &r);
            
            let rotz = Matrix::rotate_z(90.);
            let r = rotz * p;
            pt_assert_within_eps(&Point3::new(-1., 1., 1.), &r);
        }
    }
}
