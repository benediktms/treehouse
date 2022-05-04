use std::io::stdin;

struct Visitor {
    name: String,
    greeting: String,
}

impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_lowercase(),
        }
    }

    fn greet(&self) {
        println!("{}", self.greeting)
    }
}

fn main() {
    let visitor_list = [
        Visitor::new("ben", "I am ben. Hi me!"),
        Visitor::new("vale", "Hi vale, good to see you"),
        Visitor::new("el", "Hey el, what's up?"),
        Visitor::new("cho", "Hey chlo, hope you brough chocolate"),
    ];

    println!("Hello, what's your name?");
    let name = get_name();

    let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);

    match known_visitor {
        Some(visitor) => visitor.greet(),
        None => println!("I don't know you. Get outta here!"),
    }
}

fn get_name() -> String {
    let mut name = String::new();
    stdin()
        .read_line(&mut name)
        .expect("Failed to read message");

    name.trim().to_lowercase()
}
