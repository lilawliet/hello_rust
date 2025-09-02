use hello_rust::calculator::{add, subtract, multiply};

// src/main.rs
fn main() {
    let name:&'static str = "LangShift";
    let age: i32 = 25;
    let is_student: bool = true;
    let scores = [100, 95, 90];

    println!("Name: {}", name);
    println!("Age: {}", age);
    println!("Is Student: {}", is_student);
    println!("Scores: {:?}", scores);

    fn calculate_area(width: f64, height: f64) -> f64 {
        width * height
    }
    let area = calculate_area(10.0, 20.0);
    println!("Area: {}", area);


    fn is_adult(age: i32) -> bool {
        age >= 18
    }
    let is_adult = is_adult(age);
    println!("Is Adult: {}", is_adult);

    let grade = 85;
    let result = if grade >= 90 {
        "A"
    } else if grade >= 80 {
        "B"
    } else {
        "C"
    };
    println!("Grade: {}", result);

    let mut count: i32 = 0;
    count += 1;
    println!("Count: {}", count);
    
    println!("Hello, Rust!");
    
    // 变量声明（不可变）
    let name = "LangShift";
    println!("Hello, {}!", name);
    
    // 函数定义
    fn greet(name: &str) -> String {
        format!("Hello, {}!", name)
    }
    
    println!("{}", greet("Developer"));
    
    println!("1.0 + 2.0 = {}", add(1.0, 2.0));  
    println!("1.0 - 2.0 = {}", subtract(1.0, 2.0));
    println!("1.0 * 2.0 = {}", multiply(1.0, 2.0));
}