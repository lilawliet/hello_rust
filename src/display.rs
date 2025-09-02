use std::fmt;

struct Point {
    x: f64,
    y: f64,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point({},{})", self.x, self.y)
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point({},{})", self.x, self.y)
    }
}

fn main() {
    let p = Point { x: 1.0, y: 2.0 };
    println!("{}", p);
    println!("{:?}", p);
}