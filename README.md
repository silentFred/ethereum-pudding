# Pudding

Pudding is a proof of concept and learning exercise designed to download beacon states, generate proofs for finalized
beacon states, and cache these proofs. The project leverages Rust, Docker Compose, and Redis to achieve its goals.

## Features

- Download cryptographic proofs for finalized beacon states.
- Pre-emptively download states, generate and cache proofs in the background.
- Dockerized setup for easy deployment and testing.

## Prerequisites

Ensure you have the following installed:

- [Docker](https://docs.docker.com/get-docker/)
- [Docker Compose](https://docs.docker.com/compose/install/)

## Running the Project

To run the project locally, use Docker Compose:

```sh
docker-compose up
```

### API Usage

Once the project is running, you can access the API to generate and retrieve proofs.

##### Endpoint

GET /proof/state

##### Query Parameters

state_id: The ID of the state (or ``head``) you want to retrieve the proof for.
path: The path within the state for which you need the proof. Currently, only ``finalized_checkpoint,root`` is
pre-cached. If you have common paths you need proofs for, please open an issue or submit a pull request.

#### Example Request

```sh
curl "http://localhost:9000/proof/state?state_id=8865728&path=finalized_checkpoint,root"
```

#### Response

```json
{
  "state_id": "8865728",
  "path": "finalized_checkpoint,root",
  "proof": "base64_encoded_proof_data"
}
```

### Contributing

I'm learning how to build Rust projects, so any feedback is appreciated. Contributions are welcome too so please open an
issue or submit a pull request for any improvements or bug fixes.

### License

This project is licensed under the MIT License. See the LICENSE-MIT file for details.