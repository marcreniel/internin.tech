from typing import Annotated
from fastapi import Depends
from supabase import create_client, Client

from supabase.client import ClientOptions
from src.auth import validate_jwt
from src.config import Settings

settings = Settings()

async def get_supabase_client(
    access_token: Annotated[str, Depends(validate_jwt)],
) -> Client:
    supabase: Client = create_client(
        settings.supabase_users_url,
        settings.supabase_users_public_key,
        options=ClientOptions(
            persist_session=False,
            auto_refresh_token=False,
        ),
    )

    supabase.auth.set_session(access_token, refresh_token="")
    supabase.postgrest.auth(access_token)

    return supabase