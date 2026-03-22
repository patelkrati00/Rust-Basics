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
    }else{
        print!("not a female")
    }

    if is_above_18 && is_female{
        print!("legal female")
    }

}
