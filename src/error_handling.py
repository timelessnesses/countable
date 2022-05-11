import traceback
from difflib import get_close_matches

import discord
from discord.ext import commands


class Errors(commands.Cog):
    def __init__(self, bot: commands.Bot) -> None:
        self.bot = bot

    @commands.Cog.listener()
    async def on_command_error(self, ctx: commands.Context, error: Exception) -> None:
        traceback.print_exception(type(error), error, error.__traceback__)
        if isinstance(error, commands.CommandNotFound):
            matches = get_close_matches(ctx.invoked_with, ctx.bot.commands)
            if len(matches) >= 2:
                await ctx.send(
                    embed=discord.Embed(
                        title="Did you mean...",
                        description=f"{', '.join(matches)} or {matches[0]}?",
                        color=discord.Color.yellow(),
                    )
                )
            elif len(matches) == 1:
                await ctx.send(
                    embed=discord.Embed(
                        title="Did you mean...",
                        description=f"{matches[0]}?",
                        color=discord.Color.yellow(),
                    )
                )
            else:
                await ctx.send(
                    embed=discord.Embed(
                        title="Command not found",
                        description=f"{ctx.invoked_with} is not a valid command.",
                        color=discord.Color.red(),
                    )
                )
        elif isinstance(error, commands.MissingRequiredArgument):
            await ctx.send(
                embed=discord.Embed(
                    title="Missing argument",
                    description=f"{ctx.invoked_with} requires {error.param.name}.",
                    color=discord.Color.red(),
                )
            )
        elif isinstance(error, commands.BadArgument):
            await ctx.send(
                embed=discord.Embed(
                    title="Bad argument",
                    description=f"{error.param.name} is not a valid.\n```py\nBadArgument: {str(error)}\n```",
                    color=discord.Color.red(),
                )
            )
        elif isinstance(error, commands.NotOwner):
            pass
        elif isinstance(error, commands.CheckFailure):
            await ctx.send(
                embed=discord.Embed(
                    title="Permission denied",
                    description=f"You do not have the required permissions to use {ctx.invoked_with}.",
                    color=discord.Color.red(),
                )
            )
        elif isinstance(error, commands.MissingPermissions):
            await ctx.send(
                embed=discord.Embed(
                    title="Access denied",
                    description=f"I do not have permission to do that.",
                    color=discord.Color.red(),
                )
            )

        else:
            exception = "".join(
                traceback.format_exception(type(error), error, error.__traceback__)
            )
            await ctx.send(
                embed=discord.Embed(
                    title="An error occurred",
                    description=f"```py\n{exception}\n```",
                    color=discord.Color.red(),
                )
            )


async def setup(bot: commands.Bot) -> None:
    await bot.add_cog(Errors(bot))
