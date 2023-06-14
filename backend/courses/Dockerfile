FROM python:3.11-slim

COPY . ./app

RUN pip install --no-cache-dir --compile -r /app/requirements.txt

WORKDIR /app


CMD ["python", "-u", "main.py"]

