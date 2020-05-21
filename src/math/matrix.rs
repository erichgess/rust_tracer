use std::f32::*;
use super::{Vector3, Vector4};
use super::point::Point3;

/// Row Major matrix
#[derive(Debug, PartialEq, Copy, Clone)]
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

    pub fn mat_mul(&self, a: &Matrix) -> Matrix {
        let s = &self.mat;
        let a = &a.mat;
        let help = move|row:usize, col:usize| {
            let mut sum = 0.;
            for i in 0..4 {
                sum += s[row][i]*a[i][col];
            }
            sum
        };
        Matrix{
            mat:[
                [help(0,0), help(0,1), help(0,2), help(0,3)],
                [help(1,0), help(1,1), help(1,2), help(1,3)],
                [help(2,0), help(2,1), help(2,2), help(2,3)],
                [help(3,0), help(3,1), help(3,2), help(3,3)],
            ]
        }
    }

    pub fn transpose(&self) -> Matrix {
        let mut mat = Matrix::new();
        for row in 0..4 {
            for col in 0..4 {
                mat.mat[row][col] = self.mat[col][row];
            }
        }

        mat
    }

    pub fn inverse(&self) -> Matrix {
        let mut copy = *self;
        copy.invert();
        copy
    }

    pub fn invert(&mut self) {
        let mut inverse = Matrix::identity();

        for col in 0..4 {
            if self.mat[col][col].abs() < EPSILON {
                let mut big_row = col;
                for row in 0..4 {
                    if self.mat[row][col].abs() > self.mat[big_row][col].abs() { big_row = row; }
                }
                if big_row == col {panic!("Singular Matrix");}
                else {
                    for j in 0..4 {
                        let tmp = self.mat[big_row][j];
                        self.mat[big_row][j] = self.mat[col][j];
                        self.mat[col][j] = tmp;

                        let tmp = inverse.mat[big_row][j];
                        inverse.mat[big_row][j] = inverse.mat[col][j];
                        inverse.mat[col][j] = tmp;
                    }
                }
            }

            for row in 0..4 {
                if row != col {
                    let coeff = self.mat[row][col] / self.mat[col][col];
                    if coeff.abs() >= EPSILON {
                        for j in 0..4 {
                            self.mat[row][j] -= coeff * self.mat[col][j];
                            inverse.mat[row][j] -= coeff * inverse.mat[col][j];
                        }

                        self.mat[row][col] = 0.;
                    }
                }
            }
        }

        for row in 0..4 {
            for col in 0..4 {
                inverse.mat[row][col] /= self.mat[row][row];
            }
        }

        self.mat = inverse.mat;
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
        Vector4::new(
            v.x()*self.get(0,0) + v.y()*self.get(0,1) + v.z()*self.get(0,2) + v.w()*self.get(0,3),
            v.x()*self.get(1,0) + v.y()*self.get(1,1) + v.z()*self.get(1,2) + v.w()*self.get(1,3),
            v.x()*self.get(2,0) + v.y()*self.get(2,1) + v.z()*self.get(2,2) + v.w()*self.get(2,3),
            v.x()*self.get(3,0) + v.y()*self.get(3,1) + v.z()*self.get(3,2) + v.w()*self.get(3,3),
                )
    }

    pub fn vec3_mul(&self, v: &Vector3) -> Vector3 {
        Vector3::new(
            v.x()*self.get(0,0) + v.y()*self.get(0,1) + v.z()*self.get(0,2),
            v.x()*self.get(1,0) + v.y()*self.get(1,1) + v.z()*self.get(1,2),
            v.x()*self.get(2,0) + v.y()*self.get(2,1) + v.z()*self.get(2,2),
        )
    }

    pub fn pt_mul(&self, p: &Point3) -> Point3 {
        Point3::new(
            p.x()*self.get(0,0) + p.y()*self.get(0,1) + p.z()*self.get(0,2) + self.get(0,3),
            p.x()*self.get(1,0) + p.y()*self.get(1,1) + p.z()*self.get(1,2) + self.get(1,3),
            p.x()*self.get(2,0) + p.y()*self.get(2,1) + p.z()*self.get(2,2) + self.get(2,3),
        )
    }
}

