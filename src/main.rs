use std::{io, num::ParseIntError};
// I will try to write a basic calculator with selector. Good luck to me.
fn main() {
    loop {
        println!("Enter your desired option: \n1 - Summary\n2 - Subtraction\n3 - Multiplication\n4 - Dividing");
        let mut user_input = String::new();

        io::stdin().read_line(&mut user_input).expect("You wrote so wrong thing man...");
        let user_input = user_input.trim();
        match user_input {
            "1" => {
                let mut num1 = String::new();
                let mut num2 = String::new();

                println!("Enter your first number.");
                io::stdin().read_line(&mut num1).expect("You wrote so wrong thing man...");
                let num1: Result<i32, ParseIntError> = num1.trim().parse();
                println!("Enter your second number.");
                io::stdin().read_line(&mut num2).expect("You wrote so wrong thing man...");
                let num2: Result<i32, ParseIntError> = num2.trim().parse();

                match (num1, num2) {
                    (Ok(n1), Ok(n2)) => {
                        let result = summary(n1, n2);
                        println!("Result: {}", result)
                    }
                    _ => println!("We encountered an error.")
                }
            },
            "2" => {
                let mut num1 = String::new();
                let mut num2 = String::new();

                println!("Enter your first number.");
                io::stdin().read_line(&mut num1).expect("You wrote so wrong thing man...");
                let num1: Result<i32, ParseIntError> = num1.trim().parse();
                println!("Enter your second number.");
                io::stdin().read_line(&mut num2).expect("You wrote so wrong thing man...");
                let num2: Result<i32, ParseIntError> = num2.trim().parse();

                match (num1, num2) {
                    (Ok(n1), Ok(n2)) => {
                        let result = subtraction(n1, n2);
                        println!("Result: {}", result)
                    }
                    _ => println!("We encountered an error.")
                }
            },
            "3" => {
                let mut num1 = String::new();
                let mut num2 = String::new();

                println!("Enter your first number.");
                io::stdin().read_line(&mut num1).expect("You wrote so wrong thing man...");
                let num1: Result<i32, ParseIntError> = num1.trim().parse();
                println!("Enter your second number.");
                io::stdin().read_line(&mut num2).expect("You wrote so wrong thing man...");
                let num2: Result<i32, ParseIntError> = num2.trim().parse();

                match (num1, num2) {
                    (Ok(n1), Ok(n2)) => {
                        let result = multiplication(n1, n2);
                        println!("Result: {}", result)
                    }
                    _ => println!("We encountered an error.")
                }
            },
            "4" => {
                let mut num1 = String::new();
                let mut num2 = String::new();

                println!("Enter your first number. (Numerator)");
                io::stdin().read_line(&mut num1).expect("You wrote so wrong thing man...");
                let num1: Result<i32, ParseIntError> = num1.trim().parse();
                println!("Enter your second number. (Denominator)");
                io::stdin().read_line(&mut num2).expect("You wrote so wrong thing man...");
                let num2: Result<i32, ParseIntError> = num2.trim().parse();

                match (num1, num2) {
                    (Ok(n1), Ok(n2)) => {
                        let result = dividing(n1, n2);
                        println!("Result: {}", result)
                    }
                    _ => println!("We encountered an error.")
                }
            },
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
}