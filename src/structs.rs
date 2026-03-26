struct User{
    name: String,
    age: u32,
    active: bool,
}

pub fn info(){
    let user1 = User{
        name : String::from("krat"),
        age:21,
        active:true,
    };
    println!("name: {}, age: {}, active: {}", user1.name, user1.age, user1.active);
}

// implementing structs

struct Rect{
    width :u32,
    height:u32,
}

impl Rect{
    fn area(&self)->u32{
        self.width * self.height
    }
}

pub fn print_area(){
    println!("Area of rectangle is {}", Rect{width: 10, height: 20}.area());
}
