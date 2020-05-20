use super::Vector3;
use super::matrix::Matrix;

#[derive(Debug, PartialEq)]
pub struct Point3{
    x: f32,
    y: f32,
    z: f32,
}

impl Point3 {
    pub fn new(x: f32, y: f32, z: f32) -> Point3 {
        Point3{x, y, z}
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
        Point3{
            x: self.x*a,
            y: self.y*a,
            z: self.z*a,
        }
    }

    pub fn sub(&self, q: &Point3) -> Vector3 {
        Vector3{
            x: self.x - q.x,
            y: self.y - q.y,
            z: self.z - q.z,
        }
    }

    pub fn add(&self, v: &Vector3) -> Point3 {
        Point3{
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }

    pub fn mat_mul(&self, mat: &Matrix) -> Point3 {
        Point3 {
            x: self.x*mat.get(0,0) + self.y*mat.get(1,0) + self.z*mat.get(2,0) + mat.get(3,0),
            y: self.x*mat.get(0,1) + self.y*mat.get(1,1) + self.z*mat.get(2,1) + mat.get(3,1),
            z: self.x*mat.get(0,2) + self.y*mat.get(1,2) + self.z*mat.get(2,2) + mat.get(3,2),
        }
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

        let d1 = p1.sub(&p2);
        assert_eq!(Vector3::new(1., 1., 1.), d1);

        let d2 = p2.sub(&p1);
        assert_eq!(Vector3::new(-1., -1., -1.), d2);

        let v1 = Vector3::new(1., 2., -3.);
        assert_eq!(Point3::new(2., 3., -2.), p1.add(&v1));
        assert_eq!(Point3::new(1., 2., -3.), p2.add(&v1));

        let r1 = p1.scalar_mul(2.);
        assert_eq!(Point3::new(2., 2., 2.), r1);
    }

    #[test]
    fn scale() {
        let v1 = Point3::new(1., 1., 1.);
        let scale = Matrix::scale(2., 3., 4.);

        let r = v1.mat_mul(&scale);
        assert_eq!(Point3::new(2., 3., 4.), r);
    }

    #[test]
    fn translate() {
        let v1 = Point3::new(1., 1., 1.);
        let translate = Matrix::translate(2., 3., 4.);

        let r = v1.mat_mul(&translate);
        assert_eq!(Point3::new(1., 1., 1.), r);
    }

    #[test]
    fn rotate() {
        let p = Point3::new(1., 1., 1.);
        {
            let rotx = Matrix::rotate_x(90.);
            let r = p.mat_mul(&rotx);
            pt_assert_within_eps(&Point3::new(1., 1., -1.), &r);
        }
        {
            let roty = Matrix::rotate_y(90.);
            let r = p.mat_mul(&roty);
            pt_assert_within_eps(&Point3::new(-1., 1.,1.), &r);
        }
        {
            let rotz = Matrix::rotate_z(90.);
            let r = p.mat_mul(&rotz);
            pt_assert_within_eps(&Point3::new(1., -1., 1.), &r);
        }
    }
}
