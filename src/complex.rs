use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, Sub, Mul};

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

    pub fn real(&self) -> f64 {
        self.real
    }

    pub fn imaginary(&self) -> f64 {
        self.imaginary
    }
}

impl PartialEq for Complex {
    fn eq(&self, other:&Complex) -> bool {
        self.real == other.real && self.imaginary == other.imaginary
    }
}

impl Eq for Complex {}

impl Add for Complex {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        Complex::new(self.real + other.real(),
                     self.imaginary + other.imaginary())
    }
}

impl Sub for Complex {
    type Output = Self;
    
    fn sub(self, other: Self) -> Self {
        Complex::new(self.real - other.real(),
                     self.imaginary - other.imaginary())
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
            Complex { real: r, imaginary: i } if r == 0.0 => write!(f, "{}i", i),
            Complex { real: r, imaginary: i } if i >= 0.0 => write!(f, "{}+{}i", r, i),
            Complex { real: r, imaginary: i } => write!(f, "{}{}i", r, i),
        }
    }
}
