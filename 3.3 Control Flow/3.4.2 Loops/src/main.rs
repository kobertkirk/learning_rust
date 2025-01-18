fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

    
    let fahrenheit = 32.0;
    let celsius = convert_fahrenheit_to_celsius(fahrenheit);
    println!("{}°F is equal to {}°C", fahrenheit, celsius);

    let celsius = 0.0;
    let fahrenheit = convert_celsius_to_fahrenheit(celsius);
    println!("{}°C is equal to {}°F", celsius, fahrenheit);

    //generate the nth fibonaccinumber
    let n = 10;
    let fibonacci = generate_nth_fibonacci(n);
    println!("The {}th Fibonacci number is {}", n, fibonacci);

}


//comment each line of this function
fn generate_nth_fibonacci(n: u32) -> u64 {
    if n <= 0 {
        return 0;
    }

    let (mut a, mut b) = (1, 1);

    for _ in 2..=n {
        let next = a + b;
        a = b;
        b = next;
    }

    b
}

fn convert_fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn convert_celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}