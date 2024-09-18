from typing import Annotated
from fastapi import HTTPException, Header
import jwt

from src.config import Settings

settings = Settings()

def validate_jwt(authorization: Annotated[str, Header()]) -> str:
    if not authorization.startswith("Bearer "):
        raise HTTPException(status_code=400, detail="Invalid Authorization")

    access_token = authorization.split(" ")[1]

    if not access_token:
        raise HTTPException(status_code=400, detail="Invalid Authorization")

    try:
        jwt.decode(
            access_token,
            settings.supabase_users_jwt,
            algorithms=["HS256"],
            options={"verify_aud": False, "verify_signature": True},
        )

    except jwt.InvalidSignatureError as e:
        print(f"Error: {e}")
        raise HTTPException(status_code=400, detail="Invalid Authorization")
    else:
        return access_token