
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
}

#[cfg(test)]
mod tests {
    use super::*;

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
    }
}
