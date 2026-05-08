CURRENT_VERSION := $(shell sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml)

.PHONY: release release-push version

# Bump version, commit, and tag in one step.
# Usage: make release VERSION=0.2.0
release:
	@if [ -z "$(VERSION)" ]; then \
		echo "Usage: make release VERSION=x.y.z"; \
		echo "Current version: $(CURRENT_VERSION)"; \
		exit 1; \
	fi
	@if ! echo "$(VERSION)" | grep -qE '^[0-9]+\.[0-9]+\.[0-9]+$$'; then \
		echo "Error: version must be in semver format (e.g. 0.2.0)"; exit 1; \
	fi
	# Update Cargo.toml (macOS sed needs -i '')
	sed -i '' 's/^version = ".*"/version = "$(VERSION)"/' Cargo.toml
	# Commit and tag
	git add Cargo.toml Cargo.lock
	git commit -m "chore: bump version to v$(VERSION)"
	git tag -a "v$(VERSION)" -m "Release v$(VERSION)"
	@echo ""
	@echo "  Tag v$(VERSION) created."
	@echo "  Run 'make release-push' to push commit + tag to origin."

# Push the release commit and all tags to origin.
release-push:
	git push origin --follow-tags

# Show current version from Cargo.toml
version:
	@echo "$(CURRENT_VERSION)"
