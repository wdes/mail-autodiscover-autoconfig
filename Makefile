DOCKER_TAG ?= latest
IMAGE_TAG ?= wdes/mail-autodiscover-autoconfig

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
	@xmllint --noout --schema xml/xsd/mobilesync/AutodiscoverRequest.xsd xml/mobilesync/AutodiscoverRequest.xml
	@xmllint --noout --schema xml/xsd/mobilesync/AutodiscoverResponse.xsd templates/xml/autodiscover-mobilesync.xml.tera

	@xmllint --noout --schema xml/xsd/autodiscover/AutodiscoverRequest.xsd xml/autodiscover/AutodiscoverRequest.xml
	@xmllint --noout --schema xml/xsd/autodiscover/AutodiscoverResponse.xsd xml/autodiscover/AutodiscoverExchangeResponse.xml
	@xmllint --noout --schema xml/xsd/autodiscover/AutodiscoverResponse.xsd xml/autodiscover/AutodiscoverResponse.xml
	@xmllint --noout --schema xml/xsd/autodiscover/AutodiscoverResponse.xsd templates/xml/autodiscover.xml.tera
	@xmllint --noout --schema xml/xsd/autodiscover/AutodiscoverExchangeResponseRedirect.xsd xml/autodiscover/AutodiscoverResponseRedirect.xml
	@xmllint --noout --schema xml/xsd/autodiscover/AutodiscoverResponseError.xsd xml/autodiscover/AutodiscoverResponseError.xml

format:
	@cargo fmt -- --emit files

build-docker:
	@echo "Build arguments: ${BUILD_ARGS}"
	docker build --pull -f  ./alpine/Dockerfile ./ -t "${IMAGE_TAG}:${DOCKER_TAG}" --build-arg BUILD_DATE="$(shell date -u +"%Y-%m-%dT%H:%M:%SZ")" --build-arg VCS_REF="$(shell git rev-parse HEAD)" --build-arg RELEASE_VERSION="$(shell grep -P -m 1 '^version = ".*"$$' Cargo.toml | cut -d '"' -f 2)"

push-docker:
	@echo "Pushing to ${IMAGE_TAG}:${DOCKER_TAG} in 2sec"
	@sleep 2
	docker push "${IMAGE_TAG}:${DOCKER_TAG}"
