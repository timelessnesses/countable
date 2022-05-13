import typing

import asyncpg


class EasySQL:
    def __init__(self) -> None:
        self.db = None
        self.cursor = None

    async def connect(self, **kwargs) -> typing.Optional[asyncpg.connection.Connection]:
        self.db = await asyncpg.create_pool(**kwargs)
        return self.db

    async def execute(self, query, *args) -> typing.Optional[dict]:
        async with self.db.acquire() as conn:
            await conn.execute(query, *args)
            return dict(conn)

    async def fetch(self, query, *args) -> typing.Optional[dict]:
        async with self.db.acquire() as conn:
            await conn.execute(query, *args)
            return dict(conn)

    async def close(self) -> None:
        await self.cursor.close()
        await self.db.close()

    async def commit(self) -> None:
        await self.db.commit()

    async def rollback(self) -> None:
        await self.db.rollback()

    async def __aenter__(self) -> "EasySQL":
        return self

    async def __aexit__(self, exc_type, exc_val, exc_tb) -> None:
        await self.close()
