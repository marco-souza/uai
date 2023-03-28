use std::env;
use dotenv::dotenv;

fn setup() {
    // Load environment variables from the .env file
    dotenv().ok();
}

fn main() {
    setup();

    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY is not set");

    println!("Hello, world!\n{}", api_key);
}
