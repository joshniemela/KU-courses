import os
import json
import chromadb

client = chromadb.Client()

DATA_DIR = "../../data/json"

# read the jsons in the data folder
jsons = [f for f in os.listdir(DATA_DIR) if f.endswith(".json")]

# get their contents
contents = []
for json_file in jsons:
    with open(os.path.join(DATA_DIR, json_file), "r") as f:
        contents.append(json.load(f))
# texts contain the raw_description field of each json
texts = [content["raw_description"] for content in contents]
# metadatas should be the "title": content["title"] of each json
metadatas = [{"title": content["title"]} for content in contents]
ids = [content["course_id"] for content in contents]
# insert the texts and metadatas into the database
collection = client.create_collection("test")

collection.add(
    documents=texts,
    metadatas=metadatas,
    ids = ids
)

results = collection.query(query_texts = ["machine learning"], n_results=10)
