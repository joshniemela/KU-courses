import pytest
import json

# assert the values are correct in one field
def assert_field(field, expected, actual):
    assert field in actual.keys()
    assert actual[field] == expected


def get_json(name):
    with open(f"test_data/{name}.json", "r") as f:
        return json.load(f)
    
def test_LFKB10272U():
    input = get_json("LFKB10272U")


    assert_field("course code", "LFKB10272U", input)
    assert_field("duration", 1, input)
    assert_field("placement", 1, input)
    assert_field("language", "dk", input)
    assert_field("last-modified", 1677538800, input)
    assert_field("credit", 7.5, input)
    assert_field("level", "bachelor", input) # or BSc, not sure
    assert_field("primary title", "LFKB10272U Fagets videnskabsteori -\nlandskabsarkitektur og bydesign", input)
    assert_field("workload", {"Forelæsninger": 36,
                              "Holdundervisning": 18,
                              "Forberedelse": 86,
                              "Teoretiske øvelser": 30,
                              "Praktiske øvelser": 30,
                              "Vejledning": 5,
                              "Eksamen": 1,
                              "I alt": 206}, input)
