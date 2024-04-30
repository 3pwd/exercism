#!/usr/bin/env bash

set -o errexit
set -o nounset

error() {
  printf "%s\n" "$*"
  exit 1
}

usage_error() {
  error "Usage: ${0##*/} <number>"
}

success() {
  printf "%s\n" "$1"
  exit 0
}

are_equal() {
  if (($1 == $2)); then
    success true
  else
    success false
  fi
}

main() {
  [[ $# == 1 && $1 =~ [[:digit:]] ]] || usage_error

  local -i number="$1"
  local -i length=${#number}
  local -i armstrong

  while read -rn 1 digit; do
    armstrong+=$((digit ** length))
  done <<<"$number"

  are_equal "$number" "$armstrong"
}

main "$@"
