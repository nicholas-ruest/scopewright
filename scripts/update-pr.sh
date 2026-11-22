#!/usr/bin/env bash
set -euo pipefail
owner=nicholas-ruest
repo=scopewright
branch=product/scopewright-2026-11-21
remote="https://github.com/$owner/$repo.git"
test "$(gh api user --jq .login)" = "$owner" || { echo PUBLISH_DENIED:wrong-owner >&2; exit 71; }
test "$(git remote get-url origin)" = "$remote"
test -z "$(git status --porcelain)"
test "$(gh pr view 1 --repo "$owner/$repo" --json state,headRefName,baseRefName --jq '.state + " " + .headRefName + " " + .baseRefName')" = "OPEN $branch main"
source_sha=$(git rev-parse HEAD)
prior_sha=$(git ls-remote origin "refs/heads/$branch" | cut -f1)
test -n "$prior_sha"
git merge-base --is-ancestor "$prior_sha" "$source_sha"
git push origin "$source_sha:refs/heads/$branch"
stage=$(mktemp -d)
trap 'rm -rf "$stage"' EXIT
mkdir -p "$stage/product-evidence" "$stage/seed/selection-evidence"
cp -a "${DREAM_MACHINE_EVIDENCE_DIR:?}"/. "$stage/product-evidence/"
cp "${DREAM_MACHINE_SEMANTIC_REVIEW:?}" "$stage/seed/semantic-review.md"
cp "${DREAM_MACHINE_SELECTION_REPORT:?}" "$stage/seed/selection-report.json"
cp -a "${DREAM_MACHINE_SELECTION_EVIDENCE:?}"/. "$stage/seed/selection-evidence/"
asset="scopewright-evidence-$source_sha.tgz"
tar -czf "$stage/$asset" -C "$stage" product-evidence seed
gh release create "evidence-$source_sha" "$stage/$asset#$asset" --repo "$owner/$repo" --target "$source_sha" --title "Exact-source verification evidence" --notes "Source-bound product evidence and CI seed for $source_sha."
test "$(gh pr view 1 --repo "$owner/$repo" --json headRefOid --jq .headRefOid)" = "$source_sha"
printf 'PR_UPDATE_PASS source_sha=%s prior_sha=%s\n' "$source_sha" "$prior_sha"
