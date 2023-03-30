use std::{env, fs, path::PathBuf};

const INITIAL_PROMPT: &str = concat!(
    "Act as my personal AI Assistent. ",
    "You will assist me in my daily tasks.\n",
    "The assistant is helpful, creative, clever, and very friendly.\n\n",
    "Human: Hello, who are you?\n",
    "AI: I am an AI created by OpenAI. How can I help you today?\n",
    "Human: Can you calculate how much is 3+2?\n",
    "AI: 3 + 2 = 5\n",
);
const HISTORY_FILE: &str = ".openai-history";
const CONTEXT_SIZE: usize = 100;

pub struct History {
    file: String,
    fallback: String,
    context_size: usize,
}

impl History {
    pub fn new() -> Self {
        let home_dir = env::var("HOME").expect("HOME env var not set");
        Self {
            file: format!("{}/{}", home_dir, HISTORY_FILE),
            fallback: INITIAL_PROMPT.to_string(),
            context_size: CONTEXT_SIZE,
        }
    }

    pub fn context(&self) -> String {
        // load history from file
        let history = self.history();
        match history.len() {
            // limit context size
            size if size > self.context_size * 2 => {
                let offset = history.len() - self.context_size;
                let init = history[..offset].to_string();
                let end = history[offset..].to_string();
                init + &end
            }
            _ => history,
        }
    }

    pub fn persist(&self, input: String, output: String) {
        let path = PathBuf::from(&self.file);
        // Prepare context for file
        let context = format!("Human: {}\nAI: {}\n", input, output);
        let history = format!("{}{}", self.history(), context);

        fs::write(path.clone(), history).expect("Unable to write file");
    }

    fn history(&self) -> String {
        match fs::read_to_string(self.file.clone()) {
            Ok(history) => match history.len() {
                0 => self.fallback.clone(),
                _ => history,
            },
            Err(_) => self.fallback.clone(),
        }
    }
}
