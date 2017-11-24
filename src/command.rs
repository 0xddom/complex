use complex::Complex;
use std::str::SplitWhitespace;
use std::io::{Write, stdout};
use complex::parser::parse_from_string;

pub enum Command {
    Help,
    Exit,
    Clear,
    Addition,
    Subtraction,
    Multiplication,
    Real,
    Imaginary,
    Power(i32),
    Root(i32),
    Number(Complex),
}

fn parse_command(head: &str, tail: SplitWhitespace) -> Result<Command, String> {
    match head {
        "help" => Ok(Command::Help),
        "clear" => Ok(Command::Clear),
        "exit" => Ok(Command::Exit),
        "addition" => Ok(Command::Addition),
        "subtraction" => Ok(Command::Subtraction),
        "multiplication" => Ok(Command::Multiplication),
        "real" => Ok(Command::Real),
        "imaginary" => Ok(Command::Imaginary),
        "power" => Err("TODO".into()),
        "root" => Err("TODO".into()),
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
