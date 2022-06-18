IMAGE_TAG ?= wdes/mail-autodiscover-autoconfig:latest

.PHONY: build build-docker push-docker test format

build:
	@cargo build

build-release:
	@cargo build --release

test:
	@cargo test

test-coverage:
	@cargo tarpaulin

validate:
	@xmllint --schema xml/xsd/mobilesync/AutodiscoverRequest.xsd xml/mobilesync/AutodiscoverRequest.xml --noout
	@xmllint --schema xml/xsd/mobilesync/AutodiscoverResponse.xsd templates/xml/autodiscover-mobilesync.xml.tera --noout

format:
	@cargo fmt -- --emit files

build-docker:
	@echo "Build arguments: ${BUILD_ARGS}"
	docker build --pull -f  ./alpine/Dockerfile ./ -t "${IMAGE_TAG}" --build-arg BUILD_DATE="$(shell date -u +"%Y-%m-%dT%H:%M:%SZ")" --build-arg VCS_REF="$(shell git rev-parse HEAD)" --build-arg VERSION="$(shell grep -P -m 1 '^version = ".*"$$' Cargo.toml | cut -d '"' -f 2)"

push-docker:
	@echo "Pushing to ${IMAGE_TAG} in 2sec"
	@sleep 2
	docker push "${IMAGE_TAG}"
