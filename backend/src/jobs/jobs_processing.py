import getpass
from datetime import datetime
from pydantic import BaseModel, Field
from langchain_openai import ChatOpenAI
from langchain_core.output_parsers import PydanticOutputParser
from src.config import Settings

settings = Settings()

settings.langchain_api_key = getpass.getpass()
settings.openai_api_key = getpass.getpass()

class CleanedData(BaseModel):
    id: str = Field(..., description="Unique identifier")
    synopsis: str = Field(..., description="Synopsis of the content")
    tags: str = Field(..., description="Tags associated with the content")
    posted: datetime = Field(..., description="Timestamp when the content was posted")

def retrieve_unstructured() {
    
}