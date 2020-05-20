use std::f32::*;
use super::Vector4;
use super::point::Point3;

/// Row Major matrix
pub struct Matrix {
    mat: [[f32;4];4],
}

impl Matrix {
    pub fn new() -> Matrix {
        Matrix {
            mat: [[0.;4];4],
        }
    }

    pub fn get(&self, row: usize, col: usize) -> f32 {
        self.mat[row][col]
    }

    pub fn identity() -> Matrix {
        Matrix{
            mat: [[1., 0., 0., 0.],
                  [0., 1., 0., 0.],
                  [0., 0., 1., 0.],
                  [0., 0., 0., 1.]],
        }
    }

    pub fn scalar_mul(&self, a: f32) -> Matrix {
        let mut m = Matrix::new();

        for row in 0..4 {
            for col in 0..4 {
                let x = a * self.mat[row][col];
                m.mat[row][col] = x;
            }
        }

        m
    }

    pub fn scalar_div(&self, a: f32) -> Matrix {
        let mut m = Matrix::new();

        for row in 0..4 {
            for col in 0..4 {
                let x = self.mat[row][col] / a;
                m.mat[row][col] = x;
            }
        }

        m
    }

    pub fn scale(x: f32, y: f32, z: f32) -> Matrix {
        Matrix{
            mat: [[x, 0., 0., 0.],
                  [0., y, 0., 0.],
                  [0., 0., z, 0.],
                  [0., 0., 0., 1.]],
        }
    }

    pub fn translate(x: f32, y: f32, z: f32) -> Matrix {
        Matrix{
            mat: [[1., 0., 0., x],
                  [0., 1., 0., y],
                  [0., 0., 1., z],
                  [0., 0., 0., 1.]],
        }
    }

    pub fn rotate_x(angle: f32) -> Matrix {
        let pi = consts::PI;
        let rads = angle / 180.0 * pi;
        
        Matrix{
            mat: [[1., 0., 0., 0.],
                  [0., rads.cos(), -rads.sin(), 0.],
                  [0., rads.sin(), rads.cos(), 0.],
                  [0., 0., 0., 1.]],
        }
    }

    pub fn rotate_y(angle: f32) -> Matrix {
        let pi = consts::PI;
        let rads = angle / 180.0 * pi;
        
        Matrix{
            mat: [[rads.cos(), 0., rads.sin(), 0.],
                  [0., 1., 0., 0.],
                  [-rads.sin(), 0., rads.cos(), 0.],
                  [0., 0., 0., 1.]],
        }
    }

    pub fn rotate_z(angle: f32) -> Matrix {
        let pi = consts::PI;
        let rads = angle / 180.0 * pi;
        
        Matrix{
            mat: [[rads.cos(), -rads.sin(), 0., 0.],
                  [rads.sin(), rads.cos(), 0., 0.],
                  [0., 0., 1., 0.],
                  [0., 0., 0., 1.]],
        }
    }

    pub fn vec_mul(&self, v: &Vector4) -> Vector4 {
        Vector4 {
            x: v.x*self.get(0,0) + v.y*self.get(0,1) + v.z*self.get(0,2) + v.w*self.get(0,3),
            y: v.x*self.get(1,0) + v.y*self.get(1,1) + v.z*self.get(1,2) + v.w*self.get(1,3),
            z: v.x*self.get(2,0) + v.y*self.get(2,1) + v.z*self.get(2,2) + v.w*self.get(2,3),
            w: v.x*self.get(3,0) + v.y*self.get(3,1) + v.z*self.get(3,2) + v.w*self.get(3,3),
        }
    }

