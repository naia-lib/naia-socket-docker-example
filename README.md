# naia-socket-docker-example

An example project demonstrating how to run a UDP or WebRTC naia-socket Server inside of a Docker container, and then be able to connect to that Server from a client application.

#### Required:

- docker: https://docs.docker.com/get-docker/
- docker-compose: https://docs.docker.com/compose/install/
- cargo-make: https://github.com/sagiegurari/cargo-make

### Server:

**Build an image of a UDP ServerSocket app:**

	`make udp-build-docker`

**Build an image of a WebRTC ServerSocket app:**

	`make webrtc-build-docker`

**Run the last built image inside a Docker container:**

	`make serve-docker`

This should start the container and then wait for user-input after which it shuts down the container, for convenience. (May need to press Enter twice)

**Stop a running Docker container:**

	`make stop-docker`

**Attach to a running Docker container and show logs from the running app:**

	`make read-docker-logs`

### Client:

**Run a UDP client: (that will be able to communicate with a UDP server)**

    1. `cd client/wasm_bindgen`
    2. `cargo run`

**Run a WebRTC client on Web using wasm-bindgen: (that will be able to communicate with a WebRTC server)**

    1. `cd client/wasm_bindgen`
    2. `cargo make serve` //this will open a web browser pointing at http://127.0.0.1:4000/

**To run a WebRTC client on Web using miniquad: (that will be able to communicate with a WebRTC server)**

    1. `cd client/miniquad`
    2. `cargo make serve` //this will open a web browser pointing at http://127.0.0.1:4000/

