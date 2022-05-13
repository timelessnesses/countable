from discord.ext import commands


class Leaderboard(commands.Cog):
    def __init__(self, bot: commands.Bot) -> None:
        self.bot = bot

    @commands.hybrid_group()
    async def leaderboard(self, ctx: commands.Context) -> None:
        """
        Leaderboard command group
        """

    @leaderboard.command()
    async def by_chain_count(self, ctx: commands.Bot):
        """
        Get every server's alphabet chain count ranked by most to the least in top 10!
        """
        servers = await self.bot.db.fetch("SELECT * FROM counting")
        servers = sorted(servers, key=lambda x: x["chain_count"], reverse=True)
