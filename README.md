# kube-stuff

Quick test stuff for docker and kubernetes

## Running the Rust Container

### Prerequisites
* cargo (with the x86_64-unknown-linux-musl target installed) - optional if you don't mind using the prebuilt binary
* docker

To run the docker container with Rust:
* Go into the rust-stuff directory
* Build the Rust binary: `cargo build --release --target x86_64-unknown-linux-musl`
* Run the script to build the image, run it, and then destroy it: `./run-docker.sh`
