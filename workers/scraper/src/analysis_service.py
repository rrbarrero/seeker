from dataclasses import dataclass, asdict
from typing import Protocol, List, Optional
import json

@dataclass
class JobAnalysis:
    title: str
    published_at: str
    requirements: List[str]
    summary: str
    stack: List[str]
    salary: Optional[str]

    def to_json(self) -> str:
        return json.dumps(asdict(self), ensure_ascii=False)

from groq import Groq
import os

class AnalysisService(Protocol):
    def analyze(self, text: str, prompt: str) -> JobAnalysis:
        ...

class GroqAnalysisService:
    def __init__(self):
        self.client = Groq(api_key=os.getenv("GROQ_API_TOKEN"))
        self.model = os.getenv("GROQ_MODEL", "llama-3.3-70b-versatile")

    def analyze(self, text: str, prompt: str) -> JobAnalysis:
        """
        Calls Groq API to analyze the job description and return structured JSON.
        """
        formatted_prompt = prompt.format(text=text)
        
        try:
            completion = self.client.chat.completions.create(
                model=self.model,
                messages=[
                    {
                        "role": "system", 
                        "content": "You are a specialized parser for job postings. You must respond ONLY with a valid JSON object matching the requested schema. Use empty strings or lists for missing values."
                    },
                    {"role": "user", "content": formatted_prompt}
                ],
                response_format={"type": "json_object"}
            )
            
            raw_content = completion.choices[0].message.content
            data = json.loads(raw_content)
            
            return JobAnalysis(
                title=data.get("title", ""),
                published_at=data.get("published_at", ""),
                requirements=data.get("requirements", []),
                summary=data.get("summary", ""),
                stack=data.get("stack", []),
                salary=data.get("salary", "")
            )
        except Exception as e:
            # Fallback to empty analysis if LLM fails
            print(f"Error during Groq analysis: {e}")
            return JobAnalysis(
                title="", published_at="", requirements=[], summary="", stack=[], salary=""
            )

class FakeAnalysisService:
    def analyze(self, text: str, prompt: str) -> JobAnalysis:
        """
        Fake implementation that returns a static JobAnalysis object.
        """
        return JobAnalysis(
            title="Software Engineer (Fake)",
            published_at="2026-02-11",
            requirements=["Python", "Rust", "Unit Testing"],
            summary="A high-impact role in a fake environment.",
            stack=["FastAPI", "Postgres", "Docker"],
            salary="€100K - €120K"
        )
