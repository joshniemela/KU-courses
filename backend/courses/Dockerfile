FROM python:3.11-alpine

COPY . ./app

RUN pip install -r /app/requirements.txt

WORKDIR /app


CMD ["python", "-u", "main.py"]

