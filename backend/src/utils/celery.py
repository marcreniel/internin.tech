from celery import current_app as current_celery_app
from datetime import timedelta
from src.config import Settings

settings = Settings()

def create_celery():
    celery_app = current_celery_app
    celery_app.config_from_object(settings, namespace="CELERY")

    # Define the beat schedule for periodic tasks
    celery_app.conf.beat_schedule = {
        'divide-every-hour': {
            'task': 'src.jobs.tasks.divide',  # Ensure this matches your task path
            'schedule': timedelta(seconds=2),  # Every hour at minute 0
            'args': (10, 1),  # Example arguments for the divide task
        },
    }

    return celery_app