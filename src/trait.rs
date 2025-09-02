trait Printable {
    fn print(&self);
}

struct Book {
    title: String,
}

impl Printable for Book {
    fn print(&self) {
        println!("Book: {}", self.title);
    }
}

struct Movie {
    title: String,
}

impl Printable for Movie {
    fn print(&self) {
        println!("Movie: {}", self.title);
    }
}

fn main() {

    let items: Vec<Box<dyn Printable>> = vec![
        Box::new(Book { title: "Rust Programming Language".to_string() }),
        Box::new(Movie { title: "The Avengers".to_string() }),
    ];

    for item in items {
        item.print();
    }

}