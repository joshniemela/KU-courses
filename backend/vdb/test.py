import os
import json
from sentence_transformers import SentenceTransformer
import re
from sklearn.metrics.pairwise import cosine_similarity

model = SentenceTransformer('multi-qa-MiniLM-L6-cos-v1')
DATA_DIR = "../../data/new_json"

# read the jsons in the data folder
jsons = [f for f in os.listdir(DATA_DIR) if f.endswith(".json")]


"""We want to make the document not exceed the token legnth of the model."""
def split_document(document, max_token_len=512):
    """
    Split a document into multiple documents that are less than the max_token_len.
    """
    # remove html tags
    document = re.sub('<[^<]+>', ' ', document)
    sentences = document.split(".")
    for i in range(len(sentences)):
        if len(sentences[i]) > max_token_len:
            # split on newlines if it has any and append them to the list
            if "\n" in sentences[i]:
                sentences.extend(sentences[i].split("\n"))
                sentences.pop(i)

    return sentences



# get their contents
contents = []
for json_file in jsons:
    with open(os.path.join(DATA_DIR, json_file), "r") as f:
        contents.append(json.load(f))


sentence_title_pairs = []
title_title_pairs = []
for content in contents:
    title = content["title"]
    document = content["description"]["content"]
    coordinators = content["logistics"]["coordinators"]
    coordinators = [c["name"] for c in coordinators]
    sentences = split_document(document)
    for sentence in sentences:
        sentence = sentence.replace("\n", " ")
        sentence = sentence.strip()
        if len(sentence) > 0:
            sentence_title_pairs.append((sentence, title))
    # and add the title itself
    title_title_pairs.append((title, title))
    # and add the coordinators
    #for coordinator in coordinators:
    #    sentence_title_pairs.append((coordinator, title))

for pair in sentence_title_pairs:
    print(f"{pair[0]} - {pair[1]}")
    print("")


def embed_pair(pair):
    sentence, title = pair
    return model.encode(sentence), title

embedded_pairs = []
for i, pair in enumerate(sentence_title_pairs):
    print(f"Embedding {i} of {len(sentence_title_pairs)}")
    embedded_pairs.append(embed_pair(pair))
embedded_title_pairs = []
for i, pair in enumerate(title_title_pairs):
    print(f"Embedding {i} of {len(title_title_pairs)}")
    embedded_title_pairs.append(embed_pair(pair))

# query
def query_store(query):
    query_embedding = model.encode(query)
    # embeddings
    embeddings = [pair[0] for pair in embedded_pairs]
    scores = cosine_similarity([query_embedding], embeddings)[0]
    scores = [(score, pair[1]) for score, pair in zip(scores, embedded_pairs)]
    #
    title_embeddings = [pair[0] for pair in embedded_title_pairs]
    title_scores = cosine_similarity([query_embedding], title_embeddings)[0]
    title_scores = [(score, pair[1]) for score, pair in zip(title_scores, embedded_title_pairs)]
    # since we have multiple sentences per title, we need to group them
    content_scores = {}
    for score, title in scores:
        if title not in content_scores:
            content_scores[title] = []
            content_scores[title].append(score)
    # get the max average of 3
    max_scores = []
    for title, scores in content_scores.items():
        top3 = sorted(scores, reverse=True)[:2]
        avg = sum(top3)/len(top3)
        # combine with title score
        title_score = [score for score, t in title_scores if t == title][0]
        #max_scores.append((avg + title_score, title))
        # instead try taking the max of avg and title score
        titleoravg = "avg" if avg > title_score else "title"
        max_scores.append((max(avg, title_score), titleoravg, title))
    max_scores = sorted(max_scores, reverse=True)
    for score, titleoravg, title in max_scores[:10]:
        print(f"{title} - {titleoravg} - {score}")
    print("")


query_store("koman")

query_store("machine learning")
