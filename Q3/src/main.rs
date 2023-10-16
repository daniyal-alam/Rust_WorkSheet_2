fn main() {
    let numbers = (5, 6);
    println!("{}", sum_of_elements(numbers));
}

fn sum_of_elements(tup: (i32, i32)) -> i32 {
    let (a, b) = tup;
    (a + b)
}
