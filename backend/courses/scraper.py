import os
import time
import requests
from bs4 import BeautifulSoup

# this module is responsible for scraping the pages and caching them
# TODO: sitemap contains last-mod, use this to check if the page should be scraped again.
SITEMAP_URL = "https://kurser.ku.dk/sitemap.xml"  # the magic url
DATA_DIR = "../../data"  # where to store the data


# helper functions:
def name_from_url(url: str) -> str:
    return url.split("/")[-1]


def ensure_dir_exists(dir_path: str) -> None:
    os.makedirs(dir_path, exist_ok=True)


# end helper functions


def get_sitemap_urls(url: str = SITEMAP_URL) -> list[str]:
    sitemap = requests.get(url)
    sitemap_soup = BeautifulSoup(sitemap.text, features="xml")
    urls = [url.text for url in sitemap_soup.find_all("loc")][1:]  # first is root
    return urls


def get_page(url: str, data_dir: str = DATA_DIR) -> str:
    name = name_from_url(url)
    file_path = f"{data_dir}/pages/{name}.html"
    ensure_dir_exists(
        os.path.dirname(file_path)
    )  # Create necessary folders if they don't exist
    try:
        with open(file_path, "r") as f:
            return f.read()
    except FileNotFoundError:
        time.sleep(0.20)  # be nice to the server
        page = requests.get(url)
        with open(file_path, "w") as f:
            f.write(page.text)
        return page.text


def cache_pages(data_dir: str = DATA_DIR) -> None:
    ensure_dir_exists(
        f"{data_dir}/pages"
    )  # Create necessary folders if they don't exist
    cached_pages = os.listdir(f"{data_dir}/pages")
    sitemap_urls = get_sitemap_urls(SITEMAP_URL)
    missing = [
        url for url in sitemap_urls if f"{name_from_url(url)}.html" not in cached_pages
    ]
    if len(missing) != 0:
        print(f"Found {len(missing)} new pages, caching...")
    for i, url in enumerate(missing):
        get_page(url, data_dir)
        print(f"Got: {name_from_url(url)} ({i+1}/{len(missing)})", end="\r")
