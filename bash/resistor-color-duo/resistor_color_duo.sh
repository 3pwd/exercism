#!/usr/bin/env bash

set -o errexit
set -o nounset
shopt -s nocasematch

main() {
  local -A color_codes=(
    [black]=0
    [brown]=1
    [red]=2
    [orange]=3
    [yellow]=4
    [green]=5
    [blue]=6
    [violet]=7
    [grey]=8
    [white]=9
  )
  local result

  read -ra colors <<<"$@"

  for color in "${colors[@]:0:2}"; do
    if [[ ${color_codes[$color]:+defined} ]]; then
      result+=${color_codes[$color]}
    else
      printf "%s\n" "invalid color" && exit 1
    fi
  done

  printf "%i\n" "${result##0}"
}

main "$@"
