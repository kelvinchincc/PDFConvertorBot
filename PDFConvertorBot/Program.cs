//======================================================================================================================
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//======================================================================================================================
// See https://aka.ms/new-console-template for more information

using DotNetEnv;
using ImageMagick;
using Microsoft.Extensions.Logging;
using Telegram.Bot;
using Telegram.Bot.Types;
using Telegram.Bot.Types.Enums;

Env.Load();
Env.TraversePath();

// Check if verbose logging is enabled via --verbose flag
var useVerboseLogging = args.Contains("--verbose");
var minLogLevel = useVerboseLogging ? LogLevel.Debug : LogLevel.Information;

using var loggerFactory = LoggerFactory.Create(builder =>
{
    builder.SetMinimumLevel(minLogLevel);
    builder.AddFilter("Microsoft", LogLevel.Warning).AddFilter("System", LogLevel.Warning)
        .AddSimpleConsole(options =>
        {
            options.IncludeScopes = true;
            options.SingleLine = true;
            options.TimestampFormat = "yyyy-MM-dd HH:mm:ss ";
        });
});
var logger = loggerFactory.CreateLogger<Program>();
logger.LogInformation("Starting Bot");
logger.LogInformation("Args: {args}", string.Join(", ", args));
logger.LogInformation("Verbose logging: {verbose}", useVerboseLogging);

var tempDirPath = Path.Combine(Environment.CurrentDirectory, "temp");

logger.LogInformation("Temporary directory: {tempDir}", tempDirPath);

if (!Path.Exists(tempDirPath))
{
    logger.LogInformation("Creating temporary directory");
    Directory.CreateDirectory(tempDirPath);
}
else
{
    // Clean up old files
    logger.LogInformation("Cleaning up old temporary files");
    var files = Directory.GetFileSystemEntries(tempDirPath);
    foreach (var file in files)
    {
        // Delete files and directories
        try
        {
            if (File.Exists(file))
            {
                File.Delete(file);
            }
            else if (Directory.Exists(file))
            {
                Directory.Delete(file, true);
            }
        }
        catch (Exception ex)
        {
            logger.LogWarning(ex, "Failed to delete temporary file or directory: {file}", file);
        }
    }
}

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
    logger.LogDebug("Received message from {UserId}", message.From?.Id);
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

    if (message.Type != MessageType.Document)
    {
        return;
    }

    if (message.Document is not { MimeType: "application/pdf" })
    {
        await bot.SendMessage(message.Chat, "Please send a PDF document.");
        return;
    }

    var fileId = message.Document.FileId;
    var taskId = Guid.NewGuid();
    try
    {
        await FetchTgFile(fileId, taskId);
        PdfToImages(taskId);
        await SendImageCollections(taskId, message.Chat);
    }
    catch (Exception ex)
    {
        logger.LogError("Error processing file {TaskId}: {Error}", taskId, ex.Message);
        await bot.SendMessage(message.Chat, "An error occurred while processing your file.");
    }
    finally
    {
        Cleanup(taskId);
    }
}

async Task FetchTgFile(string fileId, Guid taskId)
{
    logger.LogDebug("Fetching file {FileId} from Telegram", fileId);
    var tgFile = await bot.GetFile(fileId);
    var localFilePath = Path.Combine(tempDirPath, $"{taskId}.pdf");
    await using var fileStream = new FileStream(localFilePath, FileMode.Create);
    await bot.DownloadFile(tgFile, fileStream);
}

void PdfToImages(Guid taskId)
{
    logger.LogDebug("Converting PDF {taskId} to images", taskId);
    MagickNET.SetTempDirectory(tempDirPath);
    var settings = new MagickReadSettings { Density = new Density(200, 200) };
    using var images = new MagickImageCollection();
    var imagesPath = Path.Combine(tempDirPath, $"{taskId}.pdf");

    logger.LogDebug("Reading PDF from {ImagesPath}", imagesPath);
    images.Read(imagesPath, settings);

    var outputDir = Path.Combine(tempDirPath, taskId.ToString());
    logger.LogDebug("Output directory for images: {OutputDir}", outputDir);

    if (!Directory.Exists(outputDir))
    {
        Directory.CreateDirectory(outputDir);
    }

    var page = 1;
    foreach (var image in images)
    {
        image.Format = MagickFormat.Jpg;
        image.Write(Path.Combine(outputDir, $"page_{page}.jpg"));
        page++;
    }
}

async Task SendImageCollections(Guid taskId, Chat chat)
{
    logger.LogDebug("Sending image collection for file {taskId}", taskId);
    var outputDir = Path.Combine(tempDirPath, taskId.ToString());
    var files = Directory.GetFiles(outputDir, "*.jpg");
    if (files.Length == 0)
    {
        return;
    }

    var mediaGroup = new List<FileStream>(files.Length);
    mediaGroup.AddRange(files.Select(file => new FileStream(file, FileMode.Open, FileAccess.Read)));

    try
    {
        await bot.SendMediaGroup(chat,
            mediaGroup.Select(fs => new InputMediaPhoto(fs)).ToList());
    }
    catch (Exception ex)
    {
        logger.LogError("Error sending media group for file {taskId}: {Error}", taskId, ex.Message);
    }
    finally
    {
        foreach (var file in mediaGroup)
        {
            file.Close();
        }
    }
}

void Cleanup(Guid taskId)
{
    logger.LogDebug("Cleaning up temporary files for file {taskId}", taskId);
    var tempDir = Path.Combine(Environment.CurrentDirectory, "temp");
    var pdfPath = Path.Combine(tempDir, $"{taskId}.pdf");
    if (File.Exists(pdfPath))
    {
        File.Delete(pdfPath);
    }

    var outputDir = Path.Combine(tempDir, taskId.ToString());
    if (Directory.Exists(outputDir))
    {
        Directory.Delete(outputDir, true);
    }
}
