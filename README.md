# Visual Regression Tracker backend service written in Rust

> Just trying out how trivial it would be to use OpenAPI spec and the help from ChatGPT to rebuild the backend in Rust

[![Code Smells](https://sonarcloud.io/api/project_badges/measure?project=paazmaya_vrt-backend-rust-poc&metric=code_smells)](https://sonarcloud.io/summary/new_code?id=paazmaya_vrt-backend-rust-poc)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=paazmaya_vrt-backend-rust-poc&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=paazmaya_vrt-backend-rust-poc)

## Running with Docker

Build and Run: Open a terminal and navigate to your project directory. Run the following command to build and start your Rust application along with the PostgreSQL database:

```sh
docker-compose up
```

Docker Compose will build the Docker image based on the Dockerfile and set up the services defined in the docker-compose.yml file. Your Rust application will be accessible at the specified port (e.g., http://localhost:8000), and the PostgreSQL database will be available at localhost:5432.

Stopping Containers: To stop the containers, press Ctrl+C in the terminal where you ran docker-compose up. If you want to stop and remove the containers, run:

```sh
docker-compose down
```

## License

Apache-2.0