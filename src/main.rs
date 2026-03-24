mod variables;
mod loops;
mod ownership;
fn main() {
    variables::show();
    loops::loops();
    ownership::own();
    ownership::takes_ownership();
}
