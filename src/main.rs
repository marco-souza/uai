mod history;
mod openai;

use history::History;
use tokio;

#[tokio::main]
async fn main() {
    let history = History::new();
    let client = openai::OpenAIClient::new();

    // get prompt merging all agrs
    let input = get_cli_input();
    let context = history.context();
    let prompt = format!("{}{}\nAI: ", context, input);

    let output = client.prompt(prompt).await.unwrap();
    println!("{}", output);

    history.persist(input, output);
}

fn get_cli_input() -> String {
    let args: Vec<String> = std::env::args().collect();
    args[1..].join(" ")
}
