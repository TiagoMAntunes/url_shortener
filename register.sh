#!/bin/bash

# Load required variables
if [ -f .env ]
then
  export $(cat .env | xargs)
fi

ADDR=http://localhost:${PORT}

# Login
TOKEN=$(curl -s -X POST -d '{"user":"admin", "pass":"pass"}' ${ADDR}/login)

# Submit
RESULT=$(curl -s -X POST -H 'Content-Type: application/x-www-form-urlencoded' -H "Authorization: Bearer ${TOKEN}" -d "url=${1}" $ADDR)

# Check if error
if [ "$RESULT" = "Not a valid URL" ]; then
    echo $RESULT
else
    echo "$ADDR/$RESULT"
fi