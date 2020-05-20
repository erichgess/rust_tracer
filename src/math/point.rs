use super::Vector3;

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
}

#[cfg(test)]
mod test {
    use super::*;

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
}
