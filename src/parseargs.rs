use std::str::FromStr;
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ParseArgError<T: Error> {
    kind: ArgErrorKind<T>,
}

impl<T: Error> fmt::Display for ParseArgError<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "parse_args: {}", self.kind.to_string())
    }
}

impl<T: Error> Error for ParseArgError<T> {
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

#[derive(Debug, Clone)]
pub enum ArgErrorKind<T: Error> {
    ArgAmount((usize, usize,)),
    ExternErr((String, T,)),
}

impl<T: Error> fmt::Display for ArgErrorKind<T> {
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

pub fn parse_args<T: FromStr>(argc: usize, buff: &str) -> Result<Vec<T>, ParseArgError<T::Err>> 
where <T as std::str::FromStr>::Err: std::fmt::Debug, <T as std::str::FromStr>::Err: Error {
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
                e)) });
            },
        });
    }

    Ok(result)
}
