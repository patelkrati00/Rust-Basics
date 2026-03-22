pub fn show() {
    //integers
    let x: i32 = 5; //default i32
    println!("x -> {}", x);
    let y: i64 = 10; //i64
    println!("y -> {}", y);

    //floating point numbers
    let a: f64 = 3.14; //default f64
    println!("a -> {}", a);

    //boolean
    let is_female: bool = true;
    let is_above_18 = true;

    if is_female {
        println!("female")
    } else {
        print!("not a female")
    }

    if is_above_18 && is_female {
        println!("legal female")
    }

    // String is a growable, heap-allocated type.
    // Its size can change at runtime, unlike fixed-size arrays.
    // Because of this, memory is managed dynamically.

    // one way to write strings
    let greeting: String = String::from("hello World");
    println!("{}", greeting);

    //get nth char
    let char1: Option<char> = greeting.chars().nth(1);

    // println!("{}",char1.unwrap()); //unwrap is tells compiler that it is okay with the runtime error if it happens(it'll throw error if index is larger than a string) if its valid index toh it'll give the required char

    //another way
    // pattern matching

    match char1 {
        Some(c) => println!("{}", c),
        None => println!("No character at index 1000")
    };

}
