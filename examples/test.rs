extern crate urbandict;

fn main() {
    for definition in urbandict::get_definitions("irc").unwrap() {
        println!("{:?}", definition);
    }
}
