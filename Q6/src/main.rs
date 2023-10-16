#[derive(Debug)]
enum Status {
    Active(String),
    Inactive(String),
    Suspended(String),
}

#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    pages: i32,
}

impl Book {
    fn get_title(&self) -> &str {
        &self.title
    }

    fn get_author(&self) -> &str {
        &self.author
    }

    fn book_status(book: &Book, status: Status) -> (String, Status) {
        (book.title.clone(), status)
    }

    fn own_book(book: Book) -> String {
        book.title
    }

    fn borrow_book(book: &mut Book) {
        book.title.push_str("and death");
        println!("{:?}", book);
    }
}

fn main() {
    // creating instance of a struct
    let mut book1 = Book {
        title: String::from("Fourty rules of life "),
        author: String::from("Daniyal."),
        pages: 100,
    };

    // enum instance
    let status = Status::Active(String::from("Active"));

    println!(
        "{:?}, Author's name is {}",
        Book::book_status(&book1, status),
        book1.author
    );

    let someValue: Option<i32> = Some(book1.pages);
    let noneValue: Option<i32> = None;

    match someValue {
        Some(_) => println!("Has a value."),
        None => println!("No value."),
    }

    // println!("{}", Book::own_book(book1));
    println!("{:?}", Book::borrow_book(&mut book1));
}
