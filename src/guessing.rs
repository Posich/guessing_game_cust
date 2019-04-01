use crate::parseargs;
use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct Bounds {
    lower: i64,
    upper: i64,
    initial_values: (i64, i64),
    tries: usize,
}

pub struct BoundsBuilder {
    lower: Option<i64>,
    upper: Option<i64>,
}

pub enum Reply {
    TooHigh(i64),
    TooLow(i64),
    Correct,
}

#[derive(Debug)]
pub struct BoundsError {
    kind: BoundsErrorKind,
}

impl fmt::Display for BoundsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            BoundsErrorKind::TooFew => write!(f, "{}", "Need two numbers"),
            BoundsErrorKind::BoundsEqual => write!(f, "{}", "Bounds equal"),
            BoundsErrorKind::Initialization => write!(f, "{}", "Bounds not initialized"),
            BoundsErrorKind::BoundsInvalid => write!(f, "{}", "Initialized with invalid bounds"),
            BoundsErrorKind::BoundsOverlap => write!(f, "{}", "Lower bound greater than upper"),
            BoundsErrorKind::ParseArgs(e) => write!(f, "{}", e.to_string()),
        }
    }
}

impl Error for BoundsError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.kind {
            BoundsErrorKind::ParseArgs(e) => return Some(e),
            _ => return None,
        };
    }
}

#[derive(Debug)]
pub enum BoundsErrorKind {
    TooFew,
    BoundsEqual,
    BoundsOverlap,
    BoundsInvalid,
    Initialization,
    ParseArgs(parseargs::ParseArgError),
}

impl From<parseargs::ParseArgError> for BoundsError {
    fn from(e: parseargs::ParseArgError) -> Self {
        BoundsError { kind: BoundsErrorKind::ParseArgs(e) }
    }
}

pub trait ToBounds {
    fn to_bounds(&self) -> Result<Bounds, BoundsError>;
}

impl ToBounds for &str {
    fn to_bounds(&self) -> Result<Bounds, BoundsError> {
        let args: Vec<i64> = parseargs::parse_args(2, &self)?;
        if args[0] == args[1] {
            return Err( BoundsError { kind: BoundsErrorKind::BoundsEqual } );
        }

        let ordered = order_neq_pair(args[0], args[1]);
        
        Bounds::build()
            .lower(ordered.0)
            .upper(ordered.1)
            .create()
    }
}

fn order_neq_pair(a: i64, b: i64) -> (i64, i64) {
    if a < b { (a, b) } else { (b, a) }
}

impl Bounds {
    fn is_valid(&self) -> bool {
        self.lower < self.upper
    }

    fn init(lower: i64, upper: i64) -> Self {
        Bounds { lower: lower, upper: upper, initial_values: (lower, upper), tries: 0, }
    }

    pub fn lower(&self) -> i64 {
        self.lower
    }

    pub fn upper(&self) -> i64 {
        self.upper
    }

    pub fn init_vals(&self) -> (i64, i64) {
        self.initial_values
    }

    pub fn guess(&mut self) -> i64 {
        self.tries += 1;

        ((self.upper - self.lower) / 2) + self.lower
    }

    pub fn tries(&self) -> usize {
        self.tries
    }

    pub fn adj_bounds(&mut self, re: Reply) -> Result<(), BoundsError> {
        match re {
            Reply::TooHigh(val) => self.upper = val - 1,
            Reply::TooLow(val) => self.lower = val + 1,
            Reply::Correct => panic!("Then why are we still playing?"),
        };

        if self.lower <= self.upper {
            return Ok(());
        } else {
            return Err( BoundsError { kind: BoundsErrorKind::BoundsOverlap } );
        }
    }

    pub fn build() -> BoundsBuilder {
        BoundsBuilder::new()
    }
}

impl BoundsBuilder {

    fn new() -> Self {
        BoundsBuilder { lower: None, upper: None }
    }

    pub fn lower(mut self, val: i64) -> Self {
        self.lower = Some(val);
        self
    }

    pub fn upper(mut self, val: i64) -> Self {
        self.upper = Some(val);
        self
    }

    pub fn create(self) -> Result<Bounds, BoundsError> {
        if self.lower == None || self.upper == None {
            return Err( BoundsError { kind: BoundsErrorKind::Initialization } );
        }

        let retval = Bounds::init(self.lower.unwrap(), self.upper.unwrap());

        if retval.is_valid() {
            return Ok(retval);
        } else {
            return Err( BoundsError { kind: BoundsErrorKind::BoundsInvalid } );
        }
    }
}
