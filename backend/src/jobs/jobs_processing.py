from pydantic import BaseModel, Field
from langchain_openai import ChatOpenAI
from langchain_core.prompts import ChatPromptTemplate
from langchain_core.output_parsers import PydanticOutputParser

from src.config import Settings
from src.utils.supabase import create_service_client

settings = Settings()

class CleanedData(BaseModel):
    id: str = Field(..., description="Link of the job posting")
    title: str = Field(..., description="Title of the job, STRICTLY do not include location and the company name.")
    synopsis: str = Field(..., description="Provide a brief summary of the job posting, if available. If not, return 'No description available'")
    tags: str = Field(..., description="Tags associated with the content")
    posted: str = Field(..., description="Timestamp when the content was posted")

parser = PydanticOutputParser(pydantic_object=CleanedData)

async def retrieve_unstructured():
    client = create_service_client()
    response = client.table("unstructured").select("*").execute()
    
    structured_data = []
    
    llm = ChatOpenAI(api_key=settings.openai_api_key, model="gpt-4o-mini")
    
    message_templates = [
        ("system", "You are a helpful AI bot."),
        ("human", """
        Please convert the following unstructured job posting data into a structured format:
        {format_instructions}

        Data:
        {data}
        """)
    ]
    
    prompt_template = ChatPromptTemplate.from_messages(message_templates)
    
    for record in response.data:
        timestamp = record["updated"]
        
        prompt = prompt_template.format(
            data=record,
            format_instructions=parser.get_format_instructions()
        )
        
        json_response = llm.invoke(prompt)
        
        try:
            cleaned_record = parser.parse(json_response.content)
            cleaned_record.posted = timestamp
            
            print(f"Record parsed: {cleaned_record.id}")
            structured_data.append(cleaned_record.model_dump())
            
        except Exception as e:
            print(f"Failed to parse record: {e}")
    
    if structured_data:
        try:
            insert_response = client.table("structured").upsert(structured_data).execute()
            print(f"Inserted {len(insert_response.data)} records successfully.")
        except Exception as e:
            print(f"Failed to insert records: {e}")
    
    return structured_data