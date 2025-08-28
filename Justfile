default:
	@echo "No default task. Available tasks: build, run, test, fmt, clippy, clean"

# Builds the TypeScript Client
client-build:
	cd ./src/client && bun run build

# Starts the Claude container
claude:
    docker compose -f dev/docker-compose.yml up --build --detach

# Tear down the Claude container
claude-down:
	docker compose -f dev/docker-compose.yml down

fmt:
	cargo clippy --workspace --fix --allow-dirty --allow-staged && cargo fmt --all
	cd ./src/client && bun run lint
	cd ./src/ui && bun run lint

# Runs the UI Projectn
run-ui:
	cd ./src/ui && bun run dev
