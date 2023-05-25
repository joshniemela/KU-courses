# IO
This file contains our expected IO/API calls etc...

## FRONTEND
TO COME

## Scraper
INPUT: HTML from web / KU
OUTPUT: HTMLs to `data/`
SIDE-EFFECTS: Update last scraped date in `state.???`

## Parser
INPUT: HTMLs in `data/`
OUTPUT: JSONs in `data/`
SIDE-EFFECTS: Nothing

# CONTROLLER
TO COME

# DB-MANAGER
INPUT: JSONs in HTTP request body, or other query params.
* PUT: Replace existing data in database with content in body
* GET: Find various specific data, this includes: 
    * get-all-names 
    * get-all-ids 
    TO COME (pls add kristian)
* DELETE: Delete stuff that isn't allowed in the database anymore, GDPR etc.

OUTPUT: JSONs in body of responses to HTTP.
EFFECTS: Modifies content in database, updates last modified date.




