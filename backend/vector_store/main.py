import os
import json
from sentence_transformers import SentenceTransformer
from fastapi import FastAPI
import re
import uvicorn


def split_document(document, max_token_len=512):
    """
    Split a document into multiple documents that are less than the max_token_len.
    """
    # remove html tags
    document = re.sub("<[^<]+>", " ", document)
    sentences = document.split(".")
    for i in range(len(sentences)):
        if len(sentences[i]) > max_token_len:
            # split on newlines if it has any and append them to the list
            if "\n" in sentences[i]:
                sentences.extend(sentences[i].split("\n"))
                sentences.pop(i)

    return sentences


model = SentenceTransformer("multi-qa-MiniLM-L6-cos-v1")
DATA_DIR = "../../data/new_json"

# read the jsons in the data folder
jsons = [f for f in os.listdir(DATA_DIR) if f.endswith(".json")]
# grab the json contents
courses = []
for json_file in jsons:
    with open(os.path.join(DATA_DIR, json_file), "r") as f:
        courses.append(json.load(f))

id_title_pairs = []
id_content_pairs = []
id_coordination_pairs = []
for course in courses:
    id = course["info"]["id"]
    title = course["title"]
    content = course["description"]["content"]
    coordinators = course["logistics"]["coordinators"]
    coordinators = [c["name"] for c in coordinators]
    sentences = split_document(content)
    for sentence in sentences:
        sentence = sentence.replace("\n", " ")
        sentence = sentence.strip()
        if len(sentence) > 0:
            id_content_pairs.append((id, sentence))

    for coordinator in coordinators:
        id_coordination_pairs.append((id, coordinator))

    id_title_pairs.append((id, title))


def embed_pairs(pairs):
    ids = [p[0] for p in pairs]
    sentences = [p[1] for p in pairs]
    embeddings = model.encode(
        sentences, show_progress_bar=True, normalize_embeddings=True
    )
    # return as pairs
    return list(zip(ids, embeddings))


embedded_content = embed_pairs(id_content_pairs)
embedded_coordinators = embed_pairs(id_coordination_pairs)
embedded_titles = embed_pairs(id_title_pairs)


def query_store(query):
    embedded_query = model.encode(query, normalize_embeddings=True)
    # compute the dot product between the query and the embeddings
    content_scores = [e[1].dot(embedded_query) for e in embedded_content]
    coordinator_scores = [e[1].dot(embedded_query) for e in embedded_coordinators]
    title_scores = [e[1].dot(embedded_query) for e in embedded_titles]
    #
    # we have multiple sentences per title, we want to group these and find the avg of top 3
    content_scores_dict = {}
    for i in range(len(content_scores)):
        id = embedded_content[i][0]
        score = content_scores[i]
        if id in content_scores_dict:
            content_scores_dict[id].append(score)
        else:
            content_scores_dict[id] = [score]
    #
    # find the top 3 scores for each id
    # and compute the average
    top3_content_scores = []
    for id in content_scores_dict:
        scores = content_scores_dict[id]
        scores.sort(reverse=True)
        top3_content_scores.append((id, sum(scores[:2]) / 2))
    # for each course, find the coordinator that closest matches the query
    coordinator_scores_dict = {}
    for i in range(len(coordinator_scores)):
        id = embedded_coordinators[i][0]
        score = coordinator_scores[i]
        if id in coordinator_scores_dict:
            coordinator_scores_dict[id].append(score)
        else:
            coordinator_scores_dict[id] = [score]
    coordinator_scores = []
    for id in coordinator_scores_dict:
        scores = coordinator_scores_dict[id]
        scores.sort(reverse=True)
        coordinator_scores.append(scores[0])
    # for each course, take the max of avg content score, coordinator score and title score
    # print the length of the scores
    print(len(top3_content_scores), len(coordinator_scores), len(title_scores))
    maxed_scores = []
    for i in range(len(top3_content_scores)):
        id = top3_content_scores[i][0]
        content_score = top3_content_scores[i][1]
        coordinator_score = coordinator_scores[i]
        title_score = title_scores[i] * 1.25  # give title more weight
        # print the lengths of these
        if coordinator_score < 0.5:
            coordinator_score = 0
        maxed_scores.append((id, max(content_score, coordinator_score, title_score)))
    # sort the scores
    maxed_scores.sort(key=lambda x: x[1], reverse=True)

    # TODO: this should be done in db-manager since this happens before filtering,
    # aka we might end up only matching the predicates of the entire db on only 100 courses
    return maxed_scores[:150]


app = FastAPI()


@app.get("/search")
async def search(query: str):
    results = query_store(query)
    # get the course ids
    ids = [r[0] for r in results]
    return ids


@app.get("/health")
async def health():
    return "healthy"


if __name__ == "__main__":
    uvicorn.run(app, host="0.0.0.0", port=4000)
