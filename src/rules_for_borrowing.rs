fn rule1() {
    println!("Rules for Borrowing in Rust:");
    println!("1. any num of immutable refrences are asllowed ");

    let s1 = String::from("s1");
    let s2 = &s1;
    let s3 = &s1;
    let s4 = &s1;
}

fn rule2() {
    println!("2. only one mutable reference is allowed at a time");

    let mut s1 = String::from("s1");
    let s2 = &mut s1; // only one mutable reference is allowed
    // let s3 = &mut s1; // this will cause a compile-time error because we cannot have two mutable references to the same data at the same time
}

fn rule3() {
    println!("3.you cannot have a mutable reference while you have an immutable reference");
    let mut s1 = String::from("s1");
    let s2 = &s1; // immutable reference
    // let s3 = &mut s1; // this will cause a compile-time error because we cannot have a mutable reference while we have an immutable reference
    println!("s2: {}", s2); // s2 is still valid here because it is an immutable reference
}


