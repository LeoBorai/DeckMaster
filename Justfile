default:
	@echo "No default task. Available tasks: build, run, test, fmt, clippy, clean"

claude:
    docker compose -f dev/docker-compose.yml up --build --detach

claude-down:
	docker compose -f dev/docker-compose.yml down
