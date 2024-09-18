from pydantic_settings import BaseSettings, SettingsConfigDict

class Settings(BaseSettings):
    supabase_users_url: str
    supabase_users_public_key: str
    supabase_users_jwt: str
    celery_broker_url: str
    celery_result_backend: str
    model_config = SettingsConfigDict(env_file=".env")