impl std::ops::Mul for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Self::Output {
        Matrix::mat_mul(&self, &rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mat_equal(a: &Matrix, b: &Matrix, eps: f32) {
        for row in 0..4 {
            for col in 0..4 {
                let eq = (a.mat[row][col] - b.mat[row][col]).abs() < eps;
                assert!(eq, "[{}][{}] unequal.  left is {} and right is {}", row, col, a.mat[row][col], b.mat[row][col]);
            }
        }
    }

    // Test that two vectors differ by no more than
    // f32::EPSILON in each dimension
    fn assert_within_eps(a: &Vector4, b: &Vector4) {
        let diff = a.sub(b);
        assert_eq!(true, diff.x().abs() < f32::EPSILON);
        assert_eq!(true, diff.y().abs() < f32::EPSILON);
        assert_eq!(true, diff.z().abs() < f32::EPSILON);
        assert_eq!(true, diff.w().abs() < f32::EPSILON);
    }

    // Test that two vectors differ by no more than
    // f32::EPSILON in each dimension
    fn pt_assert_within_eps(a: &Point3, b: &Point3) {
        let diff = a.sub(b);
        assert_eq!(true, diff.x().abs() < f32::EPSILON);
        assert_eq!(true, diff.y().abs() < f32::EPSILON);
        assert_eq!(true, diff.z().abs() < f32::EPSILON);
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
    pub fn transpose() {
        {
            let scale = Matrix::scale(2., 3., 4.).transpose();
            assert_eq!(Matrix::scale(2., 3., 4.), scale);
        }
        {
            let translate = Matrix::translate(1., 2., 3.);
            let transpose = translate.transpose();
            assert_ne!(translate, transpose);
            assert_eq!(translate, transpose.transpose());
        }
        {
            let rotx = Matrix::rotate_x(90.).transpose();
            mat_equal(&Matrix::rotate_x(270.), &rotx, EPSILON);
        }
        {
            let roty = Matrix::rotate_y(90.).transpose();
            mat_equal(&Matrix::rotate_y(270.), &roty, EPSILON);
        }
        {
            let rotz = Matrix::rotate_z(90.).transpose();
            mat_equal(&Matrix::rotate_z(270.), &rotz, EPSILON);
        }
    }

    #[test]
    pub fn matrix_mul() {
        {
            let scale = Matrix::scale(2., 2., 2.);
            let product = scale * Matrix::identity();
            mat_equal(&scale, &product, EPSILON);
        }
        {
            let scale = Matrix::scale(2., 2., 2.);
            let scale2 = Matrix::scale(2., 2., 2.);
            let product = scale * scale2;
            mat_equal(&Matrix::scale(4., 4., 4.), &product, EPSILON);
        }
    }

    #[test]
    pub fn inverse() {
        {
            let scale = Matrix::scale(2., 3., 4.);
            let inverse = scale.inverse();
            let product = scale * inverse;
            mat_equal(&Matrix::identity(), &product, 2.*EPSILON);
        }
        {
            let mat = Matrix::identity();
            let inverse = mat.inverse();
            mat_equal(&Matrix::identity(), &inverse, 2.*EPSILON);
        }
        {
            let translate = Matrix::translate(2., 2., -4.);
            let inverse = translate.inverse();
            let product = inverse * translate;
            mat_equal(&Matrix::identity(), &product, 2.*EPSILON);
        }
        {
            let mut mat = Matrix::identity();
            mat.mat[1][3] = 4.;
            let inverse = mat.inverse();
            let product = inverse * mat;
            mat_equal(&Matrix::identity(), &product, 2.*EPSILON);
        }
        {
            let rotx = Matrix::rotate_x(82.);
            let inverse = rotx.inverse();
            let product = rotx * inverse;
            println!("{:?}", rotx);
            println!("{:?}", inverse);
            mat_equal(&Matrix::identity(), &product, 2.*EPSILON);
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
