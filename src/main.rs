use std::io::stdin;

fn main() {
    println!("Hello, what's your name?");
    let name = get_name();
    println!("Hello, {name}")
}

fn get_name() -> String {
    let mut name = String::new();
    stdin()
        .read_line(&mut name)
        .expect("Failed to read message");
    name
}
