CURRENT_VERSION := $(shell sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml)

.PHONY: release release-push version npm-publish

# ── 发布新版本 ──────────────────────────────────────────────────────
# 用法: make release VERSION=0.3.0
release:
	@if [ -z "$(VERSION)" ]; then \
		echo "Usage: make release VERSION=x.y.z"; \
		echo "Current version: $(CURRENT_VERSION)"; \
		exit 1; \
	fi
	@if ! echo "$(VERSION)" | grep -qE '^[0-9]+\.[0-9]+\.[0-9]+$$'; then \
		echo "Error: version must be in semver format (e.g. 0.2.0)"; exit 1; \
	fi
	# 在 Cargo.toml 中写入版本
	sed -i '' 's/^version = ".*"/version = "$(VERSION)"/' Cargo.toml
	# 同步到 package.json
	node scripts/sync-version.js
	# 提交并打 tag
	git add Cargo.toml Cargo.lock package.json
	git commit -m "chore: bump version to v$(VERSION)"
	git tag -a "v$(VERSION)" -m "Release v$(VERSION)"
	@echo ""
	@echo "  Tag v$(VERSION) created."
	@echo ""
	@echo "  Next steps:"
	@echo "    make release-push    # 推送 tag，触发 CI 自动 publish"
	@echo ""

# ── 推送发布 commit + tag 到 GitHub ──────────────────────────────
# GitHub Actions 检测到 tag 后自动执行 npm publish
release-push:
	git push origin --follow-tags
	@echo "  ✅ 已推送。GitHub Actions 会自动发布到 npm。"

# ── 从本地直接发布到 npm（不经过 CI） ─────────────────────────────
# 用法: make npm-publish
npm-publish: node_modules
	# 确保版本与 Cargo.toml 一致
	node scripts/sync-version.js
	# 打包确认
	npm pack --dry-run
	@echo ""
	@read -p "  ⚠️  确认发布? 按 Enter 继续, Ctrl+C 取消" _
	# 发布
	npm publish

node_modules:
	@echo "  node_modules 不存在，跳过（不需要安装依赖即可发布）"
	@true

# ── 查看当前版本 ──────────────────────────────────────────────────
version:
	@echo "$(CURRENT_VERSION)"
