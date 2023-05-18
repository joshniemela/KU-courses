from parsing import get_all_info
from scraper import get_sitemap_urls, cache_pages
import json

if __name__=="__main__":
    cache_pages()
    sitemap_urls = get_sitemap_urls()
    
    # TODO: multithread this
    for url in sitemap_urls:
        try:
            # write to json
            with open(f"../data/json/{url.split('/')[-1]}.json", "w") as f:
                json.dump(get_all_info(url), f)
                print(f"Parsed {url}")
        except Exception as e:
            print(f"Error with {url}")
            print(e)
            continue

