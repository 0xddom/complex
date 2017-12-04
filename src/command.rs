use complex::Complex;
use std::str::SplitWhitespace;
use std::io::{Write, stdout};
use complex::parser::parse_from_string;

#[derive(Debug, PartialEq)]
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
    
    #[test]
    fn test_parse_help_command__047() {
        let input: String = "help".to_owned();
        let expected: Result<Command, String> = Ok(Command::Help);

        let mut iter = input.split_whitespace();
        let output = parse_command(iter.next().unwrap(), iter);
        
        assert_eq!(expected, output);
    }

    #[test]
    fn test_parse_clear_command__048() {
        let input: String = "clear".to_owned();
        let expected: Result<Command, String> = Ok(Command::Clear);

        let mut iter = input.split_whitespace();
        let output = parse_command(iter.next().unwrap(), iter);
        
        assert_eq!(expected, output);
    }

    #[test]
    fn test_parse_exit_command__049() {
        let input: String = "exit".to_owned();
        let expected: Result<Command, String> = Ok(Command::Exit);

        let mut iter = input.split_whitespace();
        let output = parse_command(iter.next().unwrap(), iter);
        
        assert_eq!(expected, output);
    }

    #[test]
    fn test_parse_addition_command__050() {
        let input: String = "addition".to_owned();
        let expected: Result<Command, String> = Ok(Command::Addition);

        let mut iter = input.split_whitespace();
        let output = parse_command(iter.next().unwrap(), iter);
        
        assert_eq!(expected, output);
    }

    #[test]
    fn test_parse_subtraction_command__051() {
        let input: String = "subtraction".to_owned();
        let expected: Result<Command, String> = Ok(Command::Subtraction);

        let mut iter = input.split_whitespace();
        let output = parse_command(iter.next().unwrap(), iter);
        
        assert_eq!(expected, output);
    }

    #[test]
    fn test_parse_multiplication_command__052() {
        let input: String = "multiplication".to_owned();
        let expected: Result<Command, String> = Ok(Command::Multiplication);

        let mut iter = input.split_whitespace();
        let output = parse_command(iter.next().unwrap(), iter);
        
        assert_eq!(expected, output);
    }

    #[test]
    fn test_parse_real_command__053() {
        let input: String = "real".to_owned();
        let expected: Result<Command, String> = Ok(Command::Real);

        let mut iter = input.split_whitespace();
        let output = parse_command(iter.next().unwrap(), iter);
        
        assert_eq!(expected, output);
    }

    #[test]
    fn test_parse_imaginary_command__054() {
        let input: String = "imaginary".to_owned();
        let expected: Result<Command, String> = Ok(Command::Imaginary);

        let mut iter = input.split_whitespace();
        let output = parse_command(iter.next().unwrap(), iter);
        
        assert_eq!(expected, output);
    }

    #[test]
    fn test_parse_power_command__055() {
        let input: String = "power 3".to_owned();
        let expected: Result<Command, String> = Ok(Command::Power(3.0));

        let mut iter = input.split_whitespace();
        let output = parse_command(iter.next().unwrap(), iter);
        
        assert_eq!(expected, output);
    }

    #[test]
    fn test_parse_root_command__056() {
        let input: String = "root 2".to_owned();
        let expected: Result<Command, String> = Ok(Command::Root(2.0));

        let mut iter = input.split_whitespace();
        let output = parse_command(iter.next().unwrap(), iter);
        
        assert_eq!(expected, output);
    }

    #[test]
    fn test_parse_bad_power_command__057__058() {
        {
            let input: String = "power".to_owned();
            let expected: Result<Command, String> = Err("Expecting a number".into());

            let mut iter = input.split_whitespace();
            let output = parse_command(iter.next().unwrap(), iter);

            assert_eq!(expected, output);
        }
        {
            let input: String = "power bad".to_owned();
            let expected: Result<Command, String> = Err("Expecting a number".into());

            let mut iter = input.split_whitespace();
            let output = parse_command(iter.next().unwrap(), iter);

            assert_eq!(expected, output);
        }
    }

    #[test]
    fn test_parse_bad_root_command__059__060() {
        {
            let input: String = "root".to_owned();
            let expected: Result<Command, String> = Err("Expecting a number".into());

            let mut iter = input.split_whitespace();
            let output = parse_command(iter.next().unwrap(), iter);

            assert_eq!(expected, output);
        }
        {
            let input: String = "root bad".to_owned();
            let expected: Result<Command, String> = Err("Expecting a number".into());

            let mut iter = input.split_whitespace();
            let output = parse_command(iter.next().unwrap(), iter);

            assert_eq!(expected, output);
        }
    }

    #[test]
    fn test_parse_an_unknown_command__061() {
        let input: String = "unknown".to_owned();
        let expected: Result<Command, String> = Err("Unknown command: unknown".into());

        let mut iter = input.split_whitespace();
        let output = parse_command(iter.next().unwrap(), iter);
        
        assert_eq!(expected, output);
    }

    #[test]
    fn test_parse_a_complex_number__062() {
        let input: String = "1 +1j".to_owned();
        let expected = Ok(Command::Number(Complex::new(1.0,1.0)));

        let mut iter = input.split_whitespace();
        let output = parse_command(iter.next().unwrap(), iter);

        assert_eq!(expected, output);
    }

    #[test]
    fn test_read_command_from_stdin__063() {
        let expected = Ok(Command::Help);

        let output = read_command();

        assert_eq!(expected, output);
    }
}
