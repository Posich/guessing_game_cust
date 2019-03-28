use crate::parseargs;
use crate::guessing::Reply::*;

pub struct Bounds {
    lower: i64,
    upper: i64,
    init: (i64, i64),
}

pub enum Reply {
    TooHigh(i64),
    TooLow(i64),
    Correct,
}

impl Bounds {

    pub fn new(values: Vec<i64>) -> Result<Bounds, &'static str> {
        if values.len() < 2 {
            return Err("Need two numbers");
        }

        if values[0] == values[1] {
            return Err("Bounds equal");
        }

        let mut bounds = Bounds { lower: 0, upper: 0, init: (0, 0), };

        if values[0] < values[1] {
            let lower = values[0];
            let upper = values[1];
            bounds.init_bounds(lower, upper);
        } else {
            let lower = values[1];
            let upper = values[0];
            bounds.init_bounds(lower, upper);
        }

        Ok(bounds)
    }

    fn init_bounds(&mut self, lower: i64, upper: i64) {
        self.lower = lower;
        self.upper = upper;
        self.init = (lower, upper);
    }
    
    pub fn adj_bounds(&mut self, reply: Reply) -> Result<(), &'static str> {
        match reply {
            TooHigh(val) => self.upper = val,
            TooLow(val) => self.lower = val,
            Correct => (),
        };

        if self.lower > self.upper {
            return Err("Bounds overlap.");
        } else {
            Ok(())
        }
    }

    pub fn guess(&mut self) -> i64 {
        (self.upper - self.lower) / 2
    }
}
