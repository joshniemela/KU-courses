from scraper import get_diku_staff
import json

DATA_DIR = "../../data/"

if __name__ == "__main__":
    staff = get_diku_staff()
    with open(DATA_DIR + "employed.json", "w") as outfile:
        outfile.write(staff.to_json(orient="records"))
