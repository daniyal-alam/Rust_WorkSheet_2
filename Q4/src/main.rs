enum Option {
    StringValue(String),
    NumberValue(u32),
}

fn print_number(option: Option) {
    match option {
        Option::StringValue(_) => (),
        Option::NumberValue(num) => {
            println!("The number value is: {}", num);
        }
    }
}

fn main() {
    let option_string = Option::StringValue("Hello, Rust!".to_string());
    let option_number = Option::NumberValue(1);

    print_number(option_string);
    print_number(option_number);
}
