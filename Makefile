
PROJ_DIR=$(shell pwd)

VERSION=$(shell cat VERSION)
PUBLIC_API_DOC_OUTPUT=$(PROJ_DIR)/target/api-docs/public-api.html
PRIVATE_API_DOC_OUTPUT=$(PROJ_DIR)/target/api-docs/private-api.html
LIBRARY_DOC_OUTPUT=$(PROJ_DIR)/target/doc/pandemia/index.html
USER?=postgres
DATABASE_TEST_USER?=$(USER)
DATABASE_TEST_URL?=postgresql://$(DATABASE_TEST_USER)@localhost/pandemia_test

docs: prepare lib-docs api-docs

prepare:
	@@mkdir -p target/api-docs

lib-docs:
	@@echo generating library documentation...
	@@cargo doc --package pandemia --no-deps --lib
	@@echo generated: $(LIBRARY_DOC_OUTPUT)

api-docs: prepare api-docs/public-api.md
	@@echo generating API documentation...
	@@python $(PROJ_DIR)/etc/script/gen_api_docs.py
	@@cd api-docs && aglio -i public-api.md -o $(PUBLIC_API_DOC_OUTPUT)
	@@cd api-docs && aglio -i private-api.md -o $(PRIVATE_API_DOC_OUTPUT)
	@@echo generated: $(PUBLIC_API_DOC_OUTPUT)
	@@echo generated: $(PRIVATE_API_DOC_OUTPUT)

clean-api-docs:
	rm -f api-docs/*.txt

fmt:
	cd testkit && cargo fmt
	cd macros/pandemia_proc_macro && cargo fmt
	cargo fmt

test:
	@@echo Testing...
	@@DATABASE_URL=$(DATABASE_TEST_URL) cargo test

test-dev:
	@@echo Testing...
	@@DATABASE_URL=$(DATABASE_TEST_URL) cargo test -- --nocapture

lint:
	@@echo Linting...
	@@cargo clippy

audit:
	@@echo Auditing...
	@@cargo audit

commit:
	@@echo Committing...
	@@make fmt
	@@cargo check
	@@git ci -a

version:
	@@sed -i.bak 's/version = ".*" # auto generated do not edit by hand/version = "$(VERSION)" # auto generated do not edit by hand/' Cargo.toml
	@@cat Cargo.toml | grep version | grep "edit by"

release:
	@@echo Build release mode...
	@@cargo build --release

release-linux:
	@@echo ""
	@@echo Ini akan melakukan build menggunakan Docker, 
	@@echo nantinya output binary bisa ditemukan di target/x86_64-unknown-linux-musl/release
	@@echo Building for musl Linux...
	@@docker run -it --rm -v $(PROJ_DIR):/workdir \
					-v /tmp:/root/.cargo/git \
					-v /tmp:/root/.cargo/registry \
					anvie/rust-musl-build:latest \
					cargo build --release --target=x86_64-unknown-linux-musl

build-web-frontend:
	@@echo Building web frontend...
	cd frontends/pandemia_web && yarn run build
	@@echo Web frontend built.

test-env:
	diesel database reset --database-url $(DATABASE_TEST_URL)
	diesel migration run --database-url $(DATABASE_TEST_URL)

test-env-redo:
	diesel migration redo --database-url $(DATABASE_TEST_URL)

reset-db:
	diesel database reset
	diesel migration run

.PHONY: prepare docs lib-docs api-docs fmt \
		test test-dev lint audit commit \
		release test-env test-env-redo release-linux \
		build-web-frontend \
		reset-db \
		version \
		clean-api-docs


