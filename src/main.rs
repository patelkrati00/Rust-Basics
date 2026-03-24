mod variables;
mod loops;
mod ownership;
mod borrowing;
fn main() {
    variables::show();
    loops::loops();
    ownership::own();
    ownership::takes_ownership();
    borrowing::borrow();
    borrowing::bor_variable();
    borrowing::mutable_borrow();
 }
