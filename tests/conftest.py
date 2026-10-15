from collections.abc import AsyncIterator

import httpx
import pytest

from tollway.main import create_app


@pytest.fixture
async def client() -> AsyncIterator[httpx.AsyncClient]:
    transport = httpx.ASGITransport(app=create_app())
    async with httpx.AsyncClient(transport=transport, base_url="http://tollway.test") as c:
        yield c
