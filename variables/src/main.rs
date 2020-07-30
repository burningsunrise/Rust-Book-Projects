fn main() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    let spaces = "    ";
    let spaces = spaces.len();

    println!("The value of x is: {}", x);

    let x = 2.0; // f64 (double precision)
    let y: f32  = 3.0; // f32 (single precision)

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    //multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    //remainder
    let remainder = 43 % 5;

    let t = true;
    let f: bool = false;

    let c = 'z';
    let z = 'Z';
    let heart_eyed_cat = '😻';

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of y is {}", y);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [1, 2, 3 ,4, 5];

    let a = [3; 5]; // contains 5 elements with the value 3

    let first = a[0];
    let second = a[1];

}
