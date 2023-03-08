#!/usr/bin/env bash

set -o errexit
set -o nounset

main () {
  [[ $# -ne 1 ]] && exit 1

  local -i n=$1
  local -A factors=([3]=Pling [5]=Plang [7]=Plong)
  local result=''

  for factor in "${!factors[@]}";do
    (( n % factor )) || result="${factors[$factor]}$result"
  done

  echo "${result:-$n}"

  return 0
}

main "$@"

