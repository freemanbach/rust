
fn main() {

    // boolean
    let a: bool = true;

    // 64 bit floating point
    let myfloat1: f64 = 1.0;  // Regular annotation
    let myfloat2 = 1.0f64;    // Suffix annotation
    

    // mutable integer 
    let mut myint: i32   = 5;  // regular annotation
    myint = 90;

    // non-mutable integer and floating point
    let std_float   = 3.0; // `f64`
    let std_integer = 7;   // `i32`

    // regular fixed array
    let std_array = [0;5];

    // Fixed-size array with size
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // fixed-size array with fixed initialized values
    let ys: [i32; 500] = [0; 500];


    // Integer addition
    println!("9 + 7 = {}", 9 + 7);

    // Integer subtraction
    println!("10 - 4 = {}", 10 - 4);

    // Integer multiplication 
    println!("3 * 4 = {}", 3 * 4);

    // Integer division
    println!("10.0 / 2.5 = {}", 10 / 2);
    println!("10.0 / 2.5 = {}", 10.0f64 / 2f64);

    // boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Array / Tuple
    let myarray1 = (1, 4, 6, 8u64, -1, -2, -3, -8i64, 0.1f32, 0.2f64, 'a', "Hello earth", true);
    let myarray2 = (1, 4, 6, 8u64, -1);
    println!("print out: {:?}", myarray2);

    // underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);
    println!("One million is written as {}", 1_000_000);
}
