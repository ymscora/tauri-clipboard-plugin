#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

DRY_RUN=0
VERIFY=1
PUBLISH=1
GITHUB_RELEASE=1
PUBLISH_GPR=1
ALLOW_DIRTY=0
AUTO_COMMIT=0
GITHUB_REPO=""
VERSION=""

usage() {
  cat <<USAGE
Usage:
  scripts/release.sh [version] [--dry-run] [--no-verify] [--no-publish] [--no-github-release] [--no-github-packages] [--allow-dirty] [--auto-commit] [--github-repo owner/repo]

Examples:
  scripts/release.sh 0.1.1
  scripts/release.sh 0.1.1 --dry-run
  scripts/release.sh --no-publish
  scripts/release.sh 0.1.1 --github-repo m3/tauri-clipboard-plugin
  scripts/release.sh 0.1.1 --allow-dirty
  scripts/release.sh 0.1.1 --auto-commit

Behavior:
  - Syncs version in Cargo.toml ([package].version) and package.json (version)
  - Optionally runs verification (cargo test + npm run build:npm)
  - Optionally publishes to crates.io and npmjs
  - Optionally publishes to GitHub Packages (npm.pkg.github.com)
  - Optionally creates GitHub Release via gh CLI
USAGE
}

log() {
  printf '[release] %s\n' "$*"
}

run_cmd() {
  if [[ "$DRY_RUN" -eq 1 ]]; then
    printf '[dry-run] %s\n' "$*"
  else
    eval "$@"
  fi
}

detect_github_repo() {
  local remote_url
  remote_url="$(git remote get-url origin 2>/dev/null || true)"
  if [[ -z "$remote_url" ]]; then
    return 1
  fi
  if [[ "$remote_url" =~ ^https://github.com/([^/]+)/([^/.]+)(\\.git)?$ ]]; then
    printf '%s/%s' "${BASH_REMATCH[1]}" "${BASH_REMATCH[2]}"
    return 0
  fi
  if [[ "$remote_url" =~ ^git@github.com:([^/]+)/([^/.]+)(\\.git)?$ ]]; then
    printf '%s/%s' "${BASH_REMATCH[1]}" "${BASH_REMATCH[2]}"
    return 0
  fi
  return 1
}

current_package_name() {
  node -e 'const fs=require("fs");const p=JSON.parse(fs.readFileSync("package.json","utf8"));console.log(p.name)'
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    -h|--help)
      usage
      exit 0
      ;;
    --dry-run)
      DRY_RUN=1
      shift
      ;;
    --no-verify)
      VERIFY=0
      shift
      ;;
    --no-publish)
      PUBLISH=0
      shift
      ;;
    --no-github-release)
      GITHUB_RELEASE=0
      shift
      ;;
    --no-github-packages)
      PUBLISH_GPR=0
      shift
      ;;
    --allow-dirty)
      ALLOW_DIRTY=1
      shift
      ;;
    --auto-commit)
      AUTO_COMMIT=1
      shift
      ;;
    --github-repo)
      shift
      if [[ $# -eq 0 ]]; then
        echo "--github-repo requires a value like owner/repo" >&2
        exit 1
      fi
      GITHUB_REPO="$1"
      shift
      ;;
    *)
      if [[ -z "$VERSION" ]]; then
        VERSION="$1"
        shift
      else
        echo "Unknown argument: $1" >&2
        usage
        exit 1
      fi
      ;;
  esac
done

if [[ -z "$VERSION" ]]; then
  VERSION="$(node -e "const fs=require('fs');const p=JSON.parse(fs.readFileSync('package.json','utf8'));const [a,b,c]=String(p.version).split('.').map(Number);console.log([a,b,c+1].join('.'));")"
  log "No version provided, auto bump patch to: $VERSION"
fi

if ! [[ "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+([-.][0-9A-Za-z.-]+)?$ ]]; then
  echo "Invalid version: $VERSION" >&2
  exit 1
fi

if [[ -z "$GITHUB_REPO" ]]; then
  GITHUB_REPO="$(detect_github_repo || true)"
fi

log "Target version: $VERSION"

log "Sync Cargo.toml version"
run_cmd "VERSION='$VERSION' perl -0777 -i -pe 's/(\\[package\\][\\s\\S]*?\\nversion = \")[^\"]+(\")/\$1\$ENV{VERSION}\$2/s' Cargo.toml"

log "Sync package.json version"
run_cmd "node -e 'const fs=require(\"fs\");const p=JSON.parse(fs.readFileSync(\"package.json\",\"utf8\"));p.version=\"$VERSION\";fs.writeFileSync(\"package.json\",JSON.stringify(p,null,2)+\"\\n\");'"

log "Versions after sync"
run_cmd "node -e 'const fs=require(\"fs\");const p=JSON.parse(fs.readFileSync(\"package.json\",\"utf8\"));console.log(\"package.json:\",p.version)'"
run_cmd "if command -v rg >/dev/null 2>&1; then rg -n '^version = \"' Cargo.toml | sed -n '1,5p'; else grep -n '^version = \"' Cargo.toml | sed -n '1,5p'; fi"

if [[ "$VERIFY" -eq 1 ]]; then
  log "Run verification"
  run_cmd "cargo test"
  run_cmd "npm run build:npm"
fi

if [[ "$PUBLISH" -eq 1 ]]; then
  if [[ "$AUTO_COMMIT" -eq 1 ]]; then
    if ! git diff --quiet || ! git diff --cached --quiet; then
      log "Auto-commit dirty changes"
      run_cmd "git add -A"
      run_cmd "git commit -m \"chore(release): v$VERSION\""
    else
      log "Auto-commit requested but working tree is clean"
    fi
  fi

  if [[ "$ALLOW_DIRTY" -eq 0 ]]; then
    if ! git diff --quiet || ! git diff --cached --quiet; then
      echo "[release] Working tree is dirty. Commit changes first, use --auto-commit, or use --allow-dirty to override." >&2
      git status --short >&2 || true
      exit 1
    fi
  fi

  log "Publish crate to crates.io"
  if [[ "$ALLOW_DIRTY" -eq 1 ]]; then
    run_cmd "cargo publish --allow-dirty"
  else
    run_cmd "cargo publish"
  fi

  log "Publish JS package to npmjs"
  run_cmd "npm publish --access public"

  if [[ "$PUBLISH_GPR" -eq 1 ]]; then
    if [[ -z "$GITHUB_REPO" ]]; then
      log "Skip GitHub Packages: cannot detect repository, set --github-repo owner/repo"
    elif [[ -z "${NODE_AUTH_TOKEN:-}" ]]; then
      log "Skip GitHub Packages: NODE_AUTH_TOKEN is not set (requires write:packages token)"
    else
      GH_OWNER="${GITHUB_REPO%%/*}"
      ORIG_NAME="$(current_package_name)"
      if [[ "$ORIG_NAME" =~ ^@ ]]; then
        GPR_NAME="$ORIG_NAME"
      else
        GPR_NAME="@$GH_OWNER/$ORIG_NAME"
      fi

      log "Publish JS package to GitHub Packages as $GPR_NAME"
      if [[ "$DRY_RUN" -eq 1 ]]; then
        printf '[dry-run] cp package.json package.json.release.bak\n'
        printf '[dry-run] node -e \"... set package name to %s ...\"\n' "$GPR_NAME"
        printf '[dry-run] create .npmrc.release.tmp (token hidden)\n'
        printf '[dry-run] npm publish --userconfig .npmrc.release.tmp --registry https://npm.pkg.github.com\n'
        printf '[dry-run] restore package.json and remove temp files\n'
      else
        cp package.json package.json.release.bak
        node -e "const fs=require('fs');const p=JSON.parse(fs.readFileSync('package.json','utf8'));p.name='$GPR_NAME';fs.writeFileSync('package.json',JSON.stringify(p,null,2)+'\\n');"
        cat > .npmrc.release.tmp <<NPMRC
@$GH_OWNER:registry=https://npm.pkg.github.com
//npm.pkg.github.com/:_authToken=${NODE_AUTH_TOKEN}
always-auth=true
NPMRC
        npm publish --userconfig .npmrc.release.tmp --registry https://npm.pkg.github.com
        rm -f .npmrc.release.tmp
        mv package.json.release.bak package.json
      fi
    fi
  else
    log "Skip GitHub Packages (requested by --no-github-packages)"
  fi

  log "Create git tag"
  run_cmd "git tag v$VERSION"

  log "Push tag to GitHub"
  run_cmd "git push origin v$VERSION"

  if [[ "$GITHUB_RELEASE" -eq 1 ]]; then
    if [[ -z "$GITHUB_REPO" ]]; then
      log "Skip GitHub release: cannot detect repository, set --github-repo owner/repo"
    elif ! command -v gh >/dev/null 2>&1; then
      log "Skip GitHub release: gh CLI is not installed"
    else
      log "Create GitHub release v$VERSION on $GITHUB_REPO"
      run_cmd "gh release create v$VERSION --repo $GITHUB_REPO --verify-tag --generate-notes --title v$VERSION"
    fi
  else
    log "Skip GitHub release (requested by --no-github-release)"
  fi
else
  log "Skip publishing (requested by --no-publish)"
fi

log "Release flow finished"
