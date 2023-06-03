#!/bin/bash

# Check if the argument is provided
if [ -z "$1" ]; then
  echo "Usage: ./script.sh <docker_container_id>"
  exit 1
fi

# Execute the docker cp command
sudo docker cp "$1":/var/lib/pgadmin/storage/admin_private.org/. queries/

