# shellcheck shell=bash

if { [ -t 1 ] || [ -t 2 ]; } && [ -z "${NO_COLOR:-}" ]; then
  GREEN=$'\033[0;32m' YELLOW=$'\033[0;33m' RED=$'\033[0;31m'
  BOLD=$'\033[1m' DIM=$'\033[2m' NC=$'\033[0m'
else
  GREEN="" YELLOW="" RED="" BOLD="" DIM="" NC=""
fi

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT" || exit 1

step() { echo "${YELLOW}${BOLD}==> $*${NC}"; }
ok() { echo "    ${GREEN}·${NC} ${DIM}$*${NC}"; }
passed() { echo "    ${GREEN}$*${NC}"; }

fail() {
  echo "${RED}${BOLD}$*${NC}" >&2
  exit 1
}

need() {
  local tool
  for tool in "$@"; do
    command -v "$tool" >/dev/null 2>&1 || fail "$tool is required and not installed."
  done
}

owned() {
  local -n _dest="$1"
  mapfile -t _dest < <(git ls-files --cached --others --exclude-standard | grep -E "$2" | sort || true)
}

formatting() {
  BEFORE="$(git diff | cksum)"
}

formatted() {
  [ "$(git diff | cksum)" = "$BEFORE" ] ||
    fail "Files were not formatted. They are now; review and stage them."
  passed "Nothing to reformat"
}
