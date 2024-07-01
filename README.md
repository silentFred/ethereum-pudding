# Pudding

This project is a proof of concept and learning exercise for downloading beacon states and then generating and caching proofs for finalized beacon states. 
The project is built using Rust, Docker Compose and Redis.

### Running the project

To run the project locally > docker compose up

Access the API at http://localhost:9000/proof/state?state_id={STATE_ID}&path={PATH}
Example: > curl "http://localhost:9000/proof/state?state_id=8865728&path=finalized_checkpoint,root"
