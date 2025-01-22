

enum Color {
    Red,
    Blue,
    Green,
}

/// Prints the name of the given color.
///
/// # Parameters
///
/// * `c` - An instance of the `Color` enum representing the color to be printed.
///
/// # Return
///
/// * This function does not return a value. It prints the name of the color to the console.
fn main() {
    let c: Color = Color::Blue;
    match c {
        Color::Red => println!("The color is red"),
        Color::Blue => println!("The color is blue"),
        Color::Green => println!("The color is green"),
    }
}