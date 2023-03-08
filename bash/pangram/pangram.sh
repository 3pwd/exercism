#!/usr/bin/env bash

set -o errexit
set -o nounset

success() {
  printf "%s\n" "$1"
  exit 0
}

get_alphabet() {
  echo {a..z} | tr -d ' '
}

main() {
  [[ -z $1 ]] && success false

  local letters="${1,,}"

  alphabet=$(get_alphabet)
  if [[ -z ${alphabet//[$letters]/} ]]; then
    success true
  else
    success false
  fi
}

main "$@"