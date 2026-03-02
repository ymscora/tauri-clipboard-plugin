# Publishing Guide

## English

### Package Names

- Rust crate: `tauri-plugin-clipboard-pro`
- NPM package: `tauri-plugin-clipboard-pro-api`

### Pre-release Checklist

1. Update versions:
- `Cargo.toml` -> `[package].version`
- `package.json` -> `version`
  Or use one command:

```bash
./scripts/release.sh 0.1.1 --no-publish
```

2. Ensure Tauri major/minor alignment in docs and example:
- Rust `tauri = 2.10.x`
- App `@tauri-apps/api = 2.10.x`
- App `@tauri-apps/cli = 2.10.x`

3. Run checks:

```bash
cargo fmt
cargo test
cargo test --test real_scenarios -- --ignored
npm run build:npm
```

### Publish to crates.io

```bash
cargo login
cargo publish
```

### Publish to npm

```bash
npm login
npm publish --access public
```

### One-click release script

```bash
./scripts/release.sh 0.1.1
```

Useful flags:

- `--dry-run`: print commands only
- `--no-verify`: skip `cargo test` and `npm run build:npm`
- `--no-publish`: sync versions and checks only
- `--no-github-release`: skip GitHub release creation
- `--no-github-packages`: skip GitHub Packages publishing
- `--github-repo owner/repo`: explicitly set GitHub repository

By default, the script will:

1. publish crates.io + npm
2. publish GitHub Packages (requires `NODE_AUTH_TOKEN`)
3. create tag `vX.Y.Z`
4. `git push` that tag
5. create a GitHub Release via `gh release create`

### GitHub Packages token

Set token before running release:

```bash
export NODE_AUTH_TOKEN=YOUR_GITHUB_PAT
```

Required scopes:

- `write:packages`
- `read:packages`
- `repo` (if repository/packages are private)

### Install and login gh CLI

macOS:

```bash
brew install gh
gh auth login
gh auth status
```

### Tag and Release

```bash
git tag vX.Y.Z
git push origin vX.Y.Z
```

---

## 简体中文

### 包名

- Rust Crate：`tauri-plugin-clipboard-pro`
- npm 包：`tauri-plugin-clipboard-pro-api`

### 发布前检查

1. 同步版本号：
- `Cargo.toml` 的 `[package].version`
- `package.json` 的 `version`
  或直接执行：

```bash
./scripts/release.sh 0.1.1 --no-publish
```

2. 确认 Tauri 主/次版本一致：
- Rust `tauri = 2.10.x`
- 应用侧 `@tauri-apps/api = 2.10.x`
- 应用侧 `@tauri-apps/cli = 2.10.x`

3. 执行检查：

```bash
cargo fmt
cargo test
cargo test --test real_scenarios -- --ignored
npm run build:npm
```

### 发布到 crates.io

```bash
cargo login
cargo publish
```

### 发布到 npm

```bash
npm login
npm publish --access public
```

### 一键发布脚本

```bash
./scripts/release.sh 0.1.1
```

常用参数：

- `--dry-run`：只打印命令，不实际执行
- `--no-verify`：跳过 `cargo test` 和 `npm run build:npm`
- `--no-publish`：仅同步版本并检查，不发布
- `--no-github-release`：跳过 GitHub Release 创建
- `--no-github-packages`：跳过 GitHub Packages 发布
- `--github-repo owner/repo`：显式指定仓库

脚本默认会自动执行：

1. 发布到 crates.io + npm
2. 发布到 GitHub Packages（需要 `NODE_AUTH_TOKEN`）
3. 打 `vX.Y.Z` 标签
4. 推送标签到 GitHub
5. 使用 `gh release create` 创建 GitHub Release

### GitHub Packages Token

发布前设置：

```bash
export NODE_AUTH_TOKEN=你的_GITHUB_PAT
```

建议权限：

- `write:packages`
- `read:packages`
- `repo`（仓库或包是私有时需要）

### 本机安装并登录 gh

macOS:

```bash
brew install gh
gh auth login
gh auth status
```

### 打标签与发版

```bash
git tag vX.Y.Z
git push origin vX.Y.Z
```
