use std::io;
use rust_input; // I wrote my own library to make things shorter and reusable.
// I will try to write a basic calculator with selector. Good luck to me.
fn main() {
    loop {
        println!("Enter your desired option: \n1 - Summary\n2 - Subtraction\n3 - Multiplication\n4 - Dividing\n5 - Exponent\n6 - Exit");
        let mut user_input = String::new();

        io::stdin().read_line(&mut user_input).expect("You wrote so wrong thing man...");
        let user_input = user_input.trim();
        match user_input {
            "1" => {
                let num1 = rust_input::int_input("Enter your first number:");
                let num2 = rust_input::int_input("Enter your second number:");
                let result = summary(num1, num2);
                println!("{}", result);
            },
            "2" => {
                let num1 = rust_input::int_input("Enter your first number:");
                let num2 = rust_input::int_input("Enter your second number:");
                let result = subtraction(num1, num2);
                println!("{}", result);
            },
            "3" => {
                let num1 = rust_input::int_input("Enter your first number:");
                let num2 = rust_input::int_input("Enter your second number:");
                let result = multiplication(num1, num2);
                println!("{}", result);
            },
            "4" => {
                let num1 = rust_input::int_input("Enter your first number:");
                let num2 = rust_input::int_input("Enter your second number:");
                let result = dividing(num1, num2);
                println!("{}", result);
            },
            "5" => {
                let num1 = rust_input::int_input("Enter your exponent's base:");
                let num2 = rust_input::int_input("Enter your exponent's power:");
                let result = exponent(num1, num2);
                println!("{}", result)
            },
            "6" => { println!("See you next time!"); break; },
            _ => println!("Your entered number can't be found on list.")
        }
    }

    fn summary(num1: i32, num2: i32) -> i32 {
        return num1+num2;
    }

    fn subtraction(num1: i32, num2: i32) -> i32 {
        return num1-num2;
    }

    fn multiplication(num1: i32, num2: i32) -> i32 {
        return num1*num2;
    }

    fn dividing(num1: i32, num2: i32) -> i32 {
        return num1/num2
    }

    fn exponent(base: i32, power: i32) -> i32 {
        let mut pwr = power - 1;
        let mut result = base;
        while pwr > 0 {
            result = result * base;
            pwr = pwr - 1;
        }

        return result;
    }
}