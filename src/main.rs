use std::io::{self, Write};
use rand::RngExt;

const MIN_NUMBER_SIZE: i32 = 0;
const MAX_NUMBER_SIZE: i32 = 12;

fn main() {
    let mut rng = rand::rng();
    let mut correctly_answered: u16 = 0;
    let question_amount: u16 = 16;
    let symbols: [char; 3] = ['+', '-', '*'];

    println!("taking quiz of {question_amount} questions...\n\n");

    for i in 0..question_amount {
        print!("{i}/{question_amount}: ");

        let answer: (bool, i32) = question(rng.random_range(MIN_NUMBER_SIZE..MAX_NUMBER_SIZE), rng.random_range(MIN_NUMBER_SIZE..MAX_NUMBER_SIZE), symbols[rng.random_range(0..symbols.len())]);

        if answer.0 {
            println!("well done you answered correctly!");

            correctly_answered += 1;
        } else {
            println!("unfortunately you answered the question incorrectly, the right answer was {}. give it another shot!", answer.1);
        }
    }

    println!("\nwell done you completed the test with a score of {correctly_answered}/{question_amount}");
}

fn question(a: i32, b: i32, symbol: char) -> (bool, i32) {
    let mut input = String::new();
    let mut answer = 0;

    print!("{a} {symbol} {b} = ");

    let _ = io::stdout().flush();
    io::stdin().read_line(&mut input).expect("failed to read stdin");

    let n: i32 = input.trim().parse().expect("please input a valid base 10 value");

    match symbol {
        '+' => answer = a + b,
        '-' => answer = a - b,
        '*' => answer = a * b,
        _ => println!("error: {symbol} is an invalid symbol")
    };

    (n == answer, answer)
}
