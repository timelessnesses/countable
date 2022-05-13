import discord
from discord.ext import commands

from .utils import stuffs


class setup_(commands.Cog, name="Setup"):
    """
    Alphabet's config command group
    """

    def __init__(self, bot):
        self.bot = bot

    @property
    def display_emoji(self):
        return "🔨"

    @commands.hybrid_group()
    async def alphabet(self, ctx: commands.Context):
        """
        Alphabet's main command group
        """
        if ctx.invoked_subcommand is None:
            await ctx.send_help(ctx.command)

    @alphabet.command()
    async def setup(self, ctx: commands.Context):
        """
        Setup the alphabet counter
        """
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
                answers[i] = int(answer.content.strip("<#>"))
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
        await self.bot.db.execute(
            "INSERT INTO counting (guild_id, count_number, count_channel_id, previous_person) VALUES ($1, $2, $3, $4)",
            ctx.guild.id,
            0,
            channel.id,
            None,
        )

    @alphabet.command()
    async def get_log(self, ctx: commands.Context, log_id: str):
        """
        Get the log of a specific log id
        """
        log = await self.bot.db.fetch(
            "SELECT * FROM logger WHERE guild_id = $1 AND id = $2",
            ctx.guild.id,
            log_id,
        )
        if not log:
            return await ctx.send(
                embed=discord.Embed(
                    title="Log not found",
                    description="Log not found",
                    colour=discord.Colour.red(),
                )
            )
        selected = log[0]
        channel = await ctx.guild.fetch_channel(selected["channel_id"])
        previous_message = await channel.fetch_message(selected["message_id"])
        embed = discord.Embed(
            title="Log",
            description="Here's information of this log",
            colour=discord.Colour.green(),
        )
        embed.add_field(name="Log ID", value=selected["id"], inline=False)
        embed.add_field(
            name="Guild",
            value=ctx.guild.name,
            inline=False,
        )
        embed.add_field(
            name="Channel",
            value=channel.mention,
            inline=False,
        )
        embed.add_field(
            name="Ruined message ID",
            value=selected["ruined_message_id"],
            inline=False,
        )
        embed.add_field(
            name="Ruined message",
            value=selected["ruined_jumpurl"],
            inline=False,
        )
        embed.add_field(
            name="Ruined message author",
            value=await ctx.guild.fetch_member(
                selected["ruined_author_id"]
            ).display_name,
            inline=False,
        )
        embed.add_field(
            name="Ruined message content",
            value=selected["ruined_content"],
            inline=False,
        )
        embed.add_field(
            name="When it was ruined",
            value=selected["when_ruined"],
            inline=False,
        )
        embed.add_field(
            name="Reason",
            value=selected["reason"],
            inline=False,
        )
        embed.add_field(
            name="Previous chain message",
            value=previous_message.jump_url,
            inline=False,
        )

        await ctx.send(embed=embed)


async def setup(bot: commands.Bot):
    await bot.add_cog(setup_(bot))
