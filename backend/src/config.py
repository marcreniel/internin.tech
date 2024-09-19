from pydantic_settings import BaseSettings, SettingsConfigDict
from typing import ClassVar, Dict
import ssl

class Settings(BaseSettings):
    supabase_users_url: str
    supabase_users_public_key: str
    supabase_users_jwt: str

    supabase_jobs_url: str
    supabase_jobs_public_key: str
    supabase_jobs_service_key: str

    CELERY_BROKER_URL: str
    CELERY_RESULT_BACKEND: str
    CELERY_BROKER_USE_SSL: ClassVar[Dict[str, ssl.VerifyMode]] = {
        'ssl_cert_reqs': ssl.CERT_NONE
    }
    CELERY_REDIS_BACKEND_USE_SSL: ClassVar[Dict[str, ssl.VerifyMode]] = {
        'ssl_cert_reqs': ssl.CERT_NONE
    }
    
    langchain_tracing_v2: str
    langchain_endpoint: str
    langchain_api_key: str
    langchain_project: str

    openai_api_key: str

    model_config = SettingsConfigDict(env_file=".env")