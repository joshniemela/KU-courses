from fastapi import FastAPI
from decouple import config


PORT = config("BACKEND_PORT")
print("# INITIAlIZING BACKEND #")
print(f'Port: {PORT}')

app = FastAPI()

@app.get("/")
def read_root():
    return f'DISKU Python Server'

@app.get("/courses")
def read_courses():
    return {"Hello": "From courses"}

@app.get("/course_summaries/")
def read_summaries():
    return [
            {
                "course_name": "Diskret Matematik og Formelle Sprog",
                "course_id": "1234abcd",
                "summary": "Ultra short summary here",
                "block": 1,
                "schema_group": "A",
                "institute": "Datalogisk Institut",
                "faculty": "SCIENCE",
                "level": "bachelor"
            },
            {
                "course_name": "Diskret Matematik og Formelle Sprog",
                "course_id": "1234abcd",
                "summary": "Ultra short summary here",
                "block": 1,
                "schema_group": "A",
                "institute": "Datalogisk Institut",
                "faculty": "SCIENCE",
                "level": "bachelor"
            },
            {
                "course_name": "Diskret Matematik og Formelle Sprog",
                "course_id": "1234abcd",
                "summary": "Ultra short summary here",
                "block": 1,
                "schema_group": "A",
                "institute": "Datalogisk Institut",
                "faculty": "SCIENCE",
                "level": "bachelor"
            },
            {
                "course_name": "Diskret Matematik og Formelle Sprog",
                "course_id": "1234abcd",
                "summary": "Ultra short summary here",
                "block": 1,
                "schema_group": "A",
                "institute": "Datalogisk Institut",
                "faculty": "SCIENCE",
                "level": "bachelor"
            },
            {
                "course_name": "Diskret Matematik og Formelle Sprog",
                "course_id": "1234abcd",
                "summary": "Ultra short summary here",
                "block": 1,
                "schema_group": "A",
                "institute": "Datalogisk Institut",
                "faculty": "SCIENCE",
                "level": "bachelor"
            },
            ]
