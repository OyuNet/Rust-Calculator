use std::io;
use rust_input; // I wrote my own library to make things shorter and reusable.

use rust_calc;
// I will try to write a basic calculator with selector. Good luck to me.
fn main() {
    loop {
        println!("Enter your desired option: \n1 - Summary\n2 - Subtraction\n3 - Multiplication\n4 - Dividing\n5 - Exponent\n6 - Square Root\n7 - Cube Root\n8 - Root\n9 - Exit");
        let mut user_input = String::new();

        io::stdin().read_line(&mut user_input).expect("You wrote so wrong thing man...");
        let user_input = user_input.trim();
        match user_input {
            "1" => {
                let num1 = rust_input::int_input("Enter your first number:");
                let num2 = rust_input::int_input("Enter your second number:");
                let result = rust_calc::summary(num1, num2);
                println!("{}", result);
            },
            "2" => {
                let num1 = rust_input::int_input("Enter your first number:");
                let num2 = rust_input::int_input("Enter your second number:");
                let result = rust_calc::subtraction(num1, num2);
                println!("{}", result);
            },
            "3" => {
                let num1 = rust_input::int_input("Enter your first number:");
                let num2 = rust_input::int_input("Enter your second number:");
                let result = rust_calc::multiplication(num1, num2);
                println!("{}", result);
            },
            "4" => {
                let num1 = rust_input::int_input("Enter your first number:");
                let num2 = rust_input::int_input("Enter your second number:");
                let result = rust_calc::dividing(num1 as f64, num2 as f64);
                println!("{}", result);
            },
            "5" => {
                let num1 = rust_input::int_input("Enter your exponent's base:");
                let num2 = rust_input::int_input("Enter your exponent's power:");
                let result = rust_calc::exponent(num1, num2);
                println!("{}", result)
            },
            "6" => {
                let num1 = rust_input::int_input("Enter your desired value:");
                let result = rust_calc::sqrt(num1);
                println!("{}", result)
            },
            "7" => {
                let num1 = rust_input::int_input("Enter your desired value:");
                let result = rust_calc::cbrt(num1);
                println!("{}", result)
            },
            "8" => {
                let num1 = rust_input::int_input("Enter your desired value:");
                let num2 = rust_input::int_input("Enter your root level:");
                let result = rust_calc::root(num1 as f64, num2 as f64);
                println!("{}", result)
            },
            "9" => { println!("See you next time!"); break; },
            _ => println!("Your entered number can't be found on list.")
        }
    }
}