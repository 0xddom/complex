pub mod parser;

use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, Sub, Mul};

#[derive(Debug)]
pub struct Complex {
    real: f64,
    imaginary: f64,
}

impl Complex {
    pub fn new(real: f64, imaginary: f64) -> Complex {
        Complex {
            real: real,
            imaginary: imaginary,
        }
    }

    pub fn zero() -> Complex {
        Complex::new(0.0, 0.0)
    }

    pub fn real(&self) -> f64 {
        self.real
    }

    pub fn imaginary(&self) -> f64 {
        self.imaginary
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
            Complex { real: r, imaginary: i } if i == 0.0 => write!(f, "{}", r),
            Complex { real: r, imaginary: i } if r == 0.0 => write!(f, "{}j", i),
            Complex { real: r, imaginary: i } if i >= 0.0 => write!(f, "{}+{}j", r, i),
            Complex { real: r, imaginary: i } => write!(f, "{}{}j", r, i),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complex_constructor() {
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
    fn test_zero_constructor() {
        let input = Complex::zero();

        let expected_real = 0.0;
        let expected_imaginary = 0.0;

        assert_eq!(expected_real, input.real());
        assert_eq!(expected_imaginary, input.imaginary());
    }

    #[test]
    fn test_equality() {
        let input1 = Complex::zero();
        let input2 = Complex::new(0.0, 0.0);

        assert_eq!(input1, input2);
    }

    #[test]
    fn test_add() {
        let input1 = Complex::new(1.0, 0.0);
        let input2 = Complex::new(0.0, 1.0);

        let expected = Complex::new(1.0, 1.0);

        assert_eq!(expected, input1 + input2);
    }

    #[test]
    fn test_sub() {
        let input1 = Complex::new(1.0, 0.0);
        let input2 = Complex::new(1.0, 1.0);

        let expected = Complex::new(0.0, -1.0);

        assert_eq!(expected, input1 - input2);
    }

    #[test]
    fn test_mul() {
        let input1 = Complex::new(2.0, 1.0);
        let input2 = Complex::new(1.0, 2.0);

        let expected = Complex::new(1.0, -3.0);

        assert_eq!(expected, input1 * input2);
    }

    #[test]
    fn test_display_imaginary_0() {
        let input = Complex::new(1.0, 0.0);

        let expected = "1".to_owned();

        assert_eq!(expected, format!("{}", input));
    }

    #[test]
    fn test_display_real_0() {
        let input = Complex::new(0.0, 1.0);

        let expected = "1j".to_owned();

        assert_eq!(expected, format!("{}", input));
    }

    #[test]
    fn test_display_imaginary_gt_0() {
        let input = Complex::new(1.0, 1.0);

        let expected = "1+1j".to_owned();

        assert_eq!(expected, format!("{}", input));
    }

    #[test]
    fn test_display_imaginary_le_0() {
        let input = Complex::new(1.0, -1.0);

        let expected = "1-1j".to_owned();

        assert_eq!(expected, format!("{}", input));
    }
        
}