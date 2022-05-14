from discord.ext import commands


class Leaderboard(commands.Cog):
    def __init__(self, bot: commands.Bot) -> None:
        self.bot = bot

    def column(self, num: int, res="") -> str:
        return (
            self.column(
                (num - 1) // 26,
                "ABCDEFGHIJKLMNOPQRSTUVWXYZ".lower()[(num - 1) % 26] + res,
            )
            if num > 0
            else res
        )

    def prefix(self, n: int):
        return str(n) + (
            "th"
            if 4 <= n % 100 <= 20
            else {1: "st", 2: "nd", 3: "rd"}.get(n % 10, "th")
        )

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
        servers = sorted(servers, key=lambda x: x["chain_count"], reverse=True)[:10]
        all_guilds = await self.bot.db.fetch("SELECT * FROM counting")
        embed = discord.Embed(title="Top 10 Servers by Chain Count", color=0x00FF00)
        for i, server in enumerate(servers):
            embed.add_field(
                name=f"{self.prefix(i + 1)}. {await self.bot.fetch_guild(server['guild_id'])}",
                value=f"{server['chain_count']} chains. (Currently at character {self.column(server['chain_count'])})",
            )
        for i, server in enumerate(all_guilds):
            if server["guild_id"] == ctx.guild.id:
                break
        embed.set_footer(
            text=f"{ctx.guild} is at currently at {self.prefix(i + 1)} and currently at character {self.column(server['chain_count'])}."
        )
