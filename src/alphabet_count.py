import discord
from discord.ext import commands
from .utils import stuffs


class AlphabetCount(commands.Cog):
    def __init__(self, bot):
        self.bot = bot
        self.db = bot.db

    @commands.hybrid_group()
    async def alphabet(self, ctx):
        if ctx.invoked_subcommand is None:
            await ctx.send_help(ctx.command)

    @alphabet.command()
    async def setup(self, ctx):
        is_setupped = await self.db.fetch(
            "SELECT * FROM config WHERE guild_id = $1", ctx.guild.id
        )
        if is_setupped["already_setupped"]:
            view = stuffs.Confirm()
            await ctx.send(
                embed=discord.Embed(
                    title="Already setupped",
                    description="You already setupped this server. This command will override your previous settings. Do you want to do that?",
                ),
                view=view,
            )
            await view.wait()
            if view.value:
                await self.db.execute(
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
                title="Alphabet Count Setup",
                description="""
                This command will ask you
                1. Mention a channel
                """,
                colour=discord.Colour.green(),
            ),
        )
        ask = [
            "channel",
        ]
        answers = {}
        a = None
        for i in ask:
            a = await ctx.send(
                embed=discord.Embed(
                    title="Please Mention {}".format(i), colour=discord.Colour.green()
                )
            )
            answer = await self.bot.wait_for(
                "message", check=lambda m: m.author == ctx.author
            )
            if not isinstance(answer, discord.Role):
                return await ctx.send(
                    embed=discord.Embed(
                        title=f"Please Mention a {i} type properly.",
                        description="The setup will be stopped.",
                        colour=discord.Colour.red(),
                    )
                )
            answers[i] = answer.id
            await a.delete()
        query = """
        INSERT INTO alphabet_count (
        """
