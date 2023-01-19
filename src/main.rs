use regex::Regex;

fn main() {
    // Regex
    let regex_for_add: Regex = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();
    let regex_for_mult: Regex = Regex::new(r"(\d+)\s?\*\s?(\d+)").unwrap();

    // User Data
    println!("Please introduce your expresion: ");
    let mut result: String = String::new();
    std::io::stdin().read_line(&mut result).unwrap();

    // loop for more thar two numbers
    // Multiply
    loop {
        // Operations
        let caps = regex_for_mult.captures(result.as_str());

        if caps.is_none() {
            break;
        }

        let caps = caps.unwrap();

        let cap_expression: &str = caps.get(0).unwrap().as_str();
        let firts_number: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let second_number: i32 = caps.get(2).unwrap().as_str().parse().unwrap();

        let multiply: i32 = firts_number * second_number;

        result = result.replace(cap_expression, &multiply.to_string());
        
    }
    // Adition
    loop {
        // Operations
        let caps = regex_for_add.captures(result.as_str());

        if caps.is_none() {
            break;
        }

        let caps = caps.unwrap();

        let cap_expression: &str = caps.get(0).unwrap().as_str();
        let firts_number: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let second_number: i32 = caps.get(2).unwrap().as_str().parse().unwrap();

        let addition: i32 = firts_number + second_number;

        result = result.replace(cap_expression, &addition.to_string());
        
    }

    // Results
    println!("Result: {}", result);
}
