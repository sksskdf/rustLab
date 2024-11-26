fn main() {
    let mut a: u8 = 72;
    let mut b: u8 = 73;

    print!("a = {}, b = {} \n", a as char, b as char);
    a = a ^ b;
    b = a ^ b;
    a = a ^ b;

    print!("a = {}, b = {}", a as char, b as char);   
}
