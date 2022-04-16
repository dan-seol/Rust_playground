## Basic Data Types
uint, int
- 1 byte : u8, i8
- 2 bytes : u16, i16
- 4 bytes : u32, i32
- 8 bytes : u64, i64
- 16 bytes : u128, i128
- variable : usize, isize -> architecture dependent types

floating
- 4 bytes : f32
- 8 bytes : f64

bool -> 1 byte; why 1 byte? byte is the basic addressable unit, below which computer architecture cannot address

char -> 4 bytes


## Functions

fn calculate_weight_on_mars(weight: f32) -> f32 { // type signature
    50.0 //return statement can be simplified
}

## Macros
 - println!(...) is a macro
 - macros are used for meta programming
 - functions need to specify the number of arguments and their type 
 - macros can take variable number of arguments and types can change
 - con: the errors are more complex
 - println! is a macro since it can take a variable number of arguments

## Mutability

by default every variable in rust is immutable
let result = 30;
result = result * 30; //will fail
use 
let mut result =30;
result = result * 30; // it works

## The Standard Library
std::

## Ownership

1. \forall value in Rust is owned by a variable
2. when the owner goes out of scope, the value will be deallocate
3. there can only be ONE owner at a time -> done by design to prevent double-free problem (it can lead to memory corruption)

## References and Borrowing
Reference allow us to work with value without taking ownership
fn some_fn(s: &String) {

}

borrowing can be done in two ways

1) many immutable borrowing

let s1 =  &input;

2) only one mutable borrowing
let mut s1 = &mut input

Important feature -> if it compiles, there is no data race

Result <- at the core of error handling in Rust
Result <- success/error; unwrap() terminates program if there is an error