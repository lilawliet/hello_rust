pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn sub(a: i32, b: i32) -> i32 {
    a - b
}

pub fn mul(a: i32, b: i32) -> i32 {
    a * b
}

// src/utils.rs
pub fn format_string(s: &str) -> String {
    format!("String: {}", s)
}

// src/models.rs
#[derive(Debug)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub age: u32
}

impl User {
    pub fn new(name: String, age: u32) -> Self {
        Self {
            id: 0,
            name,
            age
        }
    }

}