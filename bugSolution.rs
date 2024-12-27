fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &y; // z is an immutable reference to y. This is allowed because z doesn't try to access x while it's being modified
    *y = 10; // This is allowed, even though z is an immutable reference.  z does not borrow x
    println!("x = {}", x); // Prints 10
    println!("y = {}", *y); //Prints 10
    println!("z = {}", *z); //Prints 10
} 