FROM python:3.9


WORKDIR /app

COPY Pipfile Pipfile.lock ./ 
RUN python -m pip install --upgrade pip 
RUN pip install pipenv && pipenv install --dev --system --deploy

COPY . /app 

EXPOSE 8090

CMD ["uvicorn", "app.main:app", "--host", "0.0.0.0", "--port", "80"]
