#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

pub mod colors {
    use super::*;

    pub const BLACK: Color = Color {
        r: 0.,
        g: 0.,
        b: 0.,
    };
    pub const WHITE: Color = Color {
        r: 1.,
        g: 1.,
        b: 1.,
    };
    pub const RED: Color = Color {
        r: 1.0,
        g: 0.0,
        b: 0.0,
    };
    pub const GREEN: Color = Color {
        r: 0.0,
        g: 1.0,
        b: 0.0,
    };
    pub const BLUE: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 1.0,
    };
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color { r, g, b }
    }

    pub fn as_u8(&self) -> (u8, u8, u8) {
        let c8 = 255. * self;
        (c8.r as u8, c8.g as u8, c8.b as u8)
    }
}

impl std::ops::Add for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl std::ops::Add<&Color> for Color {
    type Output = Color;

    fn add(self, rhs: &Color) -> Self::Output {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl std::ops::Sub for Color {
    type Output = Color;

    fn sub(self, rhs: Color) -> Self::Output {
        let r = self.r - rhs.r;
        let g = self.g - rhs.g;
        let b = self.b - rhs.b;

        Color {
            r: if r < 0. { 0. } else { r },
            g: if g < 0. { 0. } else { g },
            b: if b < 0. { 0. } else { b },
        }
    }
}

impl std::ops::AddAssign for Color {
    fn add_assign(&mut self, rhs: Color) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl std::iter::Sum for Color {
    fn sum<I: Iterator<Item = Color>>(iter: I) -> Self {
        let mut total = colors::BLACK;

        for c in iter {
            total += c;
        }

        total
    }
}

impl std::ops::Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl std::ops::Mul<&Color> for Color {
    type Output = Color;

    fn mul(self, rhs: &Color) -> Self::Output {
        Color {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl std::ops::Mul<Color> for &Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl std::ops::Mul<Color> for f32 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color {
            r: self * rhs.r,
            g: self * rhs.g,
            b: self * rhs.b,
        }
    }
}

impl std::ops::Mul<&Color> for f32 {
    type Output = Color;

    fn mul(self, rhs: &Color) -> Self::Output {
        Color {
            r: self * rhs.r,
            g: self * rhs.g,
            b: self * rhs.b,
        }
    }
}

/// Implements equality comparison which takes into account the loss of
/// accuracy inherent to floating point variables.
impl PartialEq for Color {
    fn eq(&self, rhs: &Color) -> bool {
        use std::f32::EPSILON;

        (self.r - rhs.r).abs() < EPSILON
            && (self.g - rhs.g).abs() < EPSILON
            && (self.b - rhs.b).abs() < EPSILON
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(r: {}, g: {}, b: {})", self.r, self.g, self.b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let c1 = Color::new(0.2, 0.2, 0.2);
        let c2 = Color::new(0.1, 0.2, 0.3);

        assert_eq!(
            Color::new(0.02, 0.04, 0.06),
            c1 * c2,
            "Incorrect product for Color * Color"
        );
        assert_eq!(
            Color::new(0.01, 0.02, 0.03),
            0.1 * c2,
            "Incorrect product for f32 * Color"
        );
        assert_eq!(
            Color::new(0.3, 0.4, 0.5),
            c1 + c2,
            "Incorrect sum for Color + Color"
        );
    }
}

#[cfg(test)]
mod bench {
    extern crate test;
    use super::*;
    use test::Bencher;

    #[bench]
    fn color_times_color(b: &mut Bencher) {
        let c1 = Color::new(0.2, 0.2, 0.2);
        let c2 = Color::new(0.1, 0.2, 0.3);

        b.iter(|| c1 * c2);
    }

    #[bench]
    fn f32_times_color(b: &mut Bencher) {
        let c1 = Color::new(0.2, 0.2, 0.2);

        b.iter(|| 0.2 * c1);
    }

    #[bench]
    fn color_plus_color(b: &mut Bencher) {
        let c1 = Color::new(0.2, 0.2, 0.2);
        let c2 = Color::new(0.1, 0.2, 0.3);

        b.iter(|| c1 + c2);
    }
}
