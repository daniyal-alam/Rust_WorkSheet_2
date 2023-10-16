fn append(something: &mut String) {
    something.push_str("World!");
    println!("{}", something);
}

fn main() {
    let mut some = String::from("Hello, ");
    append(&mut some);
}
