#!/usr/bin/env bash
# Usage:
#   ./mod.sh mod:<module>                                -> src/modules/<module>/mod.rs
#   ./mod.sh mod:<module> <submodule>                    -> src/modules/<module>/<submodule>/mod.rs
#   ./mod.sh file:<name> mod:<module>                    -> src/modules/<module>/<name>.rs
#   ./mod.sh file:<name> mod:<module>:<submodule>        -> src/modules/<module>/<submodule>/<name>.rs
#   ./mod.sh file:<name> mod:<module> adapter:<folder>   -> src/modules/<module>/adapter/<folder>/<name>.rs
#
# Semua yang baru dibuat otomatis didaftarkan (`pub mod <nama>;`) ke mod.rs
# yang selevel dengannya.
set -euo pipefail

MODULES_DIR="${MODULES_DIR:-src/modules}"
ADAPTER_DIR="${ADAPTER_DIR:-adapter}"

err() {
  echo "error: $*" >&2
  exit 1
}

# nama harus identifier Rust yang valid dan bukan keyword `mod`
check_ident() {
  [[ "$1" =~ ^[a-z_][a-z0-9_]*$ && "$1" != "mod" ]] || err "invalid name '$1'"
}

# register <mod.rs> <nama>: tambahkan `pub mod <nama>;` jika belum ada
register() {
  local file="$1" line="pub mod $2;"

  mkdir -p "$(dirname "$file")"
  touch "$file"

  grep -qxF "$line" "$file" && return 0

  # pastikan baris terakhir diakhiri newline sebelum menambah baris baru
  if [[ -s "$file" && -n "$(tail -c1 "$file")" ]]; then
    printf '\n' >> "$file"
  fi
  printf '%s\n' "$line" >> "$file"
  echo "registered '$line' in $file"
}

# ensure_dir <parent-dir> <child>: buat <parent>/<child>/mod.rs jika belum ada
# dan daftarkan ke <parent>/mod.rs
ensure_dir() {
  local parent="$1" child="$2" dir="$1/$2"

  [[ -d "$dir" ]] && return 0
  [[ -e "$dir.rs" ]] && err "$child already exist as a file"

  mkdir -p "$dir"
  touch "$dir/mod.rs"
  echo "created $dir/mod.rs"
  register "$parent/mod.rs" "$child"
}

# mod:<name> [sub]
cmd_mod() {
  local target="$1" sub="${2:-}" k name extra
  IFS=':' read -r k name extra <<< "$target"

  [[ -n "${name:-}" ]]  || err "module name is required"
  [[ -z "${extra:-}" ]] || err "invalid target '$target'"
  check_ident "$name"
  [[ -z "$sub" ]] || check_ident "$sub"

  if [[ -z "$sub" ]]; then
    local dir="$MODULES_DIR/$name"

    [[ -e "$dir" ]] && err "module already exist"

    mkdir -p "$dir"
    touch "$dir/mod.rs"
    echo "created $dir/mod.rs"
    register "$MODULES_DIR/mod.rs" "$name"
  else
    local parent="$MODULES_DIR/$name"
    local dir="$parent/$sub"

    [[ -d "$parent" ]] || err "$name modules does not exist"
    [[ -e "$dir" ]]    && err "$name $sub already exist"

    mkdir -p "$dir"
    touch "$dir/mod.rs"
    echo "created $dir/mod.rs"
    register "$parent/mod.rs" "$sub"
  fi
}

# file:<name> mod:<module>[:<submodule>] [adapter:<folder>]
cmd_file() {
  local fname="${1#file:}" target="$2" third="${3:-}" k m s extra
  local adapter_folder=""

  [[ "$target" == mod:* ]] || err "second argument must be mod:<module>[:<submodule>]"
  IFS=':' read -r k m s extra <<< "$target"

  [[ -n "${m:-}" ]]     || err "module name is required"
  [[ -z "${extra:-}" ]] || err "invalid target '$target'"
  check_ident "$fname"
  check_ident "$m"
  [[ -z "${s:-}" ]] || check_ident "$s"

  if [[ -n "$third" ]]; then
    [[ "$third" == adapter:* ]] || err "invalid argument '$third', expected adapter:<folder>"
    adapter_folder="${third#adapter:}"
    [[ -n "$adapter_folder" ]] || err "adapter folder name is required"
    check_ident "$adapter_folder"
    [[ -z "${s:-}" ]] || err "adapter cannot be combined with a submodule"
  fi

  local dir="$MODULES_DIR/$m"
  [[ -d "$dir" ]] || err "$m modules does not exist"

  if [[ -n "${s:-}" ]]; then
    dir="$dir/$s"
    [[ -d "$dir" ]] || err "$m $s does not exist, use \`adapter:<folder>\` if the file is an adapter"
  fi

  # adapter:<folder> -> <module>/adapter/<folder> (dibuat otomatis jika belum ada)
  if [[ -n "$adapter_folder" ]]; then
    ensure_dir "$dir" "$ADAPTER_DIR"
    ensure_dir "$dir/$ADAPTER_DIR" "$adapter_folder"
    dir="$dir/$ADAPTER_DIR/$adapter_folder"
  fi

  [[ -e "$dir/$fname.rs" ]] && err "file already exist"
  [[ -e "$dir/$fname" ]]    && err "$fname already exist as a module"

  touch "$dir/$fname.rs"
  echo "created $dir/$fname.rs"
  register "$dir/mod.rs" "$fname"
}

case "${1:-}" in
  mod:*)
    [[ $# -le 2 ]] || err "too many arguments"
    cmd_mod "$1" "${2:-}"
    ;;
  file:*)
    [[ $# -ge 2 ]] || err "module target is required: file:<name> mod:<module>[:<submodule>]"
    [[ $# -le 3 ]] || err "too many arguments"
    cmd_file "$1" "$2" "${3:-}"
    ;;
  *)
    err "unknown command '${1:-}'"
    ;;
esac
