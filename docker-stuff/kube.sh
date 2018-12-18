#!/bin/sh

# Startup minikube with the default driver
# This script was tested on Ubuntu 16 using the virtualbox vm driver
minikube start

# Convince minikube to use local docker images
eval $(minikube docker-env)

# Build the image for the rust thing
docker build . -t rust-thing:0.0.1 --no-cache

# Create deployment for the image
kubectl run hello --image=rust-thing:0.0.1 --port=8080

echo "\n\nViewing current deployments"
kubectl get pods
echo "\n\n"

# Expose deployment
kubectl expose deployment hello --type=NodePort

# Open the service in the browser.  Does not wait for startup.
minikube service hello