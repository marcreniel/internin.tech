from fastapi import APIRouter
from src.jobs.tasks import retrieve_unstructured

jobs_router = APIRouter(
    prefix="/jobs",
)

@jobs_router.get("/structure-manual")
def get_unstructured():
    return retrieve_unstructured()