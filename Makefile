IMAGE_TAG ?= wdes/mail-autodiscover-autoconfig
# All: linux/386,linux/amd64,linux/arm/v5,linux/arm/v7,linux/arm64/v8,linux/mips64le,linux/ppc64le,linux/s390x
# Supported by rust (Debian variant, alpine has 2 less): linux/386,linux/amd64,linux/arm/v7,linux/arm64/v8
# Supported by alpine: linux/386,linux/amd64,linux/arm/v6,linux/arm/v7,linux/arm64/v8,linux/ppc64le,linux/s390x
PLATFORM ?= linux/amd64

ACTION ?= load
PROGRESS_MODE ?= plain
EXTRA_ARGS ?=

.PHONY: build build-release test test-coverage validate format docker-build

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

docker-build:
	# https://github.com/docker/buildx#building
	docker buildx build \
		--tag $(IMAGE_TAG) \
		--progress $(PROGRESS_MODE) \
		--platform $(PLATFORM) \
		--build-arg VCS_REF="$(shell git rev-parse HEAD)" \
		--build-arg BUILD_DATE="$(shell date -u +"%Y-%m-%dT%H:%M:%SZ")" \
		--build-arg RELEASE_VERSION="$(shell grep -P -m 1 '^version = ".*"$$' Cargo.toml | cut -d '"' -f 2)" \
		--$(ACTION) \
		$(EXTRA_ARGS) \
		--file docker/Dockerfile \
		./

docker-clean:
	@echo "Cleaning dangling images and build cache..."
	docker image prune -f
	docker builder prune -f
