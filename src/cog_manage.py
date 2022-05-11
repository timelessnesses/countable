import os

from discord.ext import commands


class CogsManagement(commands.Cog):
    def __init__(self, bot):
        self.bot = bot

    @commands.command(hidden=True)
    @commands.is_owner()
    async def load(self, ctx, *, cog: str):
        """
        Loads a cog.
        """
        try:
            await self.bot.load_extension(cog)
        except Exception as e:
            await ctx.send(f"**`ERROR:`** {type(e).__name__} - {e}")
        else:
            await ctx.send(f"**`SUCCESS`**")

    @commands.command(hidden=True)
    @commands.is_owner()
    async def unload(self, ctx, *, cog: str):
        """
        Unloads a cog.
        """
        try:
            await self.bot.unload_extension(cog)
        except Exception as e:
            await ctx.send(f"**`ERROR:`** {type(e).__name__} - {e}")
        else:
            await ctx.send(f"**`SUCCESS`**")

    @commands.command(hidden=True)
    @commands.is_owner()
    async def reload(self, ctx, *, cog: str):
        """
        Reloads a cog.
        """
        cog = "src." + cog
        try:
            await self.bot.unload_extension(cog)
            await self.bot.load_extension(cog)
        except Exception as e:
            await ctx.send(f"**`ERROR:`** {type(e).__name__} - {e}")
        else:
            await ctx.send(f"**`SUCCESS`**")

    @commands.command(hidden=True)
    @commands.is_owner()
    async def reloadall(self, ctx):
        """
        Reloads all cogs.
        """
        for cog in os.listdir("./src"):
            if cog == "cog_manage.py":
                continue
            if not cog.endswith(".py"):
                continue
            try:
                await self.bot.unload_extension("src." + cog[:-3])
                await self.bot.load_extension("src." + cog[:-3])
            except Exception as e:
                await ctx.send(f"**`ERROR:`** {type(e).__name__} - {e}")
            else:
                await ctx.send(f"**`SUCCESS`**")


async def setup(bot):
    await bot.add_cog(CogsManagement(bot))
