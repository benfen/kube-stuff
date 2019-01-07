# kube-stuff

Quick test stuff for docker and kubernetes

## Running the Rust Container

### Prerequisites
* cargo (with the x86_64-unknown-linux-musl target installed) - optional if you don't mind using the prebuilt binary
* docker
* minikube

### Running the container on minikube
* Start minikube (or whatnot) on the machine
* Run `kubectl create -f kube-config/` from the root of the repo
  * To fetch the URL for the service, run `minikube service test-service --url`
