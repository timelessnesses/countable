import datetime

import discord
from discord.ext import commands

from .utils import stuffs


class AlphabetCount(commands.Cog):
    def __init__(self, bot):
        self.bot = bot

    @commands.hybrid_group()
    async def alphabet(self, ctx: commands.Context):
        if ctx.invoked_subcommand is None:
            await ctx.send_help(ctx.command)

    @alphabet.command()
    async def setup(self, ctx: commands.Context):
        is_setupped = await self.bot.db.fetch(
            "SELECT * FROM config WHERE guild_id = $1", ctx.guild.id
        )
        if not is_setupped:
            pass
        elif is_setupped[0]["already_setupped"]:
            view = stuffs.Confirm()
            await ctx.send(
                embed=discord.Embed(
                    title="Already set",
                    description="You have already set up Letter Counting this server. This command will override your previous settings. Do you want to do that?",
                ),
                view=view,
            )
            await view.wait()
            if view.value:
                await self.bot.db.execute(
                    "DELETE FROM config WHERE guild_id = $1", ctx.guild.id
                )
                await ctx.send(
                    embed=discord.Embed(
                        title="Resetted",
                        colour=discord.Colour.green(),
                    )
                )
            else:
                await ctx.send(
                    embed=discord.Embed(
                        title="Cancelled",
                        colour=discord.Colour.red(),
                    )
                )
        await ctx.send(
            embed=discord.Embed(
                title="Letter Counting Setup",
                description="""
                This command will ask you
                1. Mention a channel
                2. Can person counting without checking if it is a same person (true/false)
                """,
                colour=discord.Colour.green(),
            ),
        )
        ask = [
            "channel",
            "same_person",
        ]
        answers = {}
        a = None
        channel = None
        for i in ask:
            a = await ctx.send(
                embed=discord.Embed(
                    title="Please answer {}".format(i), colour=discord.Colour.green()
                )
            )
            answer = await self.bot.wait_for(
                "message", check=lambda m: m.author == ctx.author
            )

            if i == "channel":
                answers[i] = answer.id
                channel = answer
            else:
                answers[i] = True if answer.content.lower() == "true" else False
            await a.delete()
        await self.bot.db.execute(
            """
            INSERT INTO config (guild_id, is_same_person, already_setupped, channel_id)
            VALUES ($1, $2, $3, $4)
            """,
            ctx.guild.id,
            answers["same_person"],
            True,
            answers["channel"],
        )
        await ctx.send(
            embed=discord.Embed(
                title="Setupped",
                description=f"""
                You have setupped this server.
                Now {channel.content} will be enforced to count alphabet.
                """,
                colour=discord.Colour.green(),
            )
        )
        channel = await ctx.guild.fetch_channel(int(channel.content.strip("<#>")))
        message = await channel.send(
            embed=discord.Embed(
                title="Alphabet Count Rules",
                description=f"""
                This channel has been claimed for alphabet count
                Here's some rules
                1. {"You can't chain alphabet count else this server will lose streak." if not answers["same_person"] else "You can chain alphabet count."}
                2. You can't broke counting chain by typing anything other than alphabet else you will lose streak.
                3. You can't make conversation in this channel else this server will lose streak.
                """,
                colour=discord.Colour.green(),
            )
        )
        message2 = await channel.send(
            embed=discord.Embed(
                title="Alphabet Count Howto",
                description="""
                Pattern:
                ```
                A
                B
                ...
                Z
                ```
                But if you reached the end of alphabet, the next format should be something like this
                ```
                AA
                AB
                ...
                AZ
                ```
                or
                ```
                BA
                BB
                ...
                BZ
                ```
                If any of you broke the pattern, the server streak will be ended and ended streak will be logged.
                """,
                colour=discord.Colour.green(),
            )
        )
        await message.pin()
        await message2.pin()

    def column(num, res=""):
        return (
            column((num - 1) // 26, "ABCDEFGHIJKLMNOPQRSTUVWXYZ"[(num - 1) % 26] + res)
            if num > 0
            else res
        )

    @commands.Cog.listener()
    async def on_message(self, message: discord.Message):
        if message.author.bot:
            return
        if message.guild is None:
            return
        if not message.channel.id == await self.bot.db.fetch(
            "SELECT channel_id FROM config WHERE guild_id = $1", message.guild.id
        ):
            return
        # -------------------------------------------------------
        # Check if it is a same person
        previous_counter = await self.bot.db.fetch(
            "SELECT previous_counter FROM counting WHERE guild_id = $1",
            message.guild.id,
        )
        if previous_counter is None:
            return
        if int(previous_counter["previous_counter"]) == message.author.id:
            id = stuffs.random_id()
            now = datetime.datetime.now()
            await message.channel.send(
                embed=discord.Embed(
                    title="You can't chain alphabet count",
                    colour=discord.Colour.red(),
                )
                .add_field(
                    name="Reason",
                    value="You can't chain alphabet count else this server will lose streak.",
                )
                .add_field(
                    name="Log ID",
                    value=id,
                )
                .add_field(
                    name="Ruined by",
                    value=message.author.mention,
                )
                .add_field(
                    name="Time",
                    value=now.strftime("%Y/%m/%d %H:%M:%S"),
                )
                .add_field(
                    name="How to fix",
                    value="You can't chain alphabet count else this server will lose streak.",
                )
            )
            await self.bot.db.execute(
                "INSERT INTO logger (id, guild_id, channel_id, ruined_chain_id, ruiner_id, ruined_jump_url, ruined content, when ruined, reason, previous_chain_id) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
                id,
                message.guild.id,
                message.channel.id,
                message.id,
                message.author.id,
                message.jump_url,
                message.content,
                now,
                "You can't chain alphabet count else this server will lose streak.",
                [a async for a in await message.channel.history(limit=1)][0].id,
            )
            return
        # -------------------------------------------------------
        # Check if it is a chained message
        expect = self.column(int(previous_counter["previous_counter"]) + 1)
        if message.content != expect:
            id = stuffs.random_id()
            now = datetime.datetime.now()
            await message.channel.send(
                embed=discord.Embed(
                     
                    title="You broke the pattern",
                    colour=discord.Colour.red(),
                )
                .add_field(
                    name="Reason",
                    value="You broke the pattern.",
                )
                .add_field(
                    name="Log ID",
                    value=id,
                )
                .add_field(
                    name="Ruined by",
                    value=message.author.mention,
                )
                .add_field(
                    name="Time",
                    value=now.strftime("%Y/%m/%d %H:%M:%S"),
                )
                .add_field(
                    name="How to fix",
                    value="You supposed to type `{}`".format(expect),
                )
            )
            await self.bot.db.execute(
                "INSERT INTO logger (id, guild_id, channel_id, ruined_chain_id, ruiner_id, ruined_jump_url, ruined content, when ruined, reason, previous_chain_id) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
                id,
                message.guild.id,
                message.channel.id,
                message.id,
                message.author.id,
                message.jump_url,
                message.content,
                now,
                "You broke the pattern.",
                [a async for a in await message.channel.history(limit=1)][0].id,
            )
            return
        # -------------------------------------------------------
        # all condition were met so we can count
        await self.bot.db.execute(
            "UPDATE counting SET previous_counter = $1 WHERE guild_id = $2",
            previous_counter["previous_counter"] + 1,
            message.guild.id,
        )
        await message.add_reaction("✅")


async def setup(bot: commands.Bot):
    await bot.add_cog(AlphabetCount(bot))
