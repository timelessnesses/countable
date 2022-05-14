import discord
from discord.ext import commands


class Log(commands.Cog, name="Logs"):
    def __init__(self, bot: commands.Bot) -> None:
        self.bot = bot

    @commands.hybrid_command()
    async def get_log(self, ctx: commands.Context, log_id: str) -> None:
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
    await bot.add_cog(Log(bot))
