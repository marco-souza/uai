use tokio;
use dotenv::dotenv;

mod openai;

fn setup() {
    // Load environment variables from the .env file
    dotenv().ok();
}

#[tokio::main]
async fn main() {
    setup();

    let client = openai::OpenAIClient::new();
    // get prompt merging all agrs
    let args = std::env::args();
    let prompt = args.skip(1).collect::<Vec<String>>().join(" ");
    let role = concat!(
        "The following is a conversation with an AI assistant. ",
        "The assistant is helpful, creative, clever, and very friendly.\n\n",
        "Human: Hello, who are you?\n",
        "AI: I am an AI created by OpenAI. How can I help you today?\n",
        "Human: ",
    );
    let prompt = format!("{}{}\n", role, prompt);

    let text = client.prompt(prompt.clone()).await.unwrap();
    println!("{}", text);
}