    pub fn pt_mul(&self, p: &Point3) -> Point3 {
        Point3::new(
            p.x()*self.get(0,0) + p.y()*self.get(0,1) + p.z()*self.get(0,2) + self.get(0,3),
            p.x()*self.get(1,0) + p.y()*self.get(1,1) + p.z()*self.get(1,2) + self.get(1,3),
            p.x()*self.get(2,0) + p.y()*self.get(2,1) + p.z()*self.get(2,2) + self.get(2,3),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test that two vectors differ by no more than
    // f32::EPSILON in each dimension
    fn assert_within_eps(a: &Vector4, b: &Vector4) {
        let diff = a.sub(b);
        assert_eq!(true, diff.x.abs() < f32::EPSILON);
        assert_eq!(true, diff.y.abs() < f32::EPSILON);
        assert_eq!(true, diff.z.abs() < f32::EPSILON);
        assert_eq!(true, diff.w.abs() < f32::EPSILON);
    }

    // Test that two vectors differ by no more than
    // f32::EPSILON in each dimension
    fn pt_assert_within_eps(a: &Point3, b: &Point3) {
        let diff = a.sub(b);
        assert_eq!(true, diff.x.abs() < f32::EPSILON);
        assert_eq!(true, diff.y.abs() < f32::EPSILON);
        assert_eq!(true, diff.z.abs() < f32::EPSILON);
    }

    #[test]
    pub fn creation() {
        {
            let m = Matrix::new();
            assert_eq!([[0.;4];4], m.mat);
        }

        {
            let m = Matrix::identity();
            assert_eq!(
                [[1., 0., 0., 0.],
                [0., 1., 0., 0.],
                [0., 0., 1., 0.],
                [0., 0., 0., 1.]], 
                m.mat);
        }
    }

    #[test]
    pub fn scalar() {
        {
            let m = Matrix::identity().scalar_mul(3.);
            assert_eq!(
                [[3., 0., 0., 0.],
                [0., 3., 0., 0.],
                [0., 0., 3., 0.],
                [0., 0., 0., 3.]], 
                m.mat);
        }

        {
            let m = Matrix::identity().scalar_div(2.);
            assert_eq!(
                [[0.5, 0., 0., 0.],
                [0., 0.5, 0., 0.],
                [0., 0., 0.5, 0.],
                [0., 0., 0., 0.5]], 
                m.mat);
        }
    }

    #[test]
    pub fn transformations() {
        {
            let m = Matrix::scale(2., 3., 4.);
            assert_eq!(
                [[2., 0., 0., 0.],
                [0., 3., 0., 0.],
                [0., 0., 4., 0.],
                [0., 0., 0., 1.]], 
                m.mat);
        }
        {
            let m = Matrix::translate(2., 3., 4.);
            assert_eq!(
                [[1., 0., 0., 2.],
                [0., 1., 0., 3.],
                [0., 0., 1., 4.],
                [0., 0., 0., 1.]], 
                m.mat);
        }
    }

    #[test]
    fn vec_scale() {
        let v1 = Vector4::new(1., 1., 1., 1.);
        let scale = Matrix::scale(2., 3., 4.);

        let r = scale.vec_mul(&v1);
        assert_eq!(Vector4::new(2., 3., 4., 1.), r);
    }

    #[test]
    fn vec_translate() {
        let v1 = Vector4::new(1., 1., 1., 1.);
        let translate = Matrix::translate(2., 3., 4.);

        let r = translate.vec_mul(&v1);
        assert_eq!(Vector4::new(3., 4., 5., 1.), r);
    }

    #[test]
    fn vec_rotate() {
        let v1 = Vector4::new(1., 1., 1., 1.);
        {
            let rotx = Matrix::rotate_x(90.);
            let r = rotx.vec_mul(&v1);
            assert_within_eps(&Vector4::new(1., -1., 1., 1.), &r);
        }
        {
            let roty = Matrix::rotate_y(90.);
            let r = roty.vec_mul(&v1);
            assert_within_eps(&Vector4::new(1., 1.,-1., 1.), &r);
        }
        {
            let rotz = Matrix::rotate_z(90.);
            let r = rotz.vec_mul(&v1);
            assert_within_eps(&Vector4::new(-1., 1., 1., 1.), &r);
        }
    }

    #[test]
    fn pt_scale() {
        let p = Point3::new(1., 1., 1.);
        let scale = Matrix::scale(2., 3., 4.);

        let r = scale.pt_mul(&p);
        assert_eq!(Point3::new(2., 3., 4.), r);
    }

    #[test]
    fn pt_translate() {
        let p = Point3::new(1., 1., 1.);
        let translate = Matrix::translate(2., 3., 4.);

        let r = translate.pt_mul(&p);
        assert_eq!(Point3::new(3., 4., 5.), r);
    }

    #[test]
    fn pt_rotate() {
        let p = Point3::new(1., 1., 1.);
        {
            let rotx = Matrix::rotate_x(90.);
            let r = rotx.pt_mul(&p);
            pt_assert_within_eps(&Point3::new(1., -1., 1.), &r);
        }
        {
            let roty = Matrix::rotate_y(90.);
            let r = roty.pt_mul(&p);
            pt_assert_within_eps(&Point3::new(1., 1.,-1.), &r);
        }
        {
            let rotz = Matrix::rotate_z(90.);
            let r = rotz.pt_mul(&p);
            pt_assert_within_eps(&Point3::new(-1., 1., 1.), &r);
        }
    }
}
