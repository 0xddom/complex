use complex::Complex;
use regex::{Regex, Captures};

// A one-liner horror story: ^\s*(?:(-?\d+(?:\.\d+)?)\s*(?:([\+-])\s*(\d+(?:\.\d+)?)j)?|(-?\d+(?:\.\d+)?)j\s*(?:([\+-])\s*(\d+(?:\.\d+)?))?)\s*$

enum Sign {
    Pos,
    Neg
}

fn str_to_sign(s: &str) -> Sign {
    match s {
        "-" => Sign::Neg,
        _ => Sign::Pos
    }
}

fn create_num(num_str: &str, sign: Sign) -> f64 {
    let n = num_str.parse::<f64>().unwrap();
    match sign {
        Sign::Neg => -1.0 * n,
        Sign::Pos => n
    }
}

fn build_from_strings(real_sign: &str, real: &str, imaginary_sign: &str, imaginary: &str) -> Complex {
    Complex::new(
        create_num(real, str_to_sign(real_sign)),
        create_num(imaginary, str_to_sign(imaginary_sign))
    )
}

pub fn parse_from_string(s: String) -> Result<Complex, String> {
    lazy_static! {
        static ref re: Regex = Regex::new(
            r"^\s*(?:(-?)(\d+(?:\.\d+)?)\s*(?:([\+-])\s*(\d+(?:\.\d+)?)j)?|(-?)(\d+(?:\.\d+)?)j\s*(?:([\+-])\s*(\d+(?:\.\d+)?))?)\s*$"
        ).unwrap();
    }

    match re.captures_iter(&s.to_owned()).next() {
        Some(cap) => {
            let groups = (
                cap.get(1),
                cap.get(2),
                cap.get(3),
                cap.get(4),
                cap.get(5),
                cap.get(6),
                cap.get(7),
                cap.get(8)
            );
            Ok(match groups {
                (Some(real_sign), Some(real), Some(imaginary_sign), Some(imaginary), None, None, None, None) => {
                    build_from_strings(real_sign.as_str(), real.as_str(), imaginary_sign.as_str(), imaginary.as_str())
                },
                (Some(real_sign), Some(real), None, None, None, None, None, None) => {
                    build_from_strings(real_sign.as_str(), real.as_str(), "+".into(), "0".into())
                },
                (None, None, None, None, Some(imaginary_sign), Some(imaginary), Some(real_sign), Some(real)) => {
                    build_from_strings(real_sign.as_str(), real.as_str(), imaginary_sign.as_str(), imaginary.as_str())
                },
                (None, None, None, None, Some(imaginary_sign), Some(imaginary), None, None) => {
                    build_from_strings("+".into(), "0".into(), imaginary_sign.as_str(), imaginary.as_str())
                },
                _ => panic!("Unexpected parsing combination")
            })
        },
        None => Err(s)
    }
}
