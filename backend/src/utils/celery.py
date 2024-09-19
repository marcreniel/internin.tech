from celery import current_app as current_celery_app
from datetime import timedelta
from src.config import Settings

settings = Settings()

def create_celery():
    celery_app = current_celery_app
    celery_app.config_from_object(settings, namespace="CELERY")

    celery_app.conf.beat_schedule = {
        'scan-unstructured-table': {
            'task': 'src.jobs.tasks.retrieve_unstructured',
            'schedule': timedelta(minutes=30)
        },
    }

    return celery_app