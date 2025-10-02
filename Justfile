# https://just.systems/man/en

set positional-arguments

latest_tag := `echo "pre-alpha-$(git rev-parse --short HEAD)"`
target_release := "x86_64-unknown-linux-musl"

# Lists available commands
default:
	just --list

# Echo latest tag
latest_tag:
	echo {{latest_tag}}

# Starts the Claude container
claude:
    docker compose -f dev/docker-compose.yml up --build --detach

# Tear down the Claude container
claude-down:
	docker compose -f dev/docker-compose.yml down

# Builds the Server binary used in the Docker Image
docker-build-server: ui-build
	cargo zigbuild --target {{target_release}} --release -p deckmaster-server

# Builds the Docker image
docker-build-image: docker-build-server
	mkdir -p ./docker/tmp/
	cp ./data/mtg.sqlite ./docker/tmp/mtg.sqlite
	cp ./target/{{target_release}}/release/deckmaster-server ./docker/tmp/deckmaster-server
	chmod +x ./docker/tmp/deckmaster-server
	docker build -t "deckmaster:{{latest_tag}}" ./docker

# Publishes the Docker image to the GitHub Container Registry
docker-publish-image:
	docker tag deckmaster:{{latest_tag}} ghcr.io/leoborai/deckmaster:{{latest_tag}}
	docker tag deckmaster:{{latest_tag}} ghcr.io/leoborai/deckmaster:latest
	docker push ghcr.io/leoborai/deckmaster:{{latest_tag}}
	docker push ghcr.io/leoborai/deckmaster:latest

docker-run-image: docker-build-image
	docker run \
		-p 7878:7878 \
		-e STORAGE_URL=$STORAGE_URL \
		deckmaster:{{latest_tag}}

fmt:
	cargo clippy --workspace --fix --allow-dirty --allow-staged && cargo fmt --all
	cd ./src/client && bun run lint
	cd ./src/ui && bun run lint

# Runs the UI Projectn
ui-dev:
	cd ./src/ui && bun run dev

# Builds the UI for the DeckMaster project
ui-build:
	cd ./src/ui && bun run build
