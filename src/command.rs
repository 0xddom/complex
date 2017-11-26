use complex::Complex;
use std::str::SplitWhitespace;
use std::io::{Write, stdout};
use complex::parser::parse_from_string;

#[derive(Debug)]
pub enum Command {
    Help,
    Exit,
    Clear,
    Addition,
    Subtraction,
    Multiplication,
    Real,
    Imaginary,
    Power(f64),
    Root(f64),
    Number(Complex),
}

fn parse_command(head: &str, mut tail: SplitWhitespace) -> Result<Command, String> {
    match head {
        "help" => Ok(Command::Help),
        "clear" => Ok(Command::Clear),
        "exit" => Ok(Command::Exit),
        "addition" => Ok(Command::Addition),
        "subtraction" => Ok(Command::Subtraction),
        "multiplication" => Ok(Command::Multiplication),
        "real" => Ok(Command::Real),
        "imaginary" => Ok(Command::Imaginary),
        "power" => match tail.next() {
            Some(s) => {
                match s.parse::<f64>() {
                    Ok(n) => Ok(Command::Power(n)),
                    Err(_) => Err("Expecting a number".into())
                }
            },
            None => Err("Expecting a number".into())
        },
        "root" => match tail.next() {
            Some(s) => {
                match s.parse::<f64>() {
                    Ok(n) => Ok(Command::Root(n)),
                    Err(_) => Err("Expecting a number".into())
                }
            },
            None => Err("Expecting a number".into())
        },
        s => {
            let mut v = vec![s];
            for t in tail {
                v.push(t);
            }
            
            match parse_from_string(v.join(" ")) {
                Ok(cplx) => Ok(Command::Number(cplx)),
                Err(unk) => Err(format!("Unknown command: {}", unk))
            }
        }
    }
}

pub fn read_command() -> Result<Command, String> {
    print!(">>> ");
    {
        let _ = stdout().flush();
    }
    let input_str: String = read!("{}\n");
    let mut iter = input_str.split_whitespace();
    match iter.next() {
        None => Err("Please enter a command".into()),
        Some(s) => parse_command(s, iter),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{self, Read};
    use std::ops::Try;
    
    #[test]
    fn test_parse_help_command() {
        let input: String = "help".to_owned();
        let expected: Result<Command, String> = Ok(Command::Help);

        let mut iter = input.split_whitespace();
        let output = parse_command(iter.next().unwrap(), iter);
        
        assert_eq!(expected, output);
    }

    #[test]
    fn test_parse_clear_command() {
        let input: String = "clear".to_owned();
        let expected: Result<Command, String> = Ok(Command::Clear);

        let mut iter = input.split_whitespace();
        let output = parse_command(iter.next().unwrap(), iter);
        
        assert_eq!(expected, output);
    }

    #[test]
    fn test_parse_exit_command() {
        let input: String = "exit".to_owned();
        let expected: Result<Command, String> = Ok(Command::Exit);

        let mut iter = input.split_whitespace();
        let output = parse_command(iter.next().unwrap(), iter);
        
        assert_eq!(expected, output);
    }

    #[test]
    fn test_parse_addition_command() {
        let input: String = "addition".to_owned();
        let expected: Result<Command, String> = Ok(Command::Addition);

        let mut iter = input.split_whitespace();
        let output = parse_command(iter.next().unwrap(), iter);
        
        assert_eq!(expected, output);
    }

    #[test]
    fn test_parse_subtraction_command() {
        let input: String = "subtraction".to_owned();
        let expected: Result<Command, String> = Ok(Command::Subtraction);

        let mut iter = input.split_whitespace();
        let output = parse_command(iter.next().unwrap(), iter);
        
        assert_eq!(expected, output);
    }

    #[test]
    fn test_parse_multiplication_command() {
        let input: String = "multiplication".to_owned();
        let expected: Result<Command, String> = Ok(Command::Multiplication);

        let mut iter = input.split_whitespace();
        let output = parse_command(iter.next().unwrap(), iter);
        
        assert_eq!(expected, output);
    }

    #[test]
    fn test_parse_real_command() {
        let input: String = "real".to_owned();
        let expected: Result<Command, String> = Ok(Command::Real);

        let mut iter = input.split_whitespace();
        let output = parse_command(iter.next().unwrap(), iter);
        
        assert_eq!(expected, output);
    }

    #[test]
    fn test_parse_imaginary_command() {
        let input: String = "imaginary".to_owned();
        let expected: Result<Command, String> = Ok(Command::Imaginary);

        let mut iter = input.split_whitespace();
        let output = parse_command(iter.next().unwrap(), iter);
        
        assert_eq!(expected, output);
    }

    #[test]
    fn test_parse_power_command() {
        let input: String = "power 3".to_owned();
        let expected: Result<Command, String> = Ok(Command::Power(3.0));

        let mut iter = input.split_whitespace();
        let output = parse_command(iter.next().unwrap(), iter);
        
        assert_eq!(expected, output);
    }

    #[test]
    fn test_parse_root_command() {
        let input: String = "root 2".to_owned();
        let expected: Result<Command, String> = Ok(Command::Root(2.0));

        let mut iter = input.split_whitespace();
        let output = parse_command(iter.next().unwrap(), iter);
        
        assert_eq!(expected, output);
    }

    #[test]
    fn test_parse_an_unknown_command() {
        let input: String = "unknown".to_owned();
        let expected: Result<Command, String> = Err("Unknown command: unknown".into());

        let mut iter = input.split_whitespace();
        let output = parse_command(iter.next().unwrap(), iter);
        
        assert_eq!(expected, output);
    }

    #[test]
    fn test_parse_a_complex_number() {
        let input: String = "1 +1j".to_owned();
        let expected = Ok(Command::Number(Complex::new(1.0,1.0)));

        let mut iter = input.split_whitespace();
        let output = parse_command(iter.next().unwrap(), iter);

        assert_eq!(expected, output);
    }

    #[test]
    fn test_read_command_from_stdin() {
        let mut input: String = "help".to_owned();
        let expected = Ok(Command::Help);

        let output = read_command();

        assert_eq!(expected, output);
    }
}
