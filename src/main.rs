use regex::Regex;

fn main() {
    // Regex
    let regex_for_add: Regex = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();

    // User Data
    println!("Please introduce your expresion: ");
    let mut expression: String = String::new();
    std::io::stdin().read_line(&mut expression).unwrap();

    // loop for more thar two numbers
    // Adition
    loop {
        // Operations
        let caps = regex_for_add.captures(expression.as_str());

        if caps.is_none() {
            break;
        }

        let caps = caps.unwrap();

        let cap_expression: &str = caps.get(0).unwrap().as_str();
        let firts_number: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let second_number: i32 = caps.get(2).unwrap().as_str().parse().unwrap();

        let addition: i32 = firts_number + second_number;

        expression = expression.replace(cap_expression, &addition.to_string());
        
    }

    // Results
    println!("Adition: {}", expression);
}
