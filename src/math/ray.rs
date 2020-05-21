use std::ops;

use super::Vector3;
use super::matrix::Matrix;
use super::point::Point3;

#[derive(Copy, Clone)]
pub struct Ray {
    origin: Point3,
    direction: Vector3,
}

impl Ray {
    pub fn new(origin: &Point3, direction: &Vector3) -> Ray {
        Ray{
            origin: *origin,
            direction: *direction,
        }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vector3 {
        self.direction
    }

    pub fn neg(&self) -> Ray {
        Ray{
            origin: self.origin,
            direction: self.direction.neg(),
        }
    }

    pub fn norm(&self) -> Ray {
        Ray{
            origin: self.origin,
            direction: self.direction.norm(),
        }
    }
}

impl ops::Neg for Ray {
    type Output = Ray;

    fn neg(self) -> Self::Output {
        Ray::neg(&self)
    }
}

impl ops::Mul<Matrix> for Ray {
    type Output = Ray;

    fn mul(self, rhs: Matrix) -> Self::Output {
        Ray {
            origin: self.origin * rhs,
            direction: self.direction * rhs,
        }
    }
}

impl ops::Mul<Ray> for Matrix {
    type Output = Ray;

    fn mul(self, rhs: Ray) -> Self::Output {
        Ray{
            origin: self * rhs.origin,
            direction: self * rhs.direction,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    fn assert_within_eps(a: &Vector3, b: &Vector3) {
        let diff = a.sub(b);
        assert_eq!(true, diff.x.abs() < f32::EPSILON, "X is not within epsilon: {}", b.x());
        assert_eq!(true, diff.y.abs() < f32::EPSILON, "Y is not within epsilon: {}", b.y());
        assert_eq!(true, diff.z.abs() < f32::EPSILON, "Z is not within epsilon: {}", b.z());
    }
    
    fn pt_assert_within_eps(a: &Point3, b: &Point3) {
        let diff = a.sub(b);
        assert_eq!(true, diff.x.abs() < f32::EPSILON, "X is not within epsilon: {}", b.x());
        assert_eq!(true, diff.y.abs() < f32::EPSILON, "Y is not with epsilon");
        assert_eq!(true, diff.z.abs() < f32::EPSILON, "Z is not within epsilon");
    }

    #[test]
    fn basic() {
        let ray = Ray::new(&Point3::new(1., 1., 1.), &Vector3::new(1., 1., 1.));

        let neg_ray = -ray;
        assert_eq!(Point3::new(1., 1., 1.), neg_ray.origin());
        assert_eq!(Vector3::new(-1., -1., -1.), neg_ray.direction());

        let norm_ray = ray.norm();
        assert_eq!(Point3::new(1., 1., 1.), norm_ray.origin());
        assert_within_eps(&Vector3::new(1., 1., 1.).norm(), &norm_ray.direction());
    }

    #[test]
    fn ray_times_matrix() {
        let ray = Ray::new(&Point3::new(1., 1., 1.), &Vector3::new(1., 1., 1.));

        // Translate does nothing in this ordering
        let translate = Matrix::translate(1., 1., 1.);
        let ray_translated = ray * translate;
        pt_assert_within_eps(&Point3::new(1., 1., 1.), &ray_translated.origin());
        assert_within_eps(&Vector3::new(1., 1., 1.), &ray_translated.direction());

        let scale = Matrix::scale(2., 2., 2.);
        let ray_scaled = ray * scale;
        pt_assert_within_eps(&Point3::new(2., 2., 2.), &ray_scaled.origin());
        assert_within_eps(&Vector3::new(2., 2., 2.), &ray_scaled.direction());

        let rotx = Matrix::rotate_x(90.);
        let ray_rotx = ray * rotx;
        pt_assert_within_eps(&Point3::new(1., 1., -1.), &ray_rotx.origin());
        assert_within_eps(&Vector3::new(1., 1., -1.), &ray_rotx.direction());

        let roty = Matrix::rotate_y(90.);
        let ray_roty = ray * roty;
        pt_assert_within_eps(&Point3::new(-1., 1., 1.), &ray_roty.origin());
        assert_within_eps(&Vector3::new(-1., 1., 1.), &ray_roty.direction());

        let rotz = Matrix::rotate_z(90.);
        let ray_rotz = ray * rotz;
        pt_assert_within_eps(&Point3::new(1., -1., 1.), &ray_rotz.origin());
        assert_within_eps(&Vector3::new(1., -1., 1.), &ray_rotz.direction());
    }

    #[test]
    fn matrix_times_ray() {
        let ray = Ray::new(&Point3::new(1., 1., 1.), &Vector3::new(1., 1., 1.));

        // Translate does nothing in this ordering
        let translate = Matrix::translate(1., 1., 1.);
        let ray_translated = translate * ray;
        pt_assert_within_eps(&Point3::new(2., 2., 2.), &ray_translated.origin());
        assert_within_eps(&Vector3::new(1., 1., 1.), &ray_translated.direction());

        let scale = Matrix::scale(2., 2., 2.);
        let ray_scaled = scale * ray;
        pt_assert_within_eps(&Point3::new(2., 2., 2.), &ray_scaled.origin());
        assert_within_eps(&Vector3::new(2., 2., 2.), &ray_scaled.direction());

        let rotx = Matrix::rotate_x(90.);
        let ray_rotx = rotx * ray;
        pt_assert_within_eps(&Point3::new(1., -1., 1.), &ray_rotx.origin());
        assert_within_eps(&Vector3::new(1., -1., 1.), &ray_rotx.direction());

        let roty = Matrix::rotate_y(90.);
        let ray_roty = roty * ray;
        pt_assert_within_eps(&Point3::new(1., 1., -1.), &ray_roty.origin());
        assert_within_eps(&Vector3::new(1., 1., -1.), &ray_roty.direction());

        let rotz = Matrix::rotate_z(90.);
        let ray_rotz = rotz * ray;
        pt_assert_within_eps(&Point3::new(-1., 1., 1.), &ray_rotz.origin());
        assert_within_eps(&Vector3::new(-1., 1., 1.), &ray_rotz.direction());
    }
}
