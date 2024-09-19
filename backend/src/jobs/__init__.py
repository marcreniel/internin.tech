from fastapi import APIRouter

jobs_router = APIRouter(
    prefix="/jobs",
)

from . import tasks