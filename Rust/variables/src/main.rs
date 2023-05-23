fn main() {
    // Variables and Mutability
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    // Data Types
    let _x = 2.0; // f64
    let _y: f32 = 3.0; // f32
    let _sum = 5 + 10; // addition
    let _difference = 95.5 - 4.3; // subtraction
    let _product = 4 * 30; // multiplication
    let _quotient = 56.7 / 32.2; // division
    let _truncated = -5 / 3; // Results in -1
    let _remainder = 43 % 5; // remainder
    let _t = true;
    let _f: bool = false; // with explicit type annotation
    let _c = 'z';
    let _z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("Emoji: {heart_eyed_cat}");
    let tup = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is: {y}");
    let last_elem = tup.2;
    println!("The value of z is: {last_elem}");
    let a: [i32; 3] = [-1, 2, -3];
    let a = a[0];
    println!("The first value of the array: {a}");

    // Functions
    another_function(23, 'c');
    let y = {
        let x = 3;
        x + 1
    };
    fn another_function(value: i32, unit: char) {
        println!("Another function unit: {value}{unit}");
    }
    fn plus_one(x: i32) -> i32 {
        x + 1
    }
    println!("The value of y is: {y}");
    let z = plus_one(5);
    println!("The value of 5 + 1 : {z}");

    // Control Flow
    let number = 7;
    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }
    let condition = false;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");
    let mut number = 3;
    while number != 0 {
        println!("Loop: {number}");
        number -= 1;
    }
    let a = [10, 20, 30, 40];
    for element in a {
        println!("The value is: {element}");
    }
}
