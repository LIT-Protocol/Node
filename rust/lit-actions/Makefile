CLANG_FORMAT_OPTS := --Werror --style="{BasedOnStyle: Google, ColumnLimit: 120}"

format:
	cargo fmt
	clang-format $(CLANG_FORMAT_OPTS) -i grpc/schema/*.proto

check-format:
	cargo fmt --check
	clang-format $(CLANG_FORMAT_OPTS) --dry-run grpc/schema/*.proto

clippy:
	cargo clippy --all-targets --locked -- -D warnings

test:
	cargo nextest run

docs:
	npm run docs

check-docs: docs
	git diff --exit-code --name-only ./docs || { echo "Error: please run 'make docs' and commit the changes"; exit 1; }

.PHONY: format check-format clippy test docs check-docs
