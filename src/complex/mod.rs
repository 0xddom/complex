pub mod parser;

use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, Sub, Mul};

#[derive(Debug)]
pub struct Complex {
    real: f64,
    imaginary: f64,
}

struct PolarComplex {
    r: f64,
    theta: f64
}

impl Complex {
    pub fn new(real: f64, imaginary: f64) -> Complex {
        Complex {
            real: real,
            imaginary: imaginary,
        }
    }

    pub fn real(&self) -> f64 {
        self.real
    }

    pub fn imaginary(&self) -> f64 {
        self.imaginary
    }

    pub fn power(&self, n: f64) -> Complex {
        self.to_polar().power(n).to_cartesian()
    }

    pub fn root(&self, num: f64) -> Complex {
        self.to_polar().root(num).to_cartesian()
    }

    fn to_polar(&self) -> PolarComplex {
        PolarComplex::new(self.real, self.imaginary)
    }
}

impl PolarComplex {
    pub fn new(real: f64, imaginary: f64) -> PolarComplex {
        PolarComplex {
            r: (real * real + imaginary * imaginary).sqrt(),
            theta: (imaginary / real).atan()
        }
    }

    pub fn to_cartesian(self) -> Complex {
        Complex::new(self.r * self.theta.cos(), self.r * self.theta.sin())
    }

    pub fn root(self, num: f64) -> PolarComplex {
        PolarComplex {
            r: self.r.powf(1.0 / num),
            theta: self.theta / num
        }
    }

    pub fn power(self, num: f64) -> PolarComplex {
        PolarComplex {
            r: self.r.powf(num),
            theta: self.theta * num
        }
    }
}

impl PartialEq for Complex {
    fn eq(&self, other: &Complex) -> bool {
        self.real == other.real && self.imaginary == other.imaginary
    }
}

impl Eq for Complex {}

impl Add for Complex {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Complex::new(self.real + other.real(), self.imaginary + other.imaginary())
    }
}

impl Sub for Complex {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Complex::new(self.real - other.real(), self.imaginary - other.imaginary())
    }
}

impl Mul for Complex {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Complex::new(self.real * other.real - self.imaginary * self.imaginary,
                     self.imaginary * other.real - self.real * other.imaginary)
    }
}

impl Display for Complex {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            Complex { real: r, imaginary: i } if i == 0.0 && r % 1.0 == 0.0 => write!(f, "{}", r),
            Complex { real: r, imaginary: i } if i == 0.0 => write!(f, "{:.3}", r),
            Complex { real: r, imaginary: i } if r == 0.0 && i % 1.0 == 0.0 => write!(f, "{}j", i),
            Complex { real: r, imaginary: i } if r == 0.0 => write!(f, "{:.3}j", i),
            Complex { real: r, imaginary: i } if i >= 0.0 && i % 1.0 == 0.0 && r % 1.0 == 0.0 => write!(f, "{}+{}j", r, i),
            Complex { real: r, imaginary: i } if i >= 0.0 && i % 1.0 == 0.0 => write!(f, "{:.3}+{}j", r, i),
            Complex { real: r, imaginary: i } if i >= 0.0 && r % 1.0 == 0.0 => write!(f, "{}+{:.3}j", r, i),
            Complex { real: r, imaginary: i } if i >= 0.0 => write!(f, "{:.3}+{:.3}j", r, i),
            Complex { real: r, imaginary: i } if i % 1.0 == 0.0 && r % 1.0 == 0.0 => write!(f, "{}{}j", r, i),
            Complex { real: r, imaginary: i } if i % 1.0 == 0.0 => write!(f, "{:.3}{}j", r, i),
            Complex { real: r, imaginary: i } if r % 1.0 == 0.0 => write!(f, "{}{:.3}j", r, i),
            Complex { real: r, imaginary: i } => write!(f, "{:.3}{:.3}j", r, i),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complex_constructor__001() {
        let input_real = 1.0;
        let input_imaginary = -1.0;
        let expected_real = 1.0;
        let expected_imaginary = -1.0;

        let input = Complex::new(input_real, input_imaginary);

        let output_real = input.real();
        let output_imaginary = input.imaginary();

        assert_eq!(expected_real, output_real);
        assert_eq!(expected_imaginary, output_imaginary);
    }

