use std::io;


fn main() {
    println!("Simple Calculator!\n");

    let number_1 = get_number_input("Please enter first number:");

    let operation = get_operation_input("Please enter operation:");

    let number_2 = get_number_input("Please enter second number:");

    let result = match operation.as_str() {
        "+" => number_1 + number_2,
        "-" => number_1 - number_2,
        "*" => number_1 * number_2,
        ":" => number_1 / number_2,
        _ => {
            println!("Invalid operation!");
            return;
        }
    };

    print!("Result: {result}\n");
}

fn get_number_input(prompt: &str) -> i32 {
    let number: i32 = loop {
        println!("{prompt}");

        let mut number_input = String::new();

        io::stdin().read_line(&mut number_input).expect("Failed to read line!");

        match number_input.trim().parse::<i32>() {
            Ok(num) => break num,
            _ => println!("Invalid number!")
        }
    };

    return number;
}

fn get_operation_input(prompt: &str) -> String {
    let valid_operations = ["+", "-", "*", ":"];

    let operation: String = loop {
        println!("{prompt}");
        println!("{:?}", valid_operations);

        let mut op = String::new();

        io::stdin().read_line(&mut op).expect("Failed to read line!");

        let op = op.trim();

        if valid_operations.contains(&op) {
            break op.to_string();
        } else {
            println!("Invalid operation!")
        }
    };
    
    return operation;
}
