from bs4 import BeautifulSoup
import json
import nltk
from nltk import pos_tag
import re
from scraper import get_page

# this module is responsible for parsing and extracting information from the html pages
dk_to_en_keys = {
    "varighed": "duration",
    "kursuskapacitet": "course capacity",
    "udbydende institutter": "contracting departments",  #
    "contracting department": "contracting departments",  #
    "udbydende institut": "contracting departments",  # These map to the same.
    "studienævn": "study board",
    "kursuskode": "course code",
    "niveau": "level",
    "sprog": "language",
    "Formelle krav": "Formal requirements",
    "skemagruppe": "schedule",
    "undervisere": "lecturers",
    "Anbefalede faglige forudsætninger": "Recommended Academic Qualifications",
    "Arbejdsbelastning": "Workload",
    "Feedbackform": "Feedback form",
    "Bemærkninger": "Remarks",
    "Kursusindhold": "Content",
    "Målbeskrivelser": "Learning Outcome",
    "Undervisningsmateriale": "Literature",
    "kursusansvarlige": "course coordinators",
    "Uddannelse": "Education",
    "placering": "placement",
    "Undervisningsform": "Teaching and learning methods",
    "point": "credit",
    "udbydende fakultet": "contracting faculty",
    "Tilmelding": "Sign up",
}
dk_to_en_faculties = {
    "Det Juridiske Fakultet": "Faculty of Law",
    "Det Humanistiske Fakultet": "Faculty of Humanities",
    "Det Teologiske Fakultet": "Faculty of Theology",
    "Det Sundhedsvidenskabelige Fakultet": "Faculty of Health and Medical Sciences",
    "Det Natur- og Biovidenskabelige Fakultet": "Faculty of Science",
    "Det Samfundsvidenskabelige Fakultet": "Faculty of Social Sciences",
}


def fixstring(sld):
    if isinstance(sld, str):
        return sld.replace("\n", " ").replace("\xa0", " ").strip()
    if isinstance(sld, list):
        return [fixstring(s) for s in sld]
    if isinstance(sld, dict):
        return {fixstring(key): fixstring(value) for key, value in sld.items()}
    return sld


def snakecase(string):
    return string.lower().replace(" ", "_").replace("-", "_")


def get_panel_info(url: str) -> dict:
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
    # top_elements = {dt.text: dd.text for dt, dd in zip(dl.find_all("dt"), dl.find_all("dd"))} # This is the one that processes schedule
    top_elements = {
        dt.text: checkdiv(dd) for dt, dd in zip(dl.find_all("dt"), dl.find_all("dd"))
    }  # This is the one that processes schedule

    # Grabbing bottom elements
    h5s = panel_body.find_all("h5")
    # For every h5, get the next div
    siblings = [h5.find_next_sibling() for h5 in h5s]

    # if a sibling is a ul, get the li's
    siblings = [s.find_all("li") if s.name == "ul" else s for s in siblings]

    # bottom elements are the 5(?) in the h5s
    def getmail(soup):
        coordinators = []
        for person in soup:
            try:
                # try to find spans
                spans = person.find("span", onclick=True)
                if not spans:
                    pass
                uglymail = spans["onclick"]
                uglymail = uglymail.split("'")[1]
                name = person.find(string=True, recursive=False)
                email = deobfuscate(uglymail)
                coordinators.append(
                    {"full_name": fixstring(name), "email": fixstring(email)}
                )
            except:
                pass
        return coordinators

    coord_names = ("kursusansvarlige", dk_to_en_keys["kursusansvarlige"])
    coord_people = []

    bottom_elements = {}

    for i, h5 in enumerate(h5s):
        key = h5.text.lower()
        if key in coord_names:
            bottom_elements[key] = getmail(siblings[i])
        else:
            bottom_elements[key] = [sibling.text.strip() for sibling in siblings[i]]

    final_dict = {
        "URL": url,
        **top_elements,
        **bottom_elements,
        "last-modified": panel_body.find("div", class_="last-modified").text,
    }

    # Lowercase all keys
    return {k.lower(): v for k, v in final_dict.items()}


