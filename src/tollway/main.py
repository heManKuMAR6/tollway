"""FastAPI application factory.

Run with ``uvicorn tollway.main:create_app --factory``.
"""

from fastapi import FastAPI

from tollway import __version__


def create_app() -> FastAPI:
    """Build the tollway ASGI application."""
    app = FastAPI(title="tollway", version=__version__)

    @app.get("/healthz")
    async def healthz() -> dict[str, str]:
        """Liveness: the process is up and serving requests."""
        return {"status": "ok"}

    return app
