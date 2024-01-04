#[no_mangle]
pub extern "C" fn datatypes() {
    // There are two types of datatypes in Rust: scalar and compound.
    // Scalar types represent a single value. Rust has four primary scalar types:
    // integers, floating-point numbers, Booleans, and characters.
    // Compound types can group multiple values into one type. Rust has two primitive compound types:
    // tuples and arrays.
    // Rust is a statically typed language, which means that it must know the types of all variables at compile time.
    
    let x = 1; // i32
    let y = 2.0; // f64
    let z = true; // bool
    let a = 'a'; // char
    let b = (1, 2.0, true, 'a'); // tuple
    let c = [1, 2, 3, 4, 5]; // array

    println!("x: {}", x);
    println!("y: {}", y);
    println!("z: {}", z);
    println!("a: {}", a);
    println!("b: {:?}", b);
    println!("c: {:?}", c);
    
}