# Python server 
This serves as the main hub for our business logic.

## Get started
As we are using pipenv to manage our dependencies, we first run the following
command (assuming pipenv is installed on your machine):
```
pipenv install
```

We then run the following command to start the server:
```
pipenv run uvicorn main:app --reload
```

Which will expose the server on port *8000*.

## Endpoints
### /course_summaries/
Fetches a set of summary information on the courses, designed to be minimal and 
provide the nescessary information for the frontend's quick overview. The data
is structured as an array of JSON objects, that take the form:

```json 
{
    "course_name": string,
    "course_id": string,
    "summary": string,
    "block": integer,
    "schema_group": char,
    "institute": string,
    "faculty": string,
    "level": string
}
```

