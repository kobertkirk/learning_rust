fn main() {
    let sum = 5 +10;

    let diference = 95.5 - 4.3;

    let product = 3 * 2;

    let quotient = 8 / 2;

    let truncated = -5/3;

    let remainder = 43 % 5;
    println!("Sum: {}, Difference: {}, Product: {}, Quotient: {}, Truncated Quotient: {}, Remainder: {}", sum, diference, product, quotient, truncated, remainder);

    let t = true;
    let f: bool = false;
    println!("{:?}", t);
    println!("{:?}", f);

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("{:?}", c);
    println!("{:?}", z);
    println!("{:?}", heart_eyed_cat);

    let tup:(i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("{}, {}, {}", x, y, z);

    //give me an array example
    let arr = [1, 2, 3, 4, 5];
    println!("{:?}", arr);

    //give me a string example
    let s = "Hello, world!";
    println!("{}", s);

    //give me a mutable string example
    let mut mutable_s = String::from("Hello, ");
    mutable_s.push_str("world!");
    println!("{}", mutable_s);

    //give me a tuple of strings example
    let tuple_of_strings = ("Hello", "world!");
    println!("{:?}", tuple_of_strings);

}