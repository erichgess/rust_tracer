mod matrix;
mod point;

#[derive(Debug, PartialEq)]
pub struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3{
            x, y, z
        }
    }

    // Scalar multiply
    pub fn mul(&self, a: f32) -> Vector3 {
        Vector3{
            x: self.x * a,
            y: self.y * a,
            z: self.z * a,
        }
    }

    // Scalar multiply
    pub fn div(&self, d: f32) -> Vector3 {
        Vector3{
            x: self.x / d,
            y: self.y / d,
            z: self.z / d,
        }
    }

    // Vector Add
    pub fn add(&self, v: &Vector3) -> Vector3 {
        Vector3{
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }

    // Vector Subtract
    pub fn sub(&self, v: &Vector3) -> Vector3 {
        Vector3{
            x: self.x - v.x,
            y: self.y - v.y,
            z: self.z - v.z,
        }
    }

    // Dot product
    pub fn dot(&self, v: &Vector3) -> f32 {
        self.x*v.x + self.y*v.y + self.z*v.z
    }

    // length
    pub fn len(&self) -> f32 {
        let len2 = self.dot(&self);
        len2.sqrt()
    }

    // Normalize
    pub fn norm(&self) -> Vector3 {
        let len = self.len();
        self.div(len)
    }

    // Cross product
    pub fn cross(&self, v: &Vector3) -> Vector3 {
        Vector3{
            x: self.y*v.z - self.z*v.y,
            y: self.z*v.x - self.x*v.z,
            z: self.x*v.y - self.y*v.x,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let v1 = Vector3::new(1., 1., 1.);
        let v2 = Vector3::new(2., 0., 2.);

        assert_eq!(Vector3::new(2., 2., 2.), v1.mul(2.));
        assert_eq!(Vector3::new(0.5, 0.5, 0.5), v1.div(2.));
        assert_eq!(Vector3::new(3., 1., 3.), v1.add(&v2));
        assert_eq!(Vector3::new(-1., 1., -1.), v1.sub(&v2));
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
}
