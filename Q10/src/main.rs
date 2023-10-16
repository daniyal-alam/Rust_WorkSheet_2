mod utils;

#[derive(Debug)]
pub struct Book {
    title: String,
    author: String,
    pages: i32,
}

fn main() {
    let book1 = Book {
        title: String::from("Fourty rules of life "),
        author: String::from("Daniyal."),
        pages: 100,
    };

    utils::display_book(&book1);
}
