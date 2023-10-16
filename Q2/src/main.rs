#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
    Yellow,
    Cyan,
    Magenta,
    Black,
}

fn RGB(color1: &Color, color2: &Color) -> Color {
    match (color1, color2) {
        (&Color::Red, &Color::Green) | (&Color::Green, &Color::Red) => Color::Yellow,
        (&Color::Red, &Color::Blue) | (&Color::Blue, &Color::Red) => Color::Magenta,
        (&Color::Green, &Color::Blue) | (&Color::Blue, &Color::Green) => Color::Cyan,
        (&Color::Red, &Color::Red) => Color::Red,
        (&Color::Green, &Color::Green) => Color::Green,
        (&Color::Blue, &Color::Blue) => Color::Blue,
        _ => Color::Black,
    }
}

fn main() {
    let color1 = Color::Red;
    let color2 = Color::Green;
    let mixed_color = RGB(&color1, &color2);
    println!("{:?}", color1);
    println!("{:?}", color2);
    println!(
        "{:?} is the combination of {:?} and {:?}.",
        mixed_color, color1, color2
    );
}
