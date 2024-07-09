use std::io::{BufRead, Write};

use crate::value::Value;

pub trait Source {
    fn next(&mut self) -> Option<Value>;
}

pub struct Stdin;

impl Stdin {
    fn value_from_stdin() -> Result<Value, &'static str> {
        let mut buffer = String::new();
        std::io::stdin()
            .lock()
            .read_line(&mut buffer)
            .map_err(|_| "io error while reading from stdin")?;
        buffer.trim().parse().map_err(|_| "invalid value given")
    }
}

impl Source for Stdin {
    fn next(&mut self) -> Option<Value> {
        println!("enter value:");
        print!("> ");
        std::io::stdout().lock().flush().unwrap();
        match Self::value_from_stdin() {
            Ok(v) => Some(v),
            Err(err) => {
                println!("error: {err}");
                return None;
            }
        }
    }
}
