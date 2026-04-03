mod borrowing;
mod enums;
mod loops;
mod ownership;
mod rules_for_borrowing;
mod structs;
mod variables;
use crate::enums::{Shape, cal_area};
mod error_handling;

fn main() {
    variables::show();
    loops::loops();
    ownership::own();
    ownership::takes_ownership();
    borrowing::borrow();
    borrowing::bor_variable();
    borrowing::mutable_borrow();
    structs::info();
    structs::print_area();
    let circle = Shape::Circle(7.0);
    let area = cal_area(circle);
    println!("Area is: {}", area);
    let result = error_handling::divide(15, 0);
    print!("Result: {:?}", result);
}