def get_course_items(url: str) -> dict:
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

    out_dict["course content"] = main_content.find(
        "div", id="course-content"
    ).text.strip()

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
        return {
            dt.text.strip(): dd.text.strip()
            for dt, dd in zip(exams.find_all("dt"), exams.find_all("dd"))
        }

    out_dict["exams"] = grab_exam_table(main_content)

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
    value = [process_course_item(tag) for tag in tags]

    # find all text in the div itself
    div_text = div.find_all(text=True, recursive=False)

    # append text to value
    value += [" " + text.strip() for text in div_text if text.strip() != ""]

    return {key: value}


def checkdiv(tag):
    divs = tag.find_all("div")
    if divs:
        return "__DIV__".join((d.text or "").strip() for d in divs)
    else:
        return tag.text.strip()


def process_course_item(course_soup):
    match course_soup.name:
        case "p":
            return course_soup.text.strip()
        case "h5":
            return course_soup.text.strip()
        case "ul":
            return [li.text.strip() for li in course_soup.find_all("li")]
        case "dl":
            return {
                dt.text.strip(): checkdiv(dd)
                for dt, dd in zip(
                    course_soup.find_all("dt"), course_soup.find_all("dd")
                )
            }  # TODO FIX THIS: this calling .text collapses listed structures such as stacks of divs into a single text, 2 exams get merged etc.
        case "a":
            return course_soup.text.strip()
        case "div":
            return course_soup.text.strip()


def get_course_items2(url: str) -> dict:
    # Find a div with regex class * main-content
    soup = BeautifulSoup(get_page(url), "html.parser")
    main_content = soup.find("div", class_=lambda x: x and "main-content" in x)

    out_dict = {}
    out_dict["primary title"] = main_content.find("h1").text.strip()

    course_items = main_content.find_all("div", class_="course-item")

    course_items = [process_course_div(course_item) for course_item in course_items]
    # append every dict to out_dict
    for course_item in course_items:
        out_dict = {**out_dict, **course_item}

    return out_dict


def rename_examkey(course):
    # Correctly name exam key
    for key in course.keys():
        if (key.startswith("Exam")) or key.startswith("Eksa"):
            examkey = key
            oldexam = course[key]
    del course[examkey]
    course["Exam"] = oldexam
    return course


def rename_exam_subkey(course):
    # Translate exam keys
    dk_en_exam_dict = {
        "Reeksamen": "Re-exam",
        "Hjælpemidler": "Aid",
        "Eksamensperiode": "Exam period",
        "Bedømmelsesform": "Marking scale",
        "Prøveformsdetaljer": "Type of assessment details",
        "Prøveform": "Type of assessment",
        "Krav til indstilling til eksamen": "Exam registration requirements",
        "Point": "Credit",
        "Censurform": "Censorship form",
    }
    if isinstance(course["Exam"], dict):
        keylist = list(course["Exam"].keys())
        for key in keylist:
            if key in dk_en_exam_dict.keys():
                thisvalue = course["Exam"][key]
                del course["Exam"][key]
                course["Exam"][dk_en_exam_dict[key]] = thisvalue
    return course


## Få workload fra en liste med en liste med key value pairs (efter header "category", "hours")
import string


def dictify_workload(course):
    workdict = {}
    worklist = course["Workload"][0][2:]
    while worklist:
        key = worklist.pop(0)
        value = worklist.pop(0)
        workdict[key] = float(value.replace(",", "."))
    course["Workload"] = workdict
    return course


