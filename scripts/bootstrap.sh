#!/bin/bash

# Build the Docker image
docker build -t htmx:0.1 .

# Start the Rust app on the Docker image
docker run -p 8000:8000 htmx:0.1 ./htmx-demo
