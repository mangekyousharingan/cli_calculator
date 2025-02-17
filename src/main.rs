use std::{collections::HashSet, io};


fn main() {

    let _supported_operations = HashSet::from(["+", "-", "*", ":"]);
    println!("Simple calculator!\n");
    println!("Supported operations: {:?}\n", _supported_operations);

    let number_1_input = get_user_number_input("enter number");

    let operation = get_user_operation_input(format!("enter operation ({:?}):", _supported_operations));

    if !_supported_operations.contains(operation.as_str()){
        print!("Unsupported operation!");
        return;
    }

    let number_2_input = get_user_number_input("enter another number:");

    let result = match operation.as_str() {
        "+" => number_1_input + number_2_input,
        "-" => number_1_input - number_2_input,
        "*" => number_1_input * number_2_input,
        ":" => number_1_input / number_2_input,
        _ => {
            println!("Unknown operation: {operation}");
            return;
        }
    };

    println!("Result: {result}");

}

fn get_user_number_input(prompt: &str) -> i32 {
    let mut input = String::new();

    println!("{}:", prompt);

    io::stdin().read_line(&mut input).expect("Faild to read input!");

    let number: i32 = input.trim().parse().expect("Not a valid integer!");

    return number;
}

fn get_user_operation_input(prompt: String) -> String {
    let mut input = String::new();

    println!("{}", prompt);

    io::stdin().read_line(&mut input).expect("Failed to read operation input!");

    return input.trim().to_string();
}