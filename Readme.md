# Rusty-bot

Rusty-bot is a Discord bot that monitors users in a Discord server and stores information about their reputation in an external database. The bot is built using Rust and utilizes Prisma for database access.

## Features

- Monitors users in a Discord server
- Stores information about their reputation in an external database
- Uses Prisma for database access

## Installation

To install Rusty-bot, follow these steps:

1. Clone the repository: `git clone https://github.com/JohnnyJumper/RustyBot.git`
2. Navigate to the project directory: `cd RustyBot`
3. Install dependencies: `cargo build`
4. Set up the environment variables for the bot and database
5. Run the bot: `cargo run`

## Environment Variables

The following environment variables are required for Rusty-bot to function properly:

- `DISCORD_TOKEN`: Discord bot token
- `DATABASE_URL`: URL of the external database

Add thouse to .env in root folder

## Usage

Rusty-bot will automatically monitor users in the Discord server it is added to. The bot will store information about each user's reputation in the external database. This information can be accessed and manipulated via the Prisma API.

## Contributing

Contributions to Rusty-bot are welcome! To contribute, follow these steps:

1. Fork the repository
2. Create a new branch: `git checkout -b my-feature-branch`
3. Make your changes and commit them: `git commit -am 'Add some feature'`
4. Push the branch to your fork: `git push origin my-feature-branch`
5. Create a new pull request

## License

Rusty-bot is licensed under the [MIT License](https://opensource.org/licenses/MIT).
