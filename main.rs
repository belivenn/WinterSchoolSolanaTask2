use std::io;

fn main() {
    let valor: f32 = loop {
        println!("Insert the first number:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Invalid");
        match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => println!("Invalid input. Please enter a valid number."),
        }
    };

    let oper: String = loop {
        println!("Which operation do you want to perform? (+) (-) (*) (/)? :");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Invalid symbol");
        let oper = input.trim().to_string();
        if ["+", "-", "*", "/"].contains(&oper.as_str()) {
            break oper;
        } else {
            println!("Invalid input. Please enter a valid symbol.");
        }
    };

    let valor1: f32 = loop {
        println!("Insert the second number:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Not Valid");
        match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => println!("Invalid input. Please enter a valid number."),
        }
    };

    let result = operation(&oper, valor, valor1);
    println!("The result is {}", result);
}

fn operation(operator: &str, value1: f32, value2: f32) -> f32 {
    match operator {
        "+" => value1 + value2,
        "-" => value1 - value2,
        "*" => value1 * value2,
        "/" => value1 / value2,
        _ => panic!("Invalid Operation"),
    }
}