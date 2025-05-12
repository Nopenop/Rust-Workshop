fn main() {
    let x = 2.0; // f64
    println!("float64 x = {}", x);

    let y: f32 = 3.0; // f32
    println!("float32 y = {}", y);

    // addition
    let sum = 5 + 10;
    println!("{}", sum);

    // subtraction
    let difference = 15 + 10;
    println!("{}", difference);

    // multiplication
    let product = 3 * 4;
    println!("{}", product);

    // division
    // println!("{}",// );
    let quotient = 56.7 / 32.2;
    println!("{}", quotient);
    let truncated = -5 / 3; // Results in -1
    println!("{}", truncated);

    // remainder
    let remainder = 43 % 5;
    println!("{}", remainder);

    // booleans
    let t = true;
    println!("{}", t);
    let f = false;
    println!("{}", f);

    // characters (either utf8 or u8)
    let c = 'z';
    println!("{}", c);
    let z: char = 'ℤ';
    println!("{}", z);
    let heart_eyed_cat = '😻';
    println!("{}", heart_eyed_cat);

    // Tuple type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{:#?}", tup);
    let (a, _, _) = tup;
    println!("Value of a is {}", a);
    println!("Value 1 is {}", tup.1);

    // array types
}
