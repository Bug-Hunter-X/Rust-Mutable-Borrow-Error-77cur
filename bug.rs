fn main() {
    let mut x = 5;
    let y = &mut x;
    let z = &mut x; // This is fine, mutable borrows are allowed
    *y = 10;
    *z = 15; // This will cause a compiler error because of mutable borrows
    println!("x = {}", x);
}