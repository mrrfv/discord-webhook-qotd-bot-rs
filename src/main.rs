use std::fs::File;
use std::io::Write;
use std::env;
use std::process::exit;
use std::fs;
use serde::{Serialize, Deserialize};
use reqwest::blocking::Client;

struct Config {
    discord_webhook_url: String,
    questions_file_location: String,
    progress_file_location: String
}

impl Config {
    fn build() -> Self {
        fn get_env_var(var_name: &str) -> String {
            env::var(var_name).unwrap_or_else(|_| {
                eprintln!("Variable {} not defined, can't continue.", var_name);
                exit(1)
            })
        }

        let discord_webhook_url = get_env_var("DISCORD_WEBHOOK_URL");
        let questions_file_location = get_env_var("QUESTIONS_FILE");
        let progress_file_location = get_env_var("PROGRESS_FILE");

        Config {
            discord_webhook_url,
            questions_file_location,
            progress_file_location
        }
    }
}

#[derive(Serialize, Deserialize)]
struct EmbedFooter {
    text: String,
}

#[derive(Serialize, Deserialize)]
struct Embed {
    color: u32,
    title: String,
    description: String,
    timestamp: String,
    footer: EmbedFooter,
}

#[derive(Serialize, Deserialize)]
struct RequestData {
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    embeds: Vec<Embed>,
}

fn post_question(client: Client, webhook_url: &String, question: &String, progress: u64, questions_length: usize) -> Result<(), String> {
    let embed = Embed {
        color: 0x00ff00,
        title: String::from("Question of the Day"),
        description: question.to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        footer: EmbedFooter { text: format!("{} questions left", questions_length - progress as usize - 1) }
    };
    let request_data = RequestData {
        content: None,
        embeds: vec![embed]
    };

    let json = serde_json::to_string(&request_data).unwrap();
    
    let res = client.post(webhook_url)
        .body(json)
        .header("Content-Type", "application/json")
        .send()        
        .expect("HTTP request to Discord failed");

    if !res.status().is_success() {
        let resp = res.text().unwrap();
        return Err(resp);
    } else {
        return Ok(());
    }
}

fn main() {
    let config = Config::build();
    let client = Client::new();

    // Read questions file
    let questions_file_content = fs::read_to_string(&config.questions_file_location)
        .expect("Couldn't read questions file");
    let questions: Vec<String> = serde_json::from_str(&questions_file_content).expect("Invalid JSON");

    // Read progress
    let mut progress: u64 = fs::read_to_string(&config.progress_file_location)
        .expect("Progress file couldn't be read")
        .trim()
        .parse()
        .expect("Progress file invalid");

    // Make sure the questions file isn't empty
    if questions.len() == 0 {
        panic!("There are no questions defined in the questions file");
    }

    // Before posting the question, ensure we aren't out of bounds (and reset the progress in that case)
    if questions.len() <= progress as usize {
        println!("All questions have been sent! Resetting progress ({}).", progress);
        progress = 0;
    }

    if let Err(e) = post_question(client, &config.discord_webhook_url, &questions[progress as usize], progress, questions.len()) {
        // Panic if a question failed to be posted so the progress isn't unnecessarily incremented
        panic!("Failed to post question: {}", e);
    } else {
        println!("Question posted successfully");
    }

    // Increment progress file
    let mut progress_file = File::create(&config.progress_file_location).expect("Couldn't open progress file for writing");
    progress_file.write_all(format!("{}", progress + 1).as_bytes()).expect("Progress couldn't be saved.");
}
