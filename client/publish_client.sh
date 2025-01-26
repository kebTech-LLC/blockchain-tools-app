#! /bin/bash

RETRIES=5
i=1

# Default tag is "latest"
TAG="latest"

# Check if -t argument is provided
while getopts ":t:" opt; do
  case $opt in
    t)
      TAG=$OPTARG
      ;;
    \?)
      echo "Invalid option: -$OPTARG" >&2
      exit 1
      ;;
    :)
      echo "Option -$OPTARG requires an argument." >&2
      exit 1
      ;;
  esac
done

until [ $i -gt $RETRIES ]
do
  if docker build --no-cache --platform=linux/amd64 --tag kebtech/blockchain-tools-client:$TAG --file Dockerfile . ; then
    if docker push kebtech/blockchain-tools-client:$TAG ; then
            # Remove only the image with the specific tag
            docker rmi kebtech/blockchain-tools-client:$TAG
            docker image prune -f --filter "dangling=true"
            docker system prune -f --filter "until=30m" --filter "label=maintainer=kebtech/blockchain-tools-client"
            break
    else
        echo "push failed" 
    fi
        
  else
    echo "build failed, retrying..."
    i=$((i+1))
    sleep 15
  fi
done
