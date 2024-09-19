from fastapi import APIRouter
from src.jobs.jobs_processing import retrieve_unstructured

jobs_router = APIRouter(
    prefix="/jobs",
)

@jobs_router.get("/unstructured")
async def get_unstructured():
    response = await retrieve_unstructured()
    return response