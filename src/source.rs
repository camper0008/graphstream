use std::io::{BufRead, IsTerminal, Write};

pub trait Source {
    fn next(&mut self) -> Option<Vec<f64>>;
}

pub struct Stdin;

impl Stdin {
    fn value_from_stdin(&mut self) -> Result<Vec<f64>, String> {
        let mut buffer = String::new();
        let bytes_taken = std::io::stdin()
            .lock()
            .read_line(&mut buffer)
            .map_err(|err| format!("io error while reading from stdin: {err}"))?;
        if bytes_taken == 0 {
            std::thread::park();
            return Err(String::from("reached EOF"));
        }
        buffer
            .trim()
            .split(';')
            .map(|x| {
                x.parse()
                    .map_err(|_| format!("'{x}' is not a valid number"))
            })
            .collect::<Result<_, _>>()
    }
}

impl Source for Stdin {
    fn next(&mut self) -> Option<Vec<f64>> {
        let is_tty = std::io::stdin().is_terminal();
        if is_tty {
            println!("enter value:");
            print!("> ");
            std::io::stdout().lock().flush().unwrap();
        }
        match self.value_from_stdin() {
            Ok(v) => Some(v),
            Err(err) if is_tty => {
                println!("error: {err}");
                None
            }
            Err(_) => None,
        }
    }
}
