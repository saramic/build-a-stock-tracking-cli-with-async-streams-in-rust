fn main() {
    struct Book {
        title: String,
        is_used: Option<bool>,
    }

    let book = Book {
        title: "Great book".to_string(),
        is_used: None,
    };

    match book.is_used {
        Some(v) => println!("Book is used: {}", v),
        None => println!("We don't know if the book is used"),
    }
}
