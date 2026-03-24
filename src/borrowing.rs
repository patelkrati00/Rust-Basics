pub fn borrow(){
    let s1 = String::from("borrowing example");
    let s2 = &s1; // s2 borrows s1, ownership is not transferred
    println!("s1: {}, s2: {}", s1, s2); // both s1 and s2 are valid here
}

pub fn bor_variable(){
    let my_string = String::from("hello! rust");
    borrow_variable(&my_string);
    println!("my_string is still valid here: {}", my_string); // my_string is still valid because it was borrowed, not moved
}

fn borrow_variable(s: &String){
    println!("Borrowed string: {}", s); // s is a reference to the original string, it does not own the data
}

//Mutable Refrences

pub fn mutable_borrow(){
    let mut my_string = String::from("Mutable Refrence Example ");
    // update_string(&my_string); // we can pass a mutable reference to the function, but we cannot modify the original string through an immutable reference
    //to do so we need to pass a mutable reference
    update_string(&mut my_string); // passing a mutable reference to the function
    println!("Updated string: {}", my_string); // my_string is updated because we passed a mutable reference to the function
}
// fn update_string(s: &String){
//     s.push_str("world"); //it'll not update the mystring as it only passed by refrence 
// }

fn update_string(s: &mut String){
    s.push_str("updating my_string");
}

//hanky panky example

// pub fn hanky_panky(){
//     let mut my_string = String::from("hanky Panky example");
//     let borrower1 = &mut my_string;
//     // print!("Borrower 1: {}", borrower1); //borrow ends
//     update_str(&mut my_string);// invalid because rihanna can do hanky panky either with owner or with borrower1 and not with other borroower and here she is doing with borrower1 so cant do with update_str

//     print!("Borrower 1: {}", borrower1); //borrow active 
// }
// fn update_str(s: &mut String){
//     s.push_str("touche");
// }