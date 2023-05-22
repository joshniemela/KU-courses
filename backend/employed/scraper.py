import os
import time
import requests
from bs4 import BeautifulSoup
import pandas as pd
import re
# this module is responsible for scraping the pages and caching them

DIKU_URL = "https://di.ku.dk/english/staff/"

def deobfuscate_email(soup):
    """
    Deobfuscates an email from a soup
    """
    try:
        onclick = soup.find("a")["onclick"]
        # remove single quotes, + signs and spaces using regex
        onclick = re.sub(r"['+ ]", "", onclick)
        # email is immediately after mailto: and ends at ;
        email = re.search(r"mailto:(.*?);", onclick).group(1)
        return email
    except Exception as e:
        return "ERROR"

def process_tr(tr):
    """
    Processes a tr tag from the staff table
    """
    # Get the name
    tds = tr.find_all("td")
    name = tds[0].text.strip()
    title = tds[1].text.strip()
    phone = tds[2].text.strip()
    email = deobfuscate_email(tds[3])
    return {"name": name, "title": title, "phone": phone, "email": email}

def get_diku_staff(url:str=DIKU_URL):
    """
    Returns a table of all the staff at DIKU, with the following columns:
    - name
    - title
    - phone?
    - email
    """
    soup = BeautifulSoup(requests.get(url).text, "html.parser")
    # find tbody
    tbody = soup.find("tbody")
    
    # make table
    table = pd.DataFrame([process_tr(tr) for tr in tbody.find_all("tr")])
    return table

