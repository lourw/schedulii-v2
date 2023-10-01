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
	cargo clippy

lint_fix_backend:
	$(CD_BACKEND) && \
	cargo clippy --fix

lint_frontend:
	$(CD_FRONTEND) && \
	pnpm run lint && pnpm run check-format

lint_fix_frontend:
	$(CD_FRONTEND) && \
	pnpm run lint-fix && pnpm run format

setup_backend:
	$(CD_BACKEND) && \
	cargo install --path .

setup_frontend:
	$(CD_FRONTEND) && \
	pnpm install

setup: setup_backend setup_frontend
	echo "Setting up pre-commit hook..." && \
	cp scripts/pre-commit .git/hooks && \
	chmod +x .git/hooks/pre-commit && \
	echo "Finished installing pre-commit hook"
	
