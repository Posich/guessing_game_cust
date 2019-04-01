use std::str::FromStr;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ParseArgError {
    kind: ArgErrorKind,
}

impl fmt::Display for ParseArgError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "parse_args: {}", self.kind.to_string())
    }
}

impl Error for ParseArgError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.kind {
            ArgErrorKind::ExternErr((_, e)) => {
                Some(&e)
            },
            _ => None,
        };

        None
    }
}

#[derive(Debug)]
pub enum ArgErrorKind {
    ArgAmount((usize, usize,)),
    ExternErr((String, Box<Error>,)),
}

impl fmt::Display for ArgErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ArgErrorKind::ArgAmount((expect, actual)) => {
                write!(f, "Expected arguments: {}, Actual amount: {}", expect, actual)
            },
            ArgErrorKind::ExternErr((input, e)) => {
                write!(f, "Error parsing \"{}\": {}", &input, e)
            },
        }
    }
}

pub fn parse_args<T: FromStr>(argc: usize, buff: &str) -> Result<Vec<T>, ParseArgError> 
where <T as std::str::FromStr>::Err: std::fmt::Debug, <T as std::str::FromStr>::Err: Error + 'static {
    let unparsed_items: Vec<&str> = buff.split_whitespace().collect();

    if unparsed_items.len() != argc {
        return Err(ParseArgError { kind: ArgErrorKind::ArgAmount((argc, unparsed_items.len())) });
    }

    let mut result: Vec<T> = Vec::new();

    for item in unparsed_items {
        result.push(match T::from_str(item) {
            Ok(val) => val,
            Err(e) => {
                return Err(ParseArgError { kind: ArgErrorKind::ExternErr((String::from_str(item).expect("String conversion panic!"),
                e.into())) });
            },
        });
    }

    Ok(result)
}
