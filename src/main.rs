use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Wordle in your terminal!");
    loop {
        println!("Enter your guess!");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Faulty input.");
        let resp =
            reqwest::blocking::get(format!("https://v1.wordle.k2bd.dev/daily?guess={}", guess))?
                .text()?;
        println!("{:#?}", resp);
    }
    Ok(())
}
