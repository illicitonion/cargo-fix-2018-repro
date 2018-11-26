extern crate ignore;

fn main() {
    for result in ignore::Walk::new(".") {
        println!("{:?}", result);
    }
}
