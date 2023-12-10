use ollama_rs::{generation::completion::request::GenerationRequest, Ollama};
use std::io;
use std::io::Write;

const DEFAULT_MODEL: &str = "mistral";
const DEFAULT_PORT: u16 = 11434;
const DEFAULT_URL: &str = "http://localhost";

const MODEL_FLAG: &str = "--model";
const PORT_FLAG: &str = "--port";
const SYSTEM_FLAG: &str = "--system";
const URL_FLAG: &str = "--url";
const USER_FLAG: &str = "--user";

const SYSTEM_PROMPT: &str = "System:";
const USER_PROMPT: &str = "User:";

struct Arguments {
    user: String,
    system: String,
    model: Option<String>,
    url: Option<String>,
    port: Option<u16>,
}

#[derive(Debug)]
enum PromptResponse {
    Response(String),
    Error(String),
}

impl PromptResponse {
    fn as_str(&self) -> &str {
        match self {
            PromptResponse::Response(res) => res.as_str(),
            PromptResponse::Error(res) => res.as_str(),
        }
    }
}

impl std::fmt::Display for PromptResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PromptResponse::Response(res) => write!(f, "{}", res),
            PromptResponse::Error(res) => write!(f, "{}", res),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let Arguments {
        system,
        user,
        model,
        url,
        port,
    } = get_parsed_arguments();

    if user.is_empty() {
        println!("user argument cannot be empty, please provide a valid input");
        return Ok(());
    }

    let port = port.unwrap_or(DEFAULT_PORT);
    let model = model.unwrap_or(DEFAULT_MODEL.to_string());
    let ollama_url = url.unwrap_or(DEFAULT_URL.to_string());
    let stdin_text = read_stdin_lines();
    let ollama = get_ollama(ollama_url.to_string(), port);

    let system = if system.is_empty() {
        get_default_system_prompt()
    } else {
        system
    };

    let stdin_input = format!(". Input: {}", stdin_text);
    let prompt = format!("{} {}.\n{} {}{}", SYSTEM_PROMPT, USER_PROMPT, system, user, stdin_input);
    let res = prompt_ollama(prompt, &ollama, model.to_string()).await;

    if let PromptResponse::Error(res) = res {
        println!("error prompting ollama with model {}", model);
        return Err(io::Error::new(io::ErrorKind::Other, res));
    }

    output_to_stdout(res.as_str());

    Ok(())
}

fn read_stdin_lines() -> String {
    if atty::is(atty::Stream::Stdin) {
        return String::new();
    }

    io::stdin()
        .lines()
        .map(|line| line.unwrap_or_default())
        .collect::<Vec<String>>()
        .join("\n")
}

fn output_to_stdout(output: &str) {
    let mut stdout = io::stdout().lock();
    stdout.write_all(output.as_bytes()).unwrap();
    stdout.flush().unwrap();
}

fn continue_gathering_args(
    args: &mut std::iter::Skip<std::env::Args>,
) -> bool {
    if let Some(arg) = args.next() {
        if arg == SYSTEM_FLAG
            || arg == USER_FLAG
            || arg == MODEL_FLAG
            || arg == URL_FLAG
            || arg == PORT_FLAG
        {
            return true;
        }
    }
    false
}

fn get_parsed_arguments() -> Arguments {
    let mut args = std::env::args().skip(1);
    let mut system = Vec::new();
    let mut user = Vec::new();
    let mut model = None;
    let mut url = None;
    let mut port = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            SYSTEM_FLAG => {
                while continue_gathering_args(&mut args) {
                    system.push(args.next().unwrap_or_default())
                }
            }
            USER_FLAG => user.push(args.next().unwrap_or_default()),
            MODEL_FLAG => model = Some(args.next().unwrap_or_default()),
            URL_FLAG => url = Some(args.next().unwrap_or_default()),
            PORT_FLAG => {
                if let Some(port_str) = args.next() {
                    port = port_str.parse::<u16>().ok();
                }
            }
            _ => user.push(arg),
        }
    }

    Arguments {
        system: system.join(" "),
        user: user.join(" "),
        model,
        url,
        port,
    }
}

fn get_ollama(url: String, port: u16) -> Ollama {
    Ollama::new(url, port)
}

async fn prompt_ollama(prompt: String, ollama: &Ollama, model: String) -> PromptResponse {
    let res = ollama.generate(GenerationRequest::new(model, prompt)).await;

    match res {
        Ok(resp) => PromptResponse::Response(resp.response),
        Err(err) => PromptResponse::Error(err.to_string()),
    }
}

fn get_default_system_prompt() -> String {
    [
        "You are a command-line program that takes an input and provides an output ONLY.",
        "Give me only the output, without any additional labels (e.g., 'Output' or 'Result').",
        "The output should be usable as input in another program that is not an LLM.",
        "Avoid unnecessary chat.",
        "No preamble, get straight to the point.",
        "Generate a text response suitable for downstream processing by another program.",
        "Do not change the content of the input unless specifically asked to.",
        "Do not repeat back the input.",
    ]
    .join(" ")
}

