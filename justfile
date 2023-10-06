# Justfile for a Cargo workspace
export DOCKER_BUILD_TAG := "test_app"

export RUST_ENV := "development"
export PORT := "8080"

# Run the Rust application
run:
  cargo watch -x run

# Process styles with pnpm and tailwindcss
styles:
  pnpm dlx tailwindcss -i ./styles/tailwind.css -o ./assets/main.css --watch

# Prepare the project with cargo-chef
chef:
  cargo +nightly chef prepare --recipe-path recipe.json

# Build a Docker image
docker-build:
  docker build . \
    --build-arg RUST_ENV=$RUST_ENV \
    -t $DOCKER_BUILD_TAG

# Run a Docker container
docker-run:
  docker run -d -e PORT=${PORT} -p ${PORT}:${PORT} --name ${DOCKER_BUILD_TAG} -it ${DOCKER_BUILD_TAG}

# Stop a Docker container
docker-stop:
  docker stop $DOCKER_BUILD_TAG

# Start a Docker container
docker-start:
  docker start $DOCKER_BUILD_TAG

# Remove a Docker container
docker-rm:
  docker rm ${DOCKER_BUILD_TAG}

# Refresh Docker containers (stop, remove, build, and run)
docker-refresh: docker-stop docker-rm docker-build docker-run
