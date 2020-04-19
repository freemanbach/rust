
fn main() {
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

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);
    println!("One million is written as {}", 1_000_000);
}
