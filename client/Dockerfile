# Use a very lightweight base image, like busybox, which is minimalistic
# FROM --platform=linux/amd64 busybox:latest
FROM busybox:latest

# Set the working directory inside the Docker container
WORKDIR /app

# Copy the pre-built dist folder from your project directory into the Docker image
COPY ./dist /app

# This Docker image does nothing by default since its sole purpose is to carry the static files
CMD ["sh"]