def translate_workkeys(course):
    workload_dictionary = {
        "E-læring": "E-Learning",
        "Eksamen": "Exam",
        "Laboratorie": "Laboratory",
        "Studiegrupper": "Study Groups",
        "Teoretiske øvelser": "Theory exercises",
        "Feltarbejde": "Field Work",
        "Forberedelse (anslået)": "Preparation",
        "Eksamensforberedelse": "Exam Preparation",
        "Ekskursioner": "Excursions",
        "Forelæsninger": "Lectures",
        "Praktiske øvelser": "Practical exercises",
        "Projektarbejde": "Project work",
        "Øvelser": "Exercises",
        "Vejledning": "Guidance",
        "Holdundervisning": "Class Instruction",
        "Praktik": "Practical Training",
        "I alt": "Total",
    }  # Seminar is Seminar

    keylist = list(course["Workload"].keys())
    for key in keylist:
        if key in workload_dictionary.keys():
            thisvalue = course["Workload"][key]
            del course["Workload"][key]
            course["Workload"][workload_dictionary[key]] = thisvalue

    # Now workload keys are normalised. Next, let's snakecase them:
    keylist = list(course["Workload"].keys())
    for key in keylist:
        thisvalue = course["Workload"][key]
        del course["Workload"][key]
        course["Workload"][snakecase(key)] = thisvalue

    return course


def fix_primary_title(course):
    course["primary title"] = fixstring(course["primary title"][11:])
    return course


def normalise_language(c):
    # normalize language codes from
    # {'Dansk', 'Engelsk', 'English', 'English - Partially in Danish'}
    # to 'en' or 'da'
    l = c["language"].lower()
    if l.startswith("da"):
        c["language"] = "da"
    elif l.startswith("en"):
        c["language"] = "en"
    return c


def tidy_content(c):
    def replace_chars(lst):
        while None in lst:
            lst.remove(None)
        while "" in lst:
            lst.remove("")
        for i in range(len(lst)):
            if isinstance(lst[i], list):
                replace_chars(lst[i])
            elif isinstance(lst[i], str):
                lst[i] = fixstring(lst[i])
            else:
                print("undetermined content type", type(lst[i]))
                print(lst[i])
        return lst

    c["Content"] = replace_chars(c["Content"])
    c["Learning Outcome"] = replace_chars(c["Learning Outcome"])
    return c


def floatify_credit(c):
    l = c["credit"].lower()
    l = fixstring(l).replace(",", ".").replace(" ects", "")
    l = float(l)
    c["credit"] = l
    return c


def ba_or_ma(c):
    l = c["level"].lower()
    if "master" in l or "kandidat" in l:
        c["study_level"] = "Master"
    elif "bachelor" in l:
        c["study_level"] = "Bachelor"
    else:
        raise ValueError(f"Course Level undetermined: {l}")
    del c["level"]
    return c


def rename_topkeys(c):
    keytrans = {
        "course code": "course_id",
        "primary title": "title",
        "language": "course_language",
        "Content": "description",
        "credit": "credits",
    }
    for k, v in keytrans.items():
        value = c[k]
        del c[k]
        c[v] = value
    return c


def normalise_examkeys(c):
    # Let's take the exam out of list:
    val = c["Exam"][0]
    c["exam"] = val
    del c["Exam"]

    dk_en_exam_dict = {
        "Reeksamen": "Re-exam",
        "Hjælpemidler": "Aid",
        "Eksamensperiode": "Exam period",
        "Bedømmelsesform": "Marking scale",
        "Prøveformsdetaljer": "Type of assessment details",
        "Prøveform": "Type of assessment",
        "Krav til indstilling til eksamen": "Exam registration requirements",
        "Point": "Credit",
        "Censurform": "Censorship form",
    }
    # Translating the exam keys:
    keylist = list(c["exam"].keys())
    for key in keylist:
        if key in dk_en_exam_dict.keys():
            thisvalue = c["exam"][key]
            del c["exam"][key]
            c["exam"][dk_en_exam_dict[key]] = thisvalue
    return c


