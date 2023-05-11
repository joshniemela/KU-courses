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
