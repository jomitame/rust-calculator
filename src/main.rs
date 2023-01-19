use regex::Regex;

fn main() {
    println!("Hello, world!");

    // Regex
    let regex_for_add: Regex = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();

    // User Data
    println!("Please introduce your expresion: ");
    let mut expression: String = String::new();
    std::io::stdin().read_line(&mut expression).unwrap();

    // Operations
    let caps = regex_for_add.captures(expression.as_str()).unwrap();
    let firts_number: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
    let second_number: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
    println!("{:?} firts: {}, second: {}", caps, firts_number, second_number);

    let addition: i32 = firts_number + second_number;

    // Results
    println!("Adition: {}", addition);
}
