install:
	@echo "Updating rust toolchain"
	rustup update stable
	rustup default stable

rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustfmt --version			#rust code formatter
	rustup --version			#rust toolchain manager
	clippy-driver --version		#rust linter 

format:
	@echo "Formatting all projects with cargo"
	./util/format.sh

lint:
	@echo "Linting all projects with cargo"
	./util/lint.sh

test:
	@echo "Testing all projects with cargo"
	./util/test.sh

alpha:
	@echo "Generating changelog and tag"
	commit-and-tag-version --prerelease alpha

beta:
	@echo "Generating changelog and tag"
	commit-and-tag-version --prerelease beta

minor:
	@echo "Generating changelog and tag"
	commit-and-tag-version --release-as minor

release:
	@echo "Generating changelog and tag"
	commit-and-tag-version

all: format lint test
