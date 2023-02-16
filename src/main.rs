use std::io;
use slack;
use slack::{Event, RtmClient};

fn main() {
    loop {
        println!("Hi! I'm Rustbot, how can I help you?");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("I'm sorry, I didn't understand that");

        if input.trim().ends_with("?") {
            let response: String = generate_response();
            println!("{}", response);
        } else {
            println!("That wasn't a question! Ask me a question.");
        }
    }
}

fn generate_response() -> String {
    let responses = vec![
        "I'm sorry, I don't know the answer to that.",
        "I'm not sure, but I'll look into it.",
        "I'm not sure, but I'll ask my friends.",
    ];
    let random_index = rand::random::<usize>() % responses.len();
    responses[random_index].to_string()
}
