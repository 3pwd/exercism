#!/usr/bin/env bash

shopt -s nocasematch

main() {
  local -A POINTS=([aeioulnrst]=1 [dg]=2 [bcmp]=3 [fhvwy]=4 [k]=5 [jx]=8 [qz]=10)
  local -i score

  while read -rn 1 letter; do
    for letters_group in "${!POINTS[@]}"; do
      points=${POINTS[$letters_group]}
      [[ "$letters_group" =~ $letter ]] && score+="$points"
    done
  done < <(printf %s "${1^^}")

  printf "%i\n" "$score"

}

main "$@"
