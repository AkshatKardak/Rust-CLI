use anyhow::Result;
use std::io::{self, Write};

use crate::chat::Chat;

pub async fn run(mut chat: Chat) -> Result<()> {
    print_banner();

    loop {
        print!("\nYou > ");
        io::stdout().flush()?;

        let mut input = String::new();
        let bytes_read = io::stdin().read_line(&mut input)?;

        if bytes_read == 0 {
            println!("\nGoodbye!");
            break;
        }

        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        if let Some(command) = input.strip_prefix('/') {
            let mut parts = command.splitn(2, char::is_whitespace);
            let name = parts.next().unwrap_or_default();
            let argument = parts.next().unwrap_or_default().trim();

            match name {
                "help" => print_help(),

                "exit" | "quit" => {
                    println!("Goodbye!");
                    break;
                }

                "clear" => {
                    chat.clear();
                    println!("Conversation history cleared.");
                }

                "history" => {
                    let history = chat.history();

                    if history.is_empty() {
                        println!("No conversation history yet.");
                    } else {
                        for (index, item) in history.iter().enumerate() {
                            let role = match item.role.as_str() {
                                "user" => "You",
                                "model" => "Gemini",
                                other => other,
                            };

                            println!("\n{}. {}: {}", index + 1, role, item.text());
                        }
                    }
                }

                "model" => {
                    if argument.is_empty() {
                        println!("Current model: {}", chat.model());
                        println!("Usage: /model <model-name>");
                    } else {
                        match chat.set_model(argument.to_string()) {
                            Ok(()) => {
                                println!("Model set to: {}", chat.model());
                            }
                            Err(error) => {
                                eprintln!("Could not change model: {error:#}");
                            }
                        }
                    }
                }

                _ => println!("Unknown command: /{name}. Type /help for commands."),
            }

            continue;
        }

        print!("\nGemini > ");

        match chat.send_message(input).await {
            Ok(()) => println!("\n"),
            Err(error) => {
                eprintln!("\n\nRequest failed: {error:#}");
                eprintln!("Your previous conversation history is preserved.");
            }
        }
    }

    Ok(())
}

fn print_banner() {
    println!(
        r#"
╭──────────────────────────────────────────╮
│           RUST CLI ASSISTANT              │
│             Powered by Gemini             │
╰──────────────────────────────────────────╯
"#
    );

    println!("Model: Use /model to view or change it.");
    println!("Type /help to see available commands.");
}

fn print_help() {
    println!(
        r#"
Available commands:

  /help              Display this help
  /history           Display the current conversation
  /clear             Clear the current conversation
  /model             Display the current model
  /model <name>      Switch to another Gemini model
  /exit              Exit the application
  /quit              Exit the application

Anything else is sent to Gemini as a prompt.
"#
    );
}