def type_of_assessmentfixer(c):
    def convert_to_minutes(s):
        # dictionary to store units and their conversion to minutes
        time_units = {
            "(min|minutes|minutter|minuts?)\.?": 1,
            "(h|hour|timer|time)\.?": 60,
            "(d|day|dage)\.?": 24 * 60,
            "(w|week|uger)\.?": 24 * 7 * 60,
        }
        # dictionary to convert words to numbers
        word_to_num = {
            "one": 1,
            "two": 2,
            "three": 3,
            "four": 4,
            "five": 5,
            "six": 6,
            "seven": 7,
            "eight": 8,
            "nine": 9,
            "ten": 10,
        }
        # regex to handle multipliers and decimals
        match_decimal = re.search(r"(\d+\.\d+)", s)
        match_mul_hour = re.search(r"(\d+)\*(\d+)", s)
        match_gange = re.search(r"(\d+) gange (\d+)", s)

        if match_decimal:
            return round(float(match_decimal.group(1)) * 60)
        elif match_mul_hour:
            return round(
                int(match_mul_hour.group(1)) * int(match_mul_hour.group(2)) * 60
            )
        elif match_gange:
            return round(int(match_gange.group(1)) * int(match_gange.group(2)))
        else:
            for unit, factor in time_units.items():
                # regex for numerical and textual number with unit
                match_num_unit = re.search(rf"(\d+) ?{unit}", s)
                match_word_unit = re.search(
                    rf'(?i)({"|".join(word_to_num.keys())}) ?{unit}', s
                )
                if match_num_unit:
                    return round(int(match_num_unit.group(1)) * factor)
                elif match_word_unit:
                    return round(word_to_num[match_word_unit.group(1).lower()] * factor)

            # If only a number is given, assume it's minutes
            match_num = re.search(r"^(\d+)$", s)
            if match_num:
                return round(int(match_num.group(1)))

        return None

    examtrans = {
        "Mundtlig prøve": "Oral examination",
        "Skriftlig prøve": "Written examination",
        "Skriftlig aflevering": "Written assignment",
        "Løbende bedømmelse": "Continuous assessment",
        "Praktisk skriftlig prøve": "Practical written examination",
        "Praktisk mundtlig prøve": "Practical oral examination",  # findes ikke på engelsk
        "Løbende bedømmelse med opsyn.": "Continuous assessment",
        "Continuous assessment under invigilation": "Continuous assessment",
        "Praktisk mundtlig prøve med opsyn.": "Practical oral examination",  # findes ikke på engelsk
        "Mundtligt forsvar": "Oral defence",
        # Other og Portfolio er engelsk
    }

    examlist = c["exam"]["Type of assessment"].split("__DIV__")
    for i, e in enumerate(examlist):
        exam_time = e.split(",")
        if exam_time[0] in examtrans.keys():
            exam_time[0] = examtrans[exam_time[0]]
        if len(exam_time) > 1:
            exam_time[1] = convert_to_minutes(
                fixstring(exam_time[1]).strip()
            )  # These are the times
        examlist[i] = exam_time[:2]
    c["exam"]["Type of assessment"] = examlist

    # snakecase exam types
    newlist = []
    for e in c["exam"]["Type of assessment"]:
        e[0] = snakecase(e[0])
        newlist += [e]
    c["exam"]["Type of assessment"] = newlist
    # exams done!!
    return c


