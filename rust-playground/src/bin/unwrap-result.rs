use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let res = try_to_parse()?;
    println!("{:?}", res);
    Ok(())
}

fn try_to_parse() -> Result<i32, ParseIntError> {
    let x: i32 = "123".parse()?; // x = 123
    let y: i32 = "24a".parse()?; // returns an Err() immediately
    Ok(x + y)                    // Doesn't run.
}

