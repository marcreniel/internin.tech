from celery import current_app as current_celery_app

from src.config import Settings

def create_celery():
    celery_app = current_celery_app
    celery_app.config_from_object(Settings, namespace="CELERY")

    return celery_app