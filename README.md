# kube-stuff

Quick test stuff for docker and kubernetes

## Running the Rust Container

### Prerequisites
* cargo (with the x86_64-unknown-linux-musl target installed) - optional if you don't mind using the prebuilt binary
* docker
* minikube

### Runnign the container on minikube
* Go into the rust-stuff directory
* Build the Rust binary: `cargo build --release --target x86_64-unknown-linux-musl`
  * Step may be skipped if `cargo` is not installed
* Run the kube script: `./kube.sh`
