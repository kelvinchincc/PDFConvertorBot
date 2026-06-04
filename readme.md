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

- `TELOXIDE_TOKEN`: The token for your Telegram bot.
- `WHITELISTED_USERS`: A comma-separated list of user IDs that are authorized to use the bot.
- `TELOXIDE_SECRET_TOKEN`: Secret token where Telegram will used to authorize with the bot.
- `TELOXIDE_WEBHOOK_URL`: Webhook URL where Telegram will call on receving messages.

You can create a `.env` file in the root of the project to store these variables. The bot uses the `DotNetEnv` library
to load the environment variables from this file. For other variables, refer to `sample.env` file. Docker evironment
variables are recommended over `.env` file if using docker.

## Deployment

The project includes a `Dockerfile` and a `docker-compose.yaml` file for easy deployment using Docker. Precompiled
images can be found at the [packages](https://github.com/users/kelvinchincc/packages/container/package/pdf-convertor-bot)
section, refer to the follwoing section for an example on using the precompiled package.

### Docker

It is recommended to use Docker to deploy the bot, a sample `docker-compose.yaml` file is as below:

```yaml
services:
  pdf-convertor-bot:
    image: ghcr.io/kelvinchincc/pdf-convertor-bot:latest
    ports:
      - "3000:3000"
    env_file:
      - .env
    volumes:
      - /docker/volumes/pdf-convertor-bot/downloads:/app/.downloads
```

### Without Docker

To run the bot without Docker, ensure you have Rust installed and cargo is up, recommened Rust 1.29.0+

```bash
cargo run
```

### Webhook URL Tips

This bot use [Teloxide](https://github.com/teloxide/teloxide) to handle web hook, please refer to the
[ngrok example](https://github.com/teloxide/teloxide/blob/master/crates/teloxide/examples/ngrok_ping_pong.rs) on how
webhook is used, the script `start-dev-tunnel.rb` would help to start a dev tunnel with
[tailscale funnel](https://tailscale.com/docs/features/tailscale-funnel)

I still exploring how the webhook path work on the Teloxide crate work so the provided script might change in future.

## License

This project use the MPL-2.0 license, refer to license.md for more deatils.
