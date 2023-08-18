# KU-Courses

![Example of KU-Courses](frontend/static/assets/og-image.png "KU Courses")

The entire application is governed through the `docker-compose.yml` file and is built with `docker compose`:

## Starting the application
* STEP 1: Install `docker` and `docker-compose`, this may need a restart of your system since Docker is a very low level program.
* STEP 2: Run `docker-compose up --build` as either a user with permissions to docker, or with `sudo`/`doas`, the build flag is required if the backend or frontend code has been changed, additionally `-d` will make it detach from the terminal.
* STEP 3: ???
* STEP 4: PROFIT!!! 
Docker may appear to hang on the `scraper` container, this is to be expected since it is downloading 5 courses a second of the ~3.5k and it may take up to 15 minutes to finish scraping since we want to be nice to KU's surprisingly fragile IT infrastructure.  

### db-manager
Our backend is built with Clojure, a functional programmering language based on Lisp which runs on the Java Virtual Machine.  
This part of the project is responsible for gluing our web-scraper together with the frontend and the database. Furthermore, it is responsible for constructing our SQL queries that are served via the API.

### scraper
The "scraper" confusingly runs the `parser`, It was previously responsible for scraping all of the HTMLs and caching them for future use, the parser will convert all the HTMLs into JSON so they can be inserted into the database. The real scraper is now integrated into `db-manager`

*TODO:*
This entire module is strongly overdue a complete rework into a lighter language (Rust, Haskell, Clojure) since the module is bigger than the entire project combined when built, and takes a long time to parse HTMLs.

### Frontend
Frontend is built in Svelte/Typescript.


### Statistics
Another module written in Clojure, this module is currently not automatically started by the `docker-compose` and should be run manually. It is responsible for scraping STADS for the statistics on all courses to add them to the returned JSONs from `db-manager`.
<!--
### PgAdmin
We've included a container running PgAdmin to provide a GUI for interacting 
with our relational data. By default it will be started up on port 5050.

At first run, it won't automatically connect to the postgres instance. To 
connect to it right click on 'servers' in the left navbar and select
register -> server. In the pop-up give it a name, and then navigate to the 
connection panel, here set the host name to equal the container name,
in our case "disku_pgdb" (as seen in the docker-compose), and point it at 
port 5432. Remember to provide it with the credentials "admin" "admin", such
that it gains access. You should now be ready to go!.

### Extracting files from PgAdmin
To extract the queries from the docker container, create a directory called queries, and run the following command:
```
sudo docker cp <docker_container_id>:/var/lib/pgadmin/storage/admin_private.org/. queries/
```
OBS: only the first few characters (3 or 4) of the id is nescessary.
The container id can be found via:

```
sudo docker ps -a
```

Conversely to import queries run the command in reverse (assuming the queries
you want to import are in /queries):

```
sudo docker cp queries/. <docker_container_id>:/var/lib/pgadmin/storage/admin_private.org/
```
Alternatively you can run one of two bash scripts `docker_export_script.sh` that extracts the queries from the docker container or `docker_import_script.sh` that imports the queries to the container. Both need the container id to be provided as a command line argument. 

#### Visualizing database schema
You can do a nifty visualization by right clicking on "postgres" under "databases"
and choosing "ERD for database".
-->
