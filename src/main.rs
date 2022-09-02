#[macro_use]
extern crate text_io;

fn main() {
    /*
        Inputting an operator
    */
    println!("Please input an operator:");
    let operator: String = read!();
    /*
        Inputting first / second number
    */
    println!("Input first number:");
    let num1: f64 = read!();

    println!("Input second number:");
    let num2: f64 = read!();

    /*
        Calculator logic
    */

    if operator == "+" {
        let num3 = num1 + num2;
        println!("{}", num3);
        }

    if operator == "-" {
        let num3 = num1 - num2;
        println!("{}", num3);
    }

    if operator == "*" {
        let num3 = num1 * num2;
        println!("{}", num3);
    }

    if operator == "/" {
        let num3 = num1 / num2;
        println!("{}", num3);
    }
    }

