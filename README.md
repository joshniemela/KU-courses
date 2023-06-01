# DIS Project

## Starting Docker
The entire application is governed through the `docker-compose.yml` file
present in the root directory. This file specifies each service we spin up, 
and makes these available to eachother on the internal network `dis-network`.
To spin the containers up simply run:

```
docker-compose down && docker-compose up
```

Administrator rights might be nescessary depending on your configuration.

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

## Initialize database
First run types.sql followed by schema.sql

## Visualizing database schema
You can do a nifty visualization by right clicking on "postgres" under "databases"
and choosing "ERD for database".