def course_start_and_duration(c):
    def extract_starting_block(s):
        # Convert to lowercase for consistency
        s = s.lower()
        s = fixstring(s)

        # translate some Danish words
        danish_to_english = {
            "blok": "block",
            "forår": "spring",
            "sommer": "summer",
            "efterår": "autumn",
            "eller": "or",
        }
        for dk, en in danish_to_english.items():
            s = s.replace(dk, en)

        # Regex pattern to find block numbers
        pattern = re.compile(r"block (\d)")

        # Regex pattern to find multioption coursestarts inside parenthesis
        parenthesis_pattern = re.compile(
            r"\(block ((\d+\+\d+, )*(\d+\+\d+ or \d+\+\d+))\)"
        )

        p_matches = parenthesis_pattern.findall(s)
        if p_matches:  # Multioption start detected. Starttime ambiguous
            return None
        # Find all block numbers
        matches = pattern.findall(s)

        if matches:
            return int(matches[0])

        # If no block numbers, map semester names to starting block
        semester_to_block = {"spring": 3, "summer": 5, "autumn": 1}

        # For each semester name
        for semester, block in semester_to_block.items():
            # If the semester name is in the string, return the associated block
            if semester in s:
                return block

        return None

    def extract_duration(s):
        # Convert to lowercase for consistency
        s = s.lower()
        # Replace all newline characters and multiple whitespaces with a single space
        s = re.sub(r"\s+", " ", s)
        # Replace "blok" with "block" for consistency
        s = re.sub(r"blok", "block", s)
        # Remove all non-alphanumeric characters except comma and space
        s = re.sub(r"[^a-z0-9, ]", "", s)

        # Try to extract block number
        match = re.search(r"(\d) block", s)
        if match:
            return int(match.group(1))

        # If there's no block number, check for semesters
        match = re.search(r"(\d) semester", s)
        if match:
            return int(match.group(1)) * 2

        return None

    if "duration" in c.keys():
        c["duration"] = extract_duration(c["duration"])
    if "placement" in c.keys():
        c["start_block"] = extract_starting_block(c["placement"])
        del c["placement"]
    return c


def fix_schedule_group(c):
    def extract_schedule_groups(l):
        s = l.split("__DIV__")[0]
        s = fixstring(s)
        # Find all normal BCD
        matches = []
        matches += re.findall(r"\b([BCD])\d?\b", s)
        s = re.sub(r"\b([BCD])\d?\b", "", s)

        # Find A2 with digits
        matches += re.findall(r"\b(A)\d\b", s)
        s = re.sub(r"\b(A)\d\b", "", s)

        # Find A with punctuation
        matches += re.findall(r"\b(A)[.,!?]", s)
        s = re.sub(r"\b(A)[.,!?]", "", s)

        # Find A with word afterwards:
        matches += re.findall(r"\b(A)\s([a-z]+)", s)
        s = re.sub(r"\b(A)\s[a-z]+", "", s)

        # I think I covered my bases. Let's find the A's and the following word
        matches += re.findall(r"\b(A)\b", s)
        s = re.sub(r"\b(A)\b", "", s)

        # Now let's check if for any tuples, and accept if the last word is conjunction, preposition, or an unknown word
        w = nltk.corpus.words.words()
        accepted_classes = ["ADP", "CONJ"]
        were_good = False
        for i, match in enumerate(matches):
            if isinstance(match, tuple):
                # print('found', match)
                letter, word = match
                _, tag = pos_tag([word], tagset="universal")[0]
                if tag in accepted_classes:
                    were_good = True
                if tag == "NOUN":
                    if word not in w:
                        were_good = True
                if were_good:
                    matches[i] = letter
                    # print('accepting')
                else:
                    del matches[i]
                    # print('rejecting')
        m = set(matches)
        return sorted(list(m))

    if "schedule" in c.keys():
        c["schedule_group"] = extract_schedule_groups(c["schedule"])
        del c["schedule"]
    return c


