from parsing import get_all_info
from scraper import get_sitemap_urls, cache_pages, ensure_dir_exists
import json
import multiprocessing as mp
DATA_DIR = "../../data" # where to store the data
json_dir = f"{DATA_DIR}/json"

def convert_to_json(url:str):
    """
    Converts all the html files in ../data/html to json files in ../data/json
    """
    try:
        with open(f"{json_dir}/{url.split('/')[-1]}.json", "w") as f:
            json.dump(get_all_info(url), f)
            print(f"Parsed {url}")
    except Exception as e:
        print(f"Error with {url}")
        print(e)
        return

    
if __name__=="__main__":
    cache_pages()
    sitemap_urls = get_sitemap_urls()
    ensure_dir_exists(json_dir)
    with mp.Pool(8) as p:
        p.map(convert_to_json, sitemap_urls)
