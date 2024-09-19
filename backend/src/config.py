from pydantic_settings import BaseSettings, SettingsConfigDict
from typing import ClassVar, Dict
import ssl

class Settings(BaseSettings):
    supabase_users_url: str
    supabase_users_public_key: str
    supabase_users_jwt: str

    CELERY_BROKER_URL: str
    CELERY_RESULT_BACKEND: str
    CELERY_BROKER_USE_SSL: ClassVar[Dict[str, ssl.VerifyMode]] = {
        'ssl_cert_reqs': ssl.CERT_NONE
    }
    CELERY_REDIS_BACKEND_USE_SSL: ClassVar[Dict[str, ssl.VerifyMode]] = {
        'ssl_cert_reqs': ssl.CERT_NONE
    }
    
    model_config = SettingsConfigDict(env_file=".env")