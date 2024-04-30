#!/usr/bin/env bash

set -o errexit
set -o nounset

error() {
  printf "%s\n" "$*"
  exit 1
}

usage_error() {
  error "Usage: ${0##*/} $1"
}

success() {
  printf "%s\n" "$1"
  exit 0
}

is_divisible_by() {
  local -i number="$1"
  local -i divider="$2"

  ((number % divider == 0)) && return 0 || return 1
}

is_divisible_by_4() {
  is_divisible_by "$1" 4
}

is_divisible_by_100() {
  is_divisible_by "$1" 100
}

is_divisible_by_400() {
  is_divisible_by "$1" 400
}

main() {
  [[ $# == 1 && $1 =~ ^[0-9]+$ ]] || usage_error "<year>"
  is_divisible_by_4 "$1" && ! is_divisible_by_100 "$1" || is_divisible_by_400 "$1" && success true
  success false
}

main "$@"
