use super::Vector3;
use super::point::Point3;

struct Ray {
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

#[cfg(test)]
mod test {
    use super::*;
    
    fn assert_within_eps(a: &Vector3, b: &Vector3) {
        let diff = a.sub(b);
        assert_eq!(true, diff.x.abs() < f32::EPSILON);
        assert_eq!(true, diff.y.abs() < f32::EPSILON);
        assert_eq!(true, diff.z.abs() < f32::EPSILON);
    }

    #[test]
    fn basic() {
        let ray = Ray::new(&Point3::new(0., 0., 0.), &Vector3::new(1., 1., 1.));

        let neg_ray = ray.neg();
        assert_eq!(Point3::new(0., 0., 0.), neg_ray.origin());
        assert_eq!(Vector3::new(-1., -1., -1.), neg_ray.direction());

        let norm_ray = ray.norm();
        assert_eq!(Point3::new(0., 0., 0.), norm_ray.origin());
        assert_within_eps(&Vector3::new(1., 1., 1.).norm(), &norm_ray.direction());
    }
}
