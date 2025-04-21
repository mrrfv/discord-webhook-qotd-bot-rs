# Discord Webhook QOTD Bot - Rust rewrite

A Rust rewrite of [discord-webhook-qotd-bot](https://github.com/mrrfv/discord-webhook-qotd-bot), an application that posts a "question of the day" to a Discord channel using a webhook, done to learn the language. Like the original Node.js project, it accepts a JSON array containing questions.

## Features

- Doesn't maintain its own schedule by design, as such features are provided by the operating system (Task Scheduler on Windows, cron on Linux)
- Automatically saves and increments progress
- Sends visually appealing embeds with additional information, such as the amount of questions remaining
- Starts over from the beginning when all questions have been asked
- Not vibe coded

## Usage

1. Copy the example question and progress files, found in the `example_data` directory, where you want to store this data.
2. Edit the `questions.json` file to add your own questions, adhering to the format.
3. Build the project using `cargo build --release` or download the precompiled binary from the releases page.
4. Run the program, specifying all the required environment variables:

- `DISCORD_WEBHOOK_URL`: The URL of the Discord webhook to send messages to.
- `PROGRESS_FILE`: The path to the progress file (default: `progress.json`).
- `QUESTION_FILE`: The location to the file containing questions (default: `questions.json`).

5. Schedule it to run at your desired interval using your operating system's task scheduler.
