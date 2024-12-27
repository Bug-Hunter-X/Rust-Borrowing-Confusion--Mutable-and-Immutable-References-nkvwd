fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &y; // z is an immutable reference to y
    *y = 10; // This is allowed, even if z is an immutable reference
}