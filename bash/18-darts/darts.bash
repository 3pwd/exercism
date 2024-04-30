#!/usr/bin/env bash

set -o nounset
set -o errexit

error() {
  printf "%s\n" "$*"
  exit 1
}

usage_error() {
  error "Usage: ${0##*/} <number> <number>"
}

result() {
  printf "%s\n" "$1"
  exit 0
}

is_not_float() {
  [[ ! $1 =~ ^[+-]?[0-9]+([.]?[0-9]*)?$ ]]
}

are_not_float() {
  is_not_float "$x" || is_not_float "$y"
}

main() {
  local x="$1"
  local y="$2"
  [[ "$#" != 2 ]] || are_not_float && usage_error

  local one five ten
  {
    read -r one
    read -r five
    read -r ten
  } < <(
    bc -l <<_END_
            x = sqrt($x^2 + $y^2)
            x <= 1
            x <= 5
            x <= 10
_END_
  )

  ((one)) && result 10
  ((five)) && result 5
  ((ten)) && result 1
  result 0
}

main "$@"
