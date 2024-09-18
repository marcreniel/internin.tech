import uvicorn

from typing import List
from typing_extensions import Annotated
from fastapi import FastAPI, Depends
from fastapi.middleware.cors import CORSMiddleware
from celery import current_app as current_celery_app
from supabase import Client

from src.utils import get_supabase_client, create_celery
from src.auth import validate_jwt
from src.config import Settings

def create_app() -> FastAPI:
    origins = [
        "http://127.0.0.1:3000",
    ]

    app = FastAPI()

    app.celery_app = create_celery()

    app.add_middleware(
        CORSMiddleware,
        allow_origins=origins,
        allow_credentials=True,
        allow_methods=["*"],
        allow_headers=["*"],
    )

    @app.get("/")
    async def home():
        return "Welcome to Ziptern API"

    # Uncomment and implement this route if needed
    # @app.get("/check-auth", dependencies=[Depends(validate_jwt)])
    # async def check_auth(
    #     supabase: Annotated[Client, Depends(get_supabase_client)]
    # ):
    #     # Implement your logic here
    #     return {"msg": "Authenticated"}

    return app

def start():
    """Launched with `poetry run start` at root level"""
    uvicorn.run("src.main:app", host="0.0.0.0", port=8000, reload=True)