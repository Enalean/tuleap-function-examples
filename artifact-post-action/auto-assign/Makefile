SHELL=/usr/bin/env bash -euo pipefail -c

.PHONY: prepare
prepare:
	pnpm install --frozen-lockfile

.PHONY: tests
tests:
	pnpm run typecheck
	pnpm run test

.PHONY: build
build:
	pnpm run build
