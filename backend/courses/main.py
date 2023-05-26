from parsing import get_all_info
from scraper import get_sitemap_urls, cache_pages, ensure_dir_exists
import json
import multiprocessing as mp
DATA_DIR = "../../data" # where to store the data
json_dir = f"{DATA_DIR}/json"
science_dir = f"{DATA_DIR}/json_science"

def main():
    cache_pages()
    sitemap_urls = get_sitemap_urls()
    ensure_dir_exists(json_dir)
    ensure_dir_exists(science_dir)
    with mp.Pool(8) as p:
        p.map(convert_to_json, sitemap_urls)


def convert_to_json(url:str):
    """
    Converts all the html files in ../data/html to json files in ../data/json
    """
    course = get_all_info(url)
    # only match the science dept
    if course['contracting faculty'] == 'Faculty of Science':

        # Correctly name exam key
        for key in course.keys():
            if (key.startswith('Exam')) or key.startswith('Eksa'):
                examkey = key
                oldexam = course[key]
        del course[examkey]
        course['Exam'] = oldexam

        # Translate exam keys
        dk_en_exam_dict = {
            'Reeksamen': 'Re-exam',
            'Hjælpemidler': 'Aid',
            'Eksamensperiode': 'Exam period',
            'Bedømmelsesform': 'Marking scale',
            'Prøveformsdetaljer': 'Type of assessment details',
            'Prøveform': 'Type of assessment',
            'Krav til indstilling til eksamen': 'Exam registration requirements',
            'Point': 'Credit',
            'Censurform': 'Censorship form',
        }
        if isinstance(course['Exam'], dict):
            keylist = list(course['Exam'].keys())
            for key in keylist:
                if key in dk_en_exam_dict.keys():
                    thisvalue = course['Exam'][key]
                    del course['Exam'][key]
                    course['Exam'][dk_en_exam_dict[key]] = thisvalue

        try:
            with open(f"{science_dir}/{url.split('/')[-1]}.json", "w") as f:
                json.dump(course, f)
            print(f"Parsed {url}")
        except Exception as e:
            print(f"Error with {url}")
            print(e)
    else:
        try:
            with open(f"{json_dir}/{url.split('/')[-1]}.json", "w") as f:
                json.dump(course, f)
            print(f"Parsed {url}")
        except Exception as e:
            print(f"Error with {url}")
            print(e)
        return

if __name__ == '__main__':
    main()
