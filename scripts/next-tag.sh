#!/usr/bin/env sh
set -eu

if ! git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  echo "当前目录不是 git 仓库，请在仓库内执行。" >&2
  exit 1
fi

branch=$(git rev-parse --abbrev-ref HEAD)
if printf '%s' "$branch" | grep -Eq '^[0-9]+\\.[0-9]+$'; then
  :
else
  echo "当前分支名必须为 X.Y 格式，无法创建 tag：$branch" >&2
  exit 1
fi

major=$(echo "$branch" | cut -d. -f1)
minor=$(echo "$branch" | cut -d. -f2)

latest_tag=$(git tag --merged HEAD --list "v${major}.${minor}.*" --sort=-v:refname | head -n 1)

case "$latest_tag" in
  v[0-9]*.[0-9]*.[0-9]*)
    :
    ;;
  "")
    latest_tag="v${major}.${minor}.0"
    ;;
  *)
    echo "最新 tag 格式不符合 vX.Y.Z：$latest_tag" >&2
    exit 1
    ;;
esac

version=${latest_tag#v}
patch=$(echo "$version" | cut -d. -f3)

if [ "$latest_tag" = "v${major}.${minor}.0" ]; then
  next_tag="$latest_tag"
else
  patch=$((patch + 1))
  next_tag="v${major}.${minor}.${patch}"
fi

echo "最新 tag: $latest_tag"
echo "即将创建: $next_tag"

git tag "$next_tag"
git push origin "$next_tag"

echo "已推送 tag: $next_tag"
