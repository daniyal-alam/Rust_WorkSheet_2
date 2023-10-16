#[derive(Debug)]

struct Person {
    name: String,
    age: u32,
}

fn main() {
    let person1 = Person {
        name: String::from("Daniyal"),
        age: 22,
    };

    println!("{}", person1.name);
}
