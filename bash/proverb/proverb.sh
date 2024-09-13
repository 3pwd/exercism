#!/usr/bin/env bash

set -o errexit
set -o nounset

line() {
  printf "For want of a %s the %s was lost.\n" "$1" "$2"
}

main() {
  (($# == 0)) && exit 0

  local first="$1"

  while [ $# -gt 1 ]; do
    line "$1" "$2"
    shift
  done

  printf "And all for the want of a %s.\n" "$first"
}

main "$@"