def final_cleanup(c):
    # fix exam:
    final_list = []
    exams = c["exam"]["Type of assessment"]
    for exam in exams:
        this_exam = {}
        if exam:
            this_exam["exam_type"] = exam.pop(0)
        if exam:
            if exam[0]:
                this_exam["minutes"] = exam.pop(0)
        final_list.append(this_exam)
    c["exam"] = final_list

    # fix workload:
    final_list = []
    workloads: dict = c["Workload"]
    for typ, dur in workloads.items():
        if typ != "total":
            final_list.append({"workload_type": typ, "hours": dur})
    c["Workload"] = final_list

    # remove None duration
    if "duration" in c.keys():
        if not c["duration"]:
            del c["duration"]
    # also remove None from start_block
    if "start_block" in c.keys():
        if not c["start_block"]:
            del c["start_block"]

    # Let's rename stuff
    def renamekey(course: dict, fromthis: str, tothis: str) -> dict:
        if fromthis in course.keys():
            v = course[fromthis]
            del course[fromthis]
            c[tothis] = v
        return course

    rename = [
        ("course code", "course_id"),
        ("primary title", "title"),
        ("language", "course_language"),
        ("Content", "description"),
        ("Workload", "workloads"),
        ("credit", "credits"),
        ("course coordinators", "coordinators"),
        ("schedule_group", "schedules"),
        ("exam", "exams"),
    ]

    for fromthis, tothis in rename:
        c = renamekey(c, fromthis, tothis)

    # Some helper functions for description manipulation
    def flatten(lst):
        if isinstance(lst, list):
            return " ".join([flatten(item) for item in lst if item is not None])
        elif isinstance(lst, str):
            return lst

    def flatten_and_format(data, depthlist, depth=0):
        result = []
        for element in data:
            if isinstance(element, str):
                result.append({"type": depthlist[depth], "string": element})
            elif isinstance(element, list):
                nested_result = flatten_and_format(element, depthlist, depth=depth + 1)
                result.extend(nested_result)
        return result

    def remove_none_elements(dictionary):
        for key, value in dictionary.items():
            if isinstance(value, list):
                dictionary[key] = [v for v in value if v is not None]
            elif isinstance(value, dict):
                remove_none_elements(value)
        return dictionary

    relevant_desc = [
        "description",
        "Learning Outcome",
        "Recommended Academic Qualifications",
    ]
    relevant_and_present = list(filter(lambda x: x in c.keys(), relevant_desc))
    c["raw_description"] = flatten([c[v] for v in relevant_and_present])

    depthlist = ["p", "li", "li_two", "li_three", "li_four"]
    merged = {}
    for d in relevant_and_present:
        merged[snakecase(d)] = c[d]
        del c[d]
    full_description = fixstring(remove_none_elements(merged))
    description = {
        key: flatten_and_format(value, depthlist)
        for key, value in full_description.items()
    }
    c["description"] = json.dumps(description)

    # every element in schedules should be a dict
    if "schedules" in c.keys():
        c["schedules"] = [{"schedule_type": s} for s in c["schedules"]]

    return c


def get_all_info(url):
    site = {**get_panel_info(url), **get_course_items2(url)}
    # Translate keys to english
    site = {dk_to_en_keys.get(k, k): v for k, v in site.items()}

    # Translate faculties to english
    faculty = site["contracting faculty"][0]

    # only attempt to translate if the danish_name is in the translation dictionary
    if faculty in dk_to_en_faculties:
        english_name = dk_to_en_faculties[faculty]
        site["contracting faculty"] = english_name
    else:
        site["contracting faculty"] = faculty

    # Only keep the science dept:
    if not site["contracting faculty"] == "Faculty of Science":
        return None

    pipeline = [
        rename_examkey,
        rename_exam_subkey,
        dictify_workload,
        translate_workkeys,
        fix_primary_title,
        normalise_language,
        tidy_content,
        floatify_credit,
        ba_or_ma,
        normalise_examkeys,
        type_of_assessmentfixer,
        course_start_and_duration,
        fix_schedule_group,
        final_cleanup,
    ]

    for func in pipeline:
        site = func(site)
    return site


# THIS IS USED TO DEOBFUSCATE TAGS IN COURSE COORDINATORS
def attempt_deobfuscate(s, mod, offset):
    s = s.split("-")
    if len(s) == 1:
        return s
    m = (len(s[1]) // 2) % mod + offset
    p = ""
    for i in range(0, len(s[1]), 2):
        # convert two hex digits and subtract by m
        value = int(s[1][i : i + 2], 16) - m
        p += chr(value)
    return p


def deobfuscate(mail):
    pattern = "(.+@.+\..+)"
    for i in range(1, 20):
        for j in range(1, 20):
            attempt_combination = attempt_deobfuscate(mail, i, j)
            if re.fullmatch(pattern, attempt_combination):
                return attempt_combination
    return None
