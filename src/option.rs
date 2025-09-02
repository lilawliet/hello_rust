fn extract_number(s: &str) -> Option<i32> {
    s.chars().filter(|c| c.is_digit(10)).collect::<String>().parse::<i32>().ok()
}

fn main() {
    println!("Number: {:?}", extract_number("hello 212"));
    println!("Number: {:?}", extract_number("123"));
    println!("Number: {:?}", extract_number("123.45"));
    println!("Number: {:?}", extract_number("aaaa"));
}