use std::io;
/**
 * Building a command line application using:
 * 
 * Functions
 * Basic Data Types
 * Standard Library
 * Memory Ownership
 * 
 * */
//uses snake case
fn main() {
    let mut input = String::new();
    // FIXME: let mut s = input -> in Rust now the ownership is moved from input to s, 
    //for Rust by design wants to prevent double-free; in this case input is no longer valid, so will throw a compiler error
    //exception: those types whose size is known at compile time -> basic types such as i8, etc.
    /*
     * let a = 5;
     * let b = a; -> this is allowed
     */
    // some_fn(input);  <- even this counts as move
    io::stdin().read_line(&mut input).unwrap();
    let weight: f32 = input.trim().parse().unwrap();
    //let result: f32 = calculate_weight_on_mars(100.0);
    let mut result: f32 = calculate_weight_on_mars(weight);
    result = result * 1000.0;
    println!("Hello, I weigh {}g on {}", result, "Mars"); //println! <- a macro
    
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    weight * 3.711/9.81
}

/*
*   fn some_fn(s: String) {}
*/

/*
*   fn some_fn(s: &mut String) {
    s.push_str("a");
}
*/
