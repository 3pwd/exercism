#!/usr/bin/env bash

error() {
  printf "%s\n" "$*"
  exit 1
}

# https://mywiki.wooledge.org/BashGuide/Parameters#Parameter_Expansion
usage_error() {
  error "Usage: ${0##*/} <string1> <string2>"
}

length_error() {
  error "strands must be of equal length"
}

main() {
  [[ $# == 2 ]] || usage_error

  readonly strand1=$1
  readonly strand2=$2
  local -i hamming=0

  [[ "${#strand1}" == "${#strand2}" ]] || length_error

  for ((i = 0; i <= "${#strand1}"; i++)); do
    [[ "${strand1:$i:1}" == "${strand2:$i:1}" ]] || hamming+=1
  done

  printf "%d\n" "$hamming"
}

main "$@"
