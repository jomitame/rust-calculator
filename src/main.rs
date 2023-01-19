use regex::Regex;

fn do_operation (regx: Regex, mut expression: String, operation: &str) -> String {
    if operation.is_empty() {
        return "".to_string();
    }

    // loop for more than two numbers
    loop {
        // Operations
        let caps = regx.captures(expression.as_str());

        if caps.is_none() {
            break;
        }
        let caps = caps.unwrap();

        let cap_expression = caps.get(0).unwrap().as_str();
        let firts_number: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let second_number: i32 = caps.get(2).unwrap().as_str().parse().unwrap();

        let result = match operation {
            "+" => firts_number + second_number,
            "*" => firts_number * second_number,
            _ => 0,
        };

        expression = expression.replace(cap_expression, &result.to_string());
    }
    expression
}

fn main() {
    // Regex
    let regex_for_add: Regex = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();
    let regex_for_mult: Regex = Regex::new(r"(\d+)\s?\*\s?(\d+)").unwrap();

    // User Data
    println!("Please introduce your expresion: ");
    let mut result: String = String::new();
    std::io::stdin().read_line(&mut result).unwrap();

    // Multiply
    result = do_operation(regex_for_mult, result, "*");
    // Sum
    result = do_operation(regex_for_add, result, "+");
    

    // Results
    println!("Result: {}", result);
}
