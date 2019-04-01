use std::io;
use std::io::Write;
use std::ops::Deref;
use std::fmt;
use std::error::Error;
use guessing_game_cust::guessing::*;

type ProgResult<T> = Result<T, ProgError>;

fn main() -> ProgResult<()> {

    let mut bounds = set_bounds()?;

    println!("{:?}", bounds);

    game_start(&mut bounds)?;

    Ok(())
}

fn set_bounds() -> ProgResult<Bounds> {
    let mut stderr = io::stderr();
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut input = String::new();

    let mut bounds: Option<Bounds> = None;

    while bounds.is_none() {
        input.clear();
        stdout.write(b"Input two numbers:  ")?;
        stdout.flush()?;

        stdin.read_line(&mut input)?;

        bounds = match input.deref().to_bounds() {
            Ok(b) => Some(b),
            Err(e) => {
                writeln!(stderr, "{}", e.to_string())?;
                None
            },
        };
    }

    Ok(bounds.unwrap())
}

fn game_start(bounds: &mut Bounds) -> ProgResult<()> {
    let mut input = String::new();

    let mut stdout = io::stdout();
    let stdin = io::stdin();

    println!("Pick a number between {} and {}.  Don't tell me what it is, it's none of my business.", bounds.lower(), bounds.upper());
    stdout.write(b"Smash <Enter> when ready.")?;
    stdout.flush()?;

    stdin.read_line(&mut input)?;

    input.clear(); // Don't want any input, really.

    println!("\nThen let the games begin!\n");
    
    loop {
        input.clear();

        let guess = bounds.guess();
        let tries = bounds.tries();

        println!("[guess #{}]: {}", tries, guess);
        println!("How did I do?");

        let mut answer: Option<Reply> = None;

        while answer.is_none() {
            stdout.write(b"Pick: Too [H]igh, Too [L]ow, [C]orrect >")?;
            stdout.flush()?;

            stdin.read_line(&mut input)?;

            println!();

            answer = get_answer(&input, guess);

            input.clear();
        }

        if let Some(Reply::Correct) = answer {
            println!("I got it in {} tries.", tries);
            break;
        }

        bounds.adj_bounds(answer.unwrap())?;
    }

    Ok(())
}

fn get_answer(input: &String, guess: i64) -> Option<Reply> {
    let value = input.trim().to_lowercase().to_string();

    if value == "h" {
        return Some(Reply::TooHigh(guess));
    } else if value == "l" {
        return Some(Reply::TooLow(guess));
    } else if value == "c" {
        return Some(Reply::Correct);
    }

    None
}

#[derive(Debug)]
enum ProgErrKind {
    IOErr(io::Error),
    Bounds(BoundsError),
}

#[derive(Debug)]
struct ProgError {
    kind: ProgErrKind,
}

impl fmt::Display for ProgError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            ProgErrKind::Bounds(e) => write!(f, "Bounds Error: {}", e.to_string()),
            ProgErrKind::IOErr(e) => write!(f, "IO ERROR: {}", e.to_string()),
        }
    }
}

impl Error for ProgError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.kind {
            ProgErrKind::Bounds(err) => Some(err),
            ProgErrKind::IOErr(err) => Some(err),
        }
    }
}

impl From<io::Error> for ProgError {
    fn from(error: io::Error) -> Self {
        ProgError { kind: ProgErrKind::IOErr(error), }
    }
}

impl From<BoundsError> for ProgError {
    fn from(error: BoundsError) -> Self {
        ProgError { kind: ProgErrKind::Bounds(error), }
    }
}
