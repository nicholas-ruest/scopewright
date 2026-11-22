#!/usr/bin/env bash
set -euo pipefail
owner=nicholas-ruest; repo=scopewright; branch=product/scopewright-2026-11-21
test "$(gh api user --jq .login)" = "$owner" || { echo PUBLISH_DENIED:wrong-owner >&2; exit 71; }
if gh repo view "$owner/$repo" >/dev/null 2>&1; then echo PUBLISH_DENIED:repository-exists >&2; exit 72; fi
test -z "$(git status --porcelain)"; source_sha=$(git rev-parse HEAD); base_sha=$(git rev-list --max-parents=0 HEAD)
test "$(git ls-tree -r --name-only "$base_sha" | wc -l)" -ge 10
gh repo create "$owner/$repo" --public --description "Evidence-bearing AI factory governance and control studio"
remote="https://github.com/$owner/$repo.git"; git remote add origin "$remote"
git push origin "$base_sha:refs/heads/main"; git push origin "$source_sha:refs/heads/$branch"
stage=$(mktemp -d); trap 'rm -rf "$stage"' EXIT
mkdir -p "$stage/product-evidence" "$stage/seed/selection-evidence"
cp -a "${DREAM_MACHINE_EVIDENCE_DIR:?}"/. "$stage/product-evidence/"
cp "${DREAM_MACHINE_SEMANTIC_REVIEW:?}" "$stage/seed/semantic-review.md"
cp "${DREAM_MACHINE_SELECTION_REPORT:?}" "$stage/seed/selection-report.json"
cp -a "${DREAM_MACHINE_SELECTION_EVIDENCE:?}"/. "$stage/seed/selection-evidence/"
asset="scopewright-evidence-$source_sha.tgz"; archive="$stage/$asset"
tar -czf "$archive" -C "$stage" product-evidence seed
tag="evidence-$source_sha"
gh release create "$tag" "$archive#$asset" --repo "$owner/$repo" --target "$branch" --title "Exact-source verification evidence" --notes "Source-bound product evidence and CI seed for $source_sha."
evidence_url="https://github.com/$owner/$repo/releases/download/$tag/$asset"
gh pr create --repo "$owner/$repo" --base main --head "$branch" --title "Build Scopewright complete product" --body "Complete React, typed API, executable middleware, 25-crate Rust backend, seven source-pinned integrations, restart-safe audit evidence, containers and three browser journeys. Human merge required."
printf '{"owner":"%s","repository":"%s","source_sha":"%s","branch":"%s","base_sha":"%s","evidence_url":"%s"}\n' "$owner" "$repo" "$source_sha" "$branch" "$base_sha" "$evidence_url"
