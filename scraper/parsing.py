from bs4 import BeautifulSoup
from scraper import get_page
# this module is responsible for parsing and extracting information from the html pages

def get_panel_info(url:str) -> dict:
    """
    This function attempts to grab:
    - Study board
    - Contracting department
    - Contracting faculty
    - Course coordinator
    -- Lecturers"""
    soup = BeautifulSoup(get_page(url), "html.parser")
    panel_bodies = soup.find_all("div", class_="panel-body")

    # Find the one with the most h5's, this is presumably the one we want
    panel_body = max(panel_bodies, key=lambda x: len(x.find_all("h5")))

    # Grabbing top elements
    dl = panel_body.find("dl", class_="dl-horizontal")
    top_elements = {dt.text: dd.text for dt, dd in zip(dl.find_all("dt"), dl.find_all("dd"))}


    # Grabbing bottom elements
    h5s = panel_body.find_all("h5")
    # For every h5, get the next div
    siblings = [h5.find_next_sibling() for h5 in h5s]

    # if a sibling is a ul, get the li's
    siblings = [s.find_all("li") if s.name == "ul" else s for s in siblings]

    # bottom elements are the 5(?) in the h5s
    def remove_spans(soup):
        """
        Removes all span tags from a soup, used to remove the mailto links
        """
        try:
            for span in soup.find_all("span"):
                span.decompose()
        except AttributeError:
            pass
        return soup

    bottom_elements = {h5.text: [remove_spans(sibling).text.strip() for sibling in siblings[i]] for i, h5 in enumerate(h5s)}
    final_dict = {
        "URL": url,
        **top_elements,
        **bottom_elements,
        "last-modified": panel_body.find("div", class_="last-modified").text}
    
    # Lowercase all keys
    return {k.lower(): v for k, v in final_dict.items()}
    
def get_course_items(url:str) -> dict:
    # Find a div with regex class * main-content
    soup = BeautifulSoup(get_page(url), "html.parser")
    main_content = soup.find("div", class_=lambda x: x and "main-content" in x)
    
    out_dict = {}
    out_dict["primary title"] = main_content.find("h1").text.strip()

    def english_title(soup):
        # If the english title exists, return else return None
        try:
            # it exists in a div with id course-language
            return soup.find("div", id="course-language").text.strip()
        except AttributeError:
            return None
        
    out_dict["english title"] = english_title(main_content)

    out_dict["course content"] = main_content.find("div", id="course-content").text.strip()

    # field is either named course-prerequisites or course-skills
    def grab_prerequisites(soup):
        try:
            return soup.find("div", id="course-skills").text.strip()
        except AttributeError:
            try:
                return soup.find("div", id="course-prerequisites").text.strip()
            except AttributeError:
                return None
        
    out_dict["recommended prerequisites"] = grab_prerequisites(main_content)

    def grab_exam_table(soup):
        exams = soup.find("div", id="course-exams1").find("dl")
        return {dt.text.strip(): dd.text.strip() for dt, dd in zip(exams.find_all("dt"), exams.find_all("dd"))}
    
    out_dict["exams"] =  grab_exam_table(main_content) 


    def grab_course_load(soup):
        course_load = soup.find("div", id="course-load")
        # li's are key-vals and should be zipped
        lis = course_load.find_all("li")
        lis = [li.text.strip() for li in lis]
        # zip every other element
        temp_dict = {k: v for k, v in zip(lis[::2], lis[1::2])}

        return temp_dict

        
    out_dict["course load"] = grab_course_load(main_content)

    return out_dict

def process_course_div(course_soup):
    # find the a tag, this is the name of the section
    key = course_soup.find("a").text.strip()
    # find the div containing the tags
    div = course_soup.find("div")
    # find all the tags
    tags = div.find_all(recursive=False)
    # process
   # print(key)
    value = [process_course_item(tag) for tag in tags]
   # print(value)
    
    # find all text in the div itself
    div_text = div.find_all(text=True, recursive=False)
    
    # append text to value
    value += [text.strip() for text in div_text if text.strip() != ""]

    return {key: value}

def process_course_item(course_soup):
    match course_soup.name:
        case "p":
            return course_soup.text.strip()
        case "h5":
            return course_soup.text.strip()
        case "ul":
            return [li.text.strip() for li in course_soup.find_all("li")]
        case "dl":
            return {dt.text.strip(): dd.text.strip() for dt, dd in zip(course_soup.find_all("dt"), course_soup.find_all("dd"))}
        case "a":
            return course_soup.text.strip()
        case "div":
            return course_soup.text.strip() + " WARNIGN DIV"


def get_course_items2(url:str) -> dict:
    # Find a div with regex class * main-content
    soup = BeautifulSoup(get_page(url), "html.parser")
    main_content = soup.find("div", class_=lambda x: x and "main-content" in x)
    
    out_dict = {}
    out_dict["primary title"] = main_content.find("h1").text.strip()

    course_items = main_content.find_all("div", class_="course-item")
    

    out_dict["course items"] = [process_course_div(course_item) for course_item in course_items]
    
    
    return out_dict



def get_all_info(url):
    return {
        **get_panel_info(url),
        **get_course_items2(url)
    }
