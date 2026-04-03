/*
Error in rust
1. compile-time error-> If your code has mistakes, the compiler fails BEFORE generating binary
2.Runtime Error -> Code is valid → compiler creates binary → CPU starts running it

By error-handling-program can continue to run even if there is an error..and we can also provide a way to handle the error gracefully

*/
// enum Result<T, E> {  //Result is a built-in enum in Rust that is used for error handling. It has two variants: Ok and Err. Ok is used to indicate a successful operation, while Err is used to indicate an error. The Result enum is defined as follows:
//     Ok(T),   // success
//     Err(E),  // error
// }

pub fn divide (a: i32, b:i32) -> Result<i32, String>{
    match b{
        0 => Err("cannot divide it by 0".to_string()),
        _ => Ok(a/b) 
     }
}