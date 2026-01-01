//======================================================================================================================
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//======================================================================================================================
// See https://aka.ms/new-console-template for more information

using DotNetEnv;
using Microsoft.Extensions.Logging;
using Telegram.Bot;
using Telegram.Bot.Types;
using Telegram.Bot.Types.Enums;

Env.Load();
Env.TraversePath();

using var loggerFactory = LoggerFactory.Create(builder =>
    builder.AddFilter("Microsoft", LogLevel.Warning).AddFilter("System", LogLevel.Warning)
        .AddFilter("LoggingConsoleApp.Program", LogLevel.Debug)
        .AddSimpleConsole(options =>
        {
            options.IncludeScopes = true;
            options.SingleLine = true;
            options.TimestampFormat = "YYYY-MM-DD hh:mm:ss ";
        }));
var logger = loggerFactory.CreateLogger<Program>();
logger.LogInformation("Starting Bot");

var botToken = Environment.GetEnvironmentVariable("BOT_TOKEN");
var whitelistedUsers = Environment.GetEnvironmentVariable("WHITELISTED_USERS")?
    .Split(',', StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries)
    .Select(long.Parse)
    .ToHashSet();

if (botToken == null)
{
    logger.LogCritical("Please set BOT_TOKEN environment variable");
    return;
}

if (whitelistedUsers == null || whitelistedUsers.Count == 0)
{
    logger.LogCritical("Please set WHITELISTED_USERS environment variable");
    return;
}

using var cancellationTokenSource = new CancellationTokenSource();
var bot = new TelegramBotClient(botToken, cancellationToken: cancellationTokenSource.Token);
var me = await bot.GetMe();
logger.LogInformation("Bot is " + me.Username);

bot.OnMessage += HandleMessage;

// Wait until cancellation
logger.LogInformation("Press any key to exit");
Console.ReadKey();
cancellationTokenSource.Cancel();

return;

async Task HandleMessage(Message message, UpdateType args)
{
    if (!whitelistedUsers.Contains(message.From?.Id ?? 0))
    {
        return;
    }

    if ((message.Text ?? "").StartsWith("/start", StringComparison.OrdinalIgnoreCase))
    {
        await bot.SendMessage(message.Chat,
            "Welcome to the PDF Converter Bot! Please send me a PDF document to process.");
        return;
    }

    if (message.Type != MessageType.Document || message.Document?.MimeType != "application/pdf")
    {
        await bot.SendMessage(message.Chat, "Please send a PDF document.");
        return;
    }

    await bot.SendMessage(message.Chat, $"Processing your PDF ({message.Document.FileName}) ...");
}
