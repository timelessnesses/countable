import discord
from discord.ext import commands
from dotenv import load_dotenv

load_dotenv()
import os
import asyncio
import logging
from sql.sql import EasySQL

logging.getLogger("discord").setLevel(logging.WARNING)  # mute

bot = commands.Bot(command_prefix="a!")
bot.log = logging.getLogger("AlphabetBot")
bot.log.setLevel(logging.INFO)

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

bot.db = asyncio.run(EasySQL().connect(**args))


@bot.event
async def on_ready():
    bot.log.info("Logged in as")
    bot.log.info(bot.user.name)
    bot.log.info(bot.user.id)
    bot.log.info("------")
    await bot.change_presence(game=discord.Game(name="a!help"))
    await bot.tree.sync()


async def main():
    try:
        started = False
        while not started:
            async with bot:
                for extension in os.listdir("src"):
                    if extension.endswith(".py") and not extension.startswith("_"):
                        await bot.load_extension(f"src.{extension[:-3]}")
                        bot.log.info(f"Loaded extension {extension[:-3]}")
                await bot.load_extension("jishaku")
                bot.log.info("Loaded jishaku")
                await bot.start(os.environ["ALPHABET_TOKEN"])
                started = True  # break loop
    except KeyboardInterrupt:
        bot.log.info("Exiting...")
        await bot.db.close()


if __name__ == "__main__":
    asyncio.run(main())
