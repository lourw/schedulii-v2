CD_BACKEND=cd backend
CD_FRONTEND=cd frontend/schedulii-ui



build_backend:
	$(CD_BACKEND) && \
	cargo build
	
build_frontend:
	$(CD_FRONTEND) && \
	pnpm run build

lint_backend:
	$(CD_BACKEND) && \
	cargo fmt --check && cargo clippy

lint_fix_backend:
	$(CD_BACKEND) && \
	cargo fmt --all && cargo clippy --fix

lint_frontend:
	$(CD_FRONTEND) && \
	pnpm run lint && pnpm run check-format

lint_fix_frontend:
	$(CD_FRONTEND) && \
	pnpm run lint-fix && pnpm run format

pre_commit_hook:
	echo "Setting up pre-commit hook..." && \
	cp assets/scripts/pre-commit .git/hooks && \
	chmod +x .git/hooks/pre-commit && \
	echo "Finished installing pre-commit hook"

setup_backend:
	$(CD_BACKEND) && \
	cargo install --path .

setup_frontend:
	$(CD_FRONTEND) && \
	pnpm install

setup: pre_commit_hook setup_backend setup_frontend

start_app: 
	docker-compose up

stop_app:
	docker-compose down

test_backend:
	$(CD_BACKEND) && \
	cargo test

test_frontend:
	$(CD_FRONTEND) && \
	pnpm run test run
