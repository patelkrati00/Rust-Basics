// direct ownership change example
pub fn own() {
    let s1 = String::from("hello");

    println!("{}", s1); 
    //  s1 is still valid here (ownership has not changed yet)

    let s2 = s1;
    // ownership moved from s1 → s2

    // println!("{}", s1);  ERROR
    // s1 is no longer valid because ownership has been moved

    println!("{}", s2); 
    //  s2 is now the owner
}

// another way to change ownership is by passing a value to a function
pub fn takes_ownership(){
    let my_string = String::from("change ownership by passing to a function");
    changeownership(my_string);
    // my_string is no longer valid here because ownership has been moved to the function
    // println!("{}", my_string); // ERROR
}

fn changeownership(s: String) {
    println!("{}", s); // s is the owner of the string now
    // s goes out of scope here and the memory is freed
}

