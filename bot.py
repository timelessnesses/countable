import discord
from discord.ext import commands
from dotenv import load_dotenv
from watchdog.events import FileSystemEventHandler
from watchdog.observers import Observer

load_dotenv()
import asyncio
import datetime
import logging
import os

import ssl
import subprocess
import traceback

from sql.sql import EasySQL

logging.getLogger("discord").setLevel(logging.WARNING)  # mute

bot = commands.Bot(command_prefix="a!", intents=discord.Intents.all())
logging.basicConfig(level=logging.NOTSET)
log = logging.getLogger("AlphabetBot")
log.setLevel(logging.NOTSET)
observer = Observer()


class FileHandler(FileSystemEventHandler):
    def on_modified(self, event):
        log.info(f"File changed: {event.src_path}")
        if event.src_path.endswith(".py"):
            log.info("Reloading...")
            path = event.src_path.replace("\\", "/").replace("/", ".")[:-3]
            try:
                asyncio.run(bot.reload_extension(path))
                log.info(f"Reloaded {path}")
            except Exception as e:
                log.error(f"Failed to reload {path}")
                log.error(e)
                log.error(traceback.format_exc())


observer.schedule(FileHandler(), path="src", recursive=False)


def get_git_revision_short_hash() -> str:
    return (
        subprocess.check_output(["git", "rev-parse", "--short", "HEAD"])
        .decode("ascii")
        .strip()
    )


def get_version():
    is_updated = (
        subprocess.run(["git", "status", "-uno"], stdout=subprocess.PIPE)
        .stdout.decode("ascii")
        .strip()
    )
    if "up to date" in is_updated:
        is_updated = True
    else:
        is_updated = False

    if is_updated:
        bot.version_ = f"latest ({get_git_revision_short_hash()})"
    else:
        bot.version_ = f"old ({get_git_revision_short_hash()}) - not up to date"


if os.environ.get("ALPHABET_URI"):  # exists
    args = dict(
        dsn=os.environ["ALPHABET_URI"],
    )
else:
    args = dict(
        host=os.environ["ALPHABET_DB_HOST"],
        user=os.environ["ALPHABET_DB_USER"],
        password=os.environ["ALPHABET_DB_PASSWORD"],
        database=os.environ["ALPHABET_DB_NAME"],
    )

ssl_object = ssl.create_default_context()
ssl_object.check_hostname = False
ssl_object.verify_mode = ssl.CERT_NONE
args["ssl"] = ssl_object


@bot.event
async def on_ready():
    log.info("Logged in as")
    log.info(bot.user.name)
    log.info(bot.user.id)
    log.info("------")
    await bot.change_presence(activity=discord.Game(name="a!help"))
    await bot.tree.sync()


async def main():
    try:
        started = False
        while not started:
            async with bot:
                for extension in os.listdir("src"):
                    if extension.endswith(".py") and not extension.startswith("_"):
                        await bot.load_extension(f"src.{extension[:-3]}")
                        log.info(f"Loaded extension {extension[:-3]}")
                await bot.load_extension("jishaku")
                log.info("Loaded jishaku")
                bot.db = await EasySQL().connect(**args)
                log.info("Connected to database")
                await bot.db.execute(open("sql/starter.sql", "r").read())
                log.info("Executed starter sql")
                observer.start()
                log.info("Started file watcher")
                bot.start_time = datetime.datetime.utcnow()
                get_version()
                log.info(
                    f"Started with version {bot.version_} and started at {bot.start_time}"
                )

                await bot.start(os.environ["ALPHABET_TOKEN"])
                started = True  # break loop
    except KeyboardInterrupt:
        log.info("Exiting...")
        await bot.db.close()


if __name__ == "__main__":
    asyncio.run(main())
    observer.stop()
