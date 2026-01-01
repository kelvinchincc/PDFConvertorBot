# PDF Convertor Bot

A Telegram bot that converts PDF documents to images.

## Features

- Converts each page of a PDF document into a separate JPG image.
- Sends the converted images back to the user as a media group.
- Whitelist-based access control to restrict usage to authorized users.
- In the future might add Docker support for easy deployment.

## How to Use

1.  **Start a chat with the bot on Telegram.**
2.  **Send the `/start` command.** The bot will reply with a welcome message.
3.  **Send a PDF document to the bot.** The bot will process the document and send the converted images back to you.

## Configuration

The bot is configured using environment variables. The following variables are required:

- `BOT_TOKEN`: The token for your Telegram bot.
- `WHITELISTED_USERS`: A comma-separated list of user IDs that are authorized to use the bot.

You can create a `.env` file in the root of the project to store these variables. The bot uses the `DotNetEnv` library
to load the environment variables from this file.

### Example `.env` file

```
BOT_TOKEN=1234567890:ABCDEFGHIJKLMNOPQRSTUVWXYZ
WHITELISTED_USERS=123456789,987654321
```

## Deployment

The project includes a `Dockerfile` and a `compose.yaml` file for easy deployment using Docker.

> Note: Docker build are currently in experimental stage and may not work as expected.

### Docker

To build and run the bot using Docker, you can use the following commands:

```bash
docker build -t pdf-convertor-bot .
docker run -d --env-file .env pdf-convertor-bot
```

### Docker Compose

To build and run the bot using Docker Compose, you can use the following command:

```bash
docker-compose up -d
```

### Without Docker

To run the bot without Docker, ensure you have .NET installed and use the following commands:

```bash
dotnet build
dotnet run
```

Or download the prebuild from the [release](https://codeberg.org/kelvinchincc/PDFConvertorBot/releases) page

To run with the prebuild, use the following command:

```bash
7z x pdf-convertor-bot-linux-amd64-version-here.7z
sudo chmod u+x ./bin/PDFConvertorBot
./bin/PDFConvertorBot
```

Make sure to replace `version-here` with the actual version number of the release you downloaded. And ensure that the
`.env` file is in the same directory as the executable or provide the environment variables through other means.

To run it headless, systemd service or running with PM2 is recommended. Use fork mode if you are using PM2. Eg:

```bash
cd bin
pm2 start ./PDFConvertorBot --name "pdf-convertor-bot"
pm2 save
```
