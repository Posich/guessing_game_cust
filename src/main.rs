use std::io;
use std::io::Write;
use std::str::FromStr;
use guessing_game_cust::parseargs;

fn main() {

    let mut input = String::new();
    let mut nums: Vec<i64>;

    loop {
        print!("Input two numbers: ");
        io::stdout().flush().unwrap_or_else(|e| { println!("IO Error: {}", e); });

        io::stdin().read_line(&mut input).unwrap_or_else(|e| { println!("IO Error: {}", e); 0 as usize });

        nums = match parseargs::parse_args(2, &input) {
            Ok(numbers) => numbers,
            Err(e) => {
                println!("Error: {}", e);
                continue;
            },
        };

        break;
    }

    println!("Numbers: {:?}", nums);
}

fn get_numbers() -> Result<(i64, i64), String> {
    print!("Input two numbers: ");
    io::stdout().flush().expect("IO Error");

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("IO Error");

    let unparsed_nums: Vec<&str> = input.split_whitespace().collect();

    if unparsed_nums.len() != 2 {
        return Err(format!("Two arguments accepted, no more, no less.  You gave {}.", unparsed_nums.len()));
    }

    let mut retval: (i64, i64) = (0, 0);

    retval.0 = match i64::from_str(unparsed_nums[0]) {
        Ok(num) => num,
        Err(e) => {
            return Err(format!("Could not parse {}.  Error: {:?}", unparsed_nums[0], e));
        },
    };

    retval.1 = match i64::from_str(unparsed_nums[1]) {
        Ok(num) => num,
        Err(e) => {
            return Err(format!("Could not parse {}.  Error: {:?}", unparsed_nums[1], e));
        },
    };

    Ok(retval)
}
