import string

import discord
from discord.ext import commands


class Leaderboard(commands.Cog):
    """
    Get a ranking of the top 10 users/servers!
    """

    def __init__(self, bot: commands.Bot) -> None:
        self.bot = bot

    @property
    def display_emoji(self) -> str:
        return "📊"

    def column(self, num: int, res="") -> str:
        return (
            self.column(
                (num - 1) // 26,
                string.ascii_lowercase[(num - 1) % 26] + res,
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
        Get every server's alphabet chain count that wasn't ruined ranked by most to the least in top 10!
        """
        servers = sorted(
            await self.bot.db.fetch("SELECT * FROM counting"),
            key=lambda x: x["chain_count"],
            reverse=True,
        )[:10]
        all_guilds = sorted(
            await self.bot.db.fetch("SELECT * FROM counting"),
            key=lambda x: x["chain_count"],
            reverse=True,
        )
        embed = discord.Embed(title="Top 10 Servers by Chain Count", color=0x00FF00)
        for i, server in enumerate(servers):
            embed.add_field(
                name=f"{self.prefix(i + 1)}. {await self.bot.fetch_guild(server['guild_id'])}",
                value=f"{server['chain_count']} chains. (Currently at character {self.column(server['chain_count'])})",
            )
        for i, server in enumerate(all_guilds):
            if server["guild_id"] == ctx.guild.id:
                i_ = i
                server_ = server
                break
        try:
            embed.set_footer(
                text=f"{ctx.guild} is at currently at {self.prefix(i_ + 1)} and currently at character {self.column(server_['chain_count'])}."
            )
        except UnboundLocalError:
            pass
        await ctx.send(embed=embed)

    @leaderboard.command()
    async def by_contribution_alphabets(self, ctx: commands.Context):
        """
        Get letters count by alphabet ranked by most to the least in top 10!
        """
        users = sorted(
            await self.bot.db.fetch("SELECT * FROM user_stats"),
            key=lambda x: x["count_number"],
            reverse=True,
        )[:10]
        all_users = sorted(
            await self.bot.db.fetch("SELECT * FROM user_stats"),
            key=lambda x: x["count_number"],
            reverse=True,
        )
        embed = discord.Embed(title="Top 10 Users by Alphabet Count", color=0x00FF00)
        for i, user in enumerate(users):
            embed.add_field(
                name=f"{self.prefix(i + 1)}. {await self.bot.fetch_user(user['user_id'])}",
                value=f"{user['count_number']} letters. (Currently at character {self.column(user['count_number'])})",
            )
        for i, user in enumerate(all_users):
            if user["user_id"] == ctx.author.id:
                i_ = i
                user_ = user
                break
        try:
            embed.set_footer(
                text=f"{ctx.author} is at currently at {self.prefix(i_ + 1)} and currently at character {self.column(user_['count_number'])}."
            )
        except UnboundLocalError:
            pass
        await ctx.send(embed=embed)

    @leaderboard.command()
    async def by_longest_chain(self, ctx: commands.Context):
        """
        Get every server's alphabet chain count that's ruined ranked by most to the least in top 10!
        """
        servers = sorted(
            await self.bot.db.fetch("SELECT * FROM counting"),
            key=lambda x: x["longest_chain"],
            reverse=True,
        )[:10]
        all_guilds = sorted(
            await self.bot.db.fetch("SELECT * FROM counting"),
            key=lambda x: x["longest_chain"],
            reverse=True,
        )
        embed = discord.Embed(title="Top 10 Servers by Longest Chain", color=0x00FF00)
        for i, server in enumerate(servers):
            embed.add_field(
                name=f"{self.prefix(i + 1)}. {await self.bot.fetch_guild(server['guild_id'])}",
                value=f"{server['longest_chain']} characters. (Currently at character {self.column(server['longest_chain'])})",
            )
        for i, server in enumerate(all_guilds):
            if int(server["guild_id"]) == ctx.guild.id:
                i_ = i
                server_ = server
                break
        try:
            embed.set_footer(
                text=f"{ctx.guild} is at currently at {self.prefix(i_ + 1)} and currently at character {self.column(server_['longest_chain'])}."
            )
        except UnboundLocalError:
            pass
        await ctx.send(embed=embed)


async def setup(bot: commands.Bot) -> None:
    await bot.add_cog(Leaderboard(bot))
