FROM python:3.11-slim

COPY . ./app


RUN pip install --no-cache-dir --compile -r /app/requirements.txt
RUN apt-get update && apt-get install -y curl

WORKDIR /app


CMD ["python", "-u", "main.py"]
