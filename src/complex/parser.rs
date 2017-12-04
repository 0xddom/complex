use complex::Complex;
use regex::{Regex};

// A one-liner horror story: ^\s*(?:(-?\d+(?:\.\d+)?)\s*(?:([\+-])\s*(\d+(?:\.\d+)?)j)?|(-?\d+(?:\.\d+)?)j\s*(?:([\+-])\s*(\d+(?:\.\d+)?))?)\s*$

#[derive(Debug,Eq,PartialEq)]
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
        static ref RE: Regex = Regex::new(
            r"^\s*(?:(-?)(\d+(?:\.\d+)?)\s*(?:([\+-])\s*(\d+(?:\.\d+)?)j)?|(-?)(\d+(?:\.\d+)?)j\s*(?:([\+-])\s*(\d+(?:\.\d+)?))?)\s*$"
        ).unwrap();
    }

    match RE.captures_iter(&s.to_owned()).next() {
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_complex_number_parser_first_form__020() {
        let input = "1+1j".into();
        let expected = Ok(Complex::new(1.0, 1.0));

        let output = parse_from_string(input);

        assert_eq!(expected, output);
    }

    #[test]
    fn test_complex_number_parser_second_form__021() {
        let input = "1".into();
        let expected = Ok(Complex::new(1.0, 0.0));

        let output = parse_from_string(input);

        assert_eq!(expected, output);
    }

    #[test]
    fn test_complex_number_parser_third_form__022() {
        let input = "1j+1".into();
        let expected = Ok(Complex::new(1.0, 1.0));

        let output = parse_from_string(input);

        assert_eq!(expected, output);
    }

    #[test]
    fn test_complex_number_parser_fourth_form__023() {
        let input = "1j".into();
        let expected = Ok(Complex::new(0.0, 1.0));

        let output = parse_from_string(input);

        assert_eq!(expected, output);
    }

    #[test]
    fn test_complex_number_parse_bad_input__024() {
        let input = "1+1".into();
        let expected = Err("1+1".into());

        let output = parse_from_string(input);

        assert_eq!(expected, output);
    }

    #[test]
    fn test_plus_str_to_sign__025() {
        let input = "+";
        let expected = Sign::Pos;

        let output = str_to_sign(input);

        assert_eq!(expected, output);
    }

    #[test]
    fn test_empty_string_to_sign__026() {
        let input = "";
        let expected = Sign::Pos;

        let output = str_to_sign(input);

        assert_eq!(expected, output);
    }

    #[test]
    fn test_minus_string_to_sign__027() {
        let input = "-";
        let expected = Sign::Neg;

        let output = str_to_sign(input);

        assert_eq!(expected, output);
    }

    #[test]
    fn test_create_negative_num__028() {
        let input_num = "1";
        let input_sign = Sign::Neg;
        let expected = -1.0;

        let output = create_num(input_num, input_sign);

        assert_eq!(expected, output);
    }        
}
        
