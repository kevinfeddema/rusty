use std::io;
use std::fs::File;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    println!("Enter cocktail name...");

    let mut user_input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut user_input)?;

    let mut file = File::create("cocktails.txt")?;
    file.write(user_input.as_bytes())?;

    println!("Written: {}", user_input);
    Ok(())
}