    #[test]
    fn test_equality__002() {
        let input1 = Complex::new(0.0, 0.0);
        let input2 = Complex::new(0.0, 0.0);

        assert_eq!(input1, input2);
    }

    #[test]
    fn test_add__003() {
        let input1 = Complex::new(1.0, 0.0);
        let input2 = Complex::new(0.0, 1.0);

        let expected = Complex::new(1.0, 1.0);

        assert_eq!(expected, input1 + input2);
    }

    #[test]
    fn test_sub__004() {
        let input1 = Complex::new(1.0, 0.0);
        let input2 = Complex::new(1.0, 1.0);

        let expected = Complex::new(0.0, -1.0);

        assert_eq!(expected, input1 - input2);
    }

    #[test]
    fn test_mul__005() {
        let input1 = Complex::new(2.0, 1.0);
        let input2 = Complex::new(1.0, 2.0);

        let expected = Complex::new(1.0, -3.0);

        assert_eq!(expected, input1 * input2);
    }

    #[test]
    fn test_display_imaginary_0__006__007() {
        {
            let input = Complex::new(1.0, 0.0);
            let expected = "1".to_owned();
            assert_eq!(expected, format!("{}", input));
        }
        {
            let input = Complex::new(1.333, 0.0);
            let expected = "1.333".to_owned();
            assert_eq!(expected, format!("{}", input));
        }
    }

    #[test]
    fn test_display_real_0__008__009() {
        {
            let input = Complex::new(0.0, 1.0);
            let expected = "1j".to_owned();
            assert_eq!(expected, format!("{}", input));
        }
        {
            let input = Complex::new(0.0, 1.333);
            let expected = "1.333j".to_owned();
            assert_eq!(expected, format!("{}", input));
        }
    }

    #[test]
    fn test_display_imaginary_gt_0__010__011__012__013() {
        {
            let input = Complex::new(1.0, 1.0);
            let expected = "1+1j".to_owned();
            assert_eq!(expected, format!("{}", input));
        }
        {
            let input = Complex::new(1.333, 1.333);
            let expected = "1.333+1.333j".to_owned();

            assert_eq!(expected, format!("{}", input));
        }
        {
            let input = Complex::new(1.333, 1.0);
            
            let expected = "1.333+1j".to_owned();

            assert_eq!(expected, format!("{}", input));
        }
        {
            let input = Complex::new(1.0, 1.333);

            let expected = "1+1.333j".to_owned();

            assert_eq!(expected, format!("{}", input));
        }
    }

    #[test]
    fn test_display_imaginary_le_0__014__015__016__017() {
        {
            let input = Complex::new(1.0, -1.0);
            let expected = "1-1j".to_owned();
            assert_eq!(expected, format!("{}", input));
        }
        {
            let input = Complex::new(1.333, -1.333);
            let expected = "1.333-1.333j".to_owned();

            assert_eq!(expected, format!("{}", input));
        }
        {
            let input = Complex::new(1.333, -1.0);
            
            let expected = "1.333-1j".to_owned();

            assert_eq!(expected, format!("{}", input));
        }
        {
            let input = Complex::new(1.0, -1.333);

            let expected = "1-1.333j".to_owned();

            assert_eq!(expected, format!("{}", input));
        }
    }

    #[test]
    fn test_power__018() {
        let input = Complex::new(2.0, 2.0);
        let expected = Complex::new(0.0000000000000004898587196589414, 8.000000000000002);

        assert_eq!(expected, input.power(2.0));
    }

    #[test]
    fn test_root__019() {
        let input = Complex::new(2.0, 2.0);
        let expected = Complex::new(1.5537739740300374, 0.6435942529055827);

        assert_eq!(expected, input.root(2.0));
    }
}
