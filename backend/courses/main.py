from parsing import get_all_info
from scraper import get_sitemap_urls, cache_pages, ensure_dir_exists
import os
import json
import multiprocessing as mp
import nltk

nltk_path = "../../data/nltk_resources"
DATA_DIR = "../../data"  # where to store the data
json_dir = f"{DATA_DIR}/json"


def main():
    nltk_packages = ["words", "averaged_perceptron_tagger", "universal_tagset"]
    cache_pages()
    sitemap_urls = get_sitemap_urls()
    ensure_dir_exists(json_dir)
    ensure_dir_exists(nltk_path)
    delete_files_in_directory(json_dir)
    for package in nltk_packages:
        nltk.download(info_or_id=package, download_dir=nltk_path)
    nltk.data.path.append(nltk_path)
    with mp.Pool(8) as p:
        p.map(convert_to_json, sitemap_urls)


def convert_to_json(url: str):
    """
    Converts all the html files in ../data/html to json files in ../data/json
    """
    course = get_all_info(url)
    if course:
        try:
            with open(f"{json_dir}/{url.split('/')[-1]}.json", "w", encoding='utf8') as f:
                json.dump(course, f, ensure_ascii=False)
            print(f"Parsed {url}")
        except Exception as e:
            print(f"Error with {url}")
            print(e)


def delete_files_in_directory(directory):
    for filename in os.listdir(directory):
        file_path = os.path.join(directory, filename)
        if os.path.isfile(file_path):
            os.remove(file_path)


if __name__ == "__main__":
    main()
