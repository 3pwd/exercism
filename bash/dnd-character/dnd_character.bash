#!/usr/bin/env bash

set -o errexit
set -o nounset

error() {
  printf "%s\n" "$@"
  exit 1
}

usage_error() {
  error "Usage: ${0##*/} $1"
}

random_ability_value() {
  local -i value=0 min=6 roll

  for _ in {1..4}; do
    roll=$(((RANDOM % 6) + 1))
    ((min = (roll < min) ? roll : min, value += roll))
  done

  printf "%i\n" "$((value - min))"
}

modifier() {
  [[ $# == 1 && $1 =~ [[:digit:]] ]] || usage_error "modifier <number>"
  printf "%i\n" "$(($1 / 2 - 5))"
}

calc_hit_points() {
  printf "%i\n" "$((10 + $(modifier "$constitution")))"
}

generate() {
  local -i constitution
  local -i hit_points
  constitution=$(random_ability_value)
  hit_points=$(calc_hit_points)

  for ability in strength dexterity constitution intelligence wisdom charisma; do
    printf "%s " "$ability"
    [[ $ability == "constitution" ]] && printf "%i\n" "$constitution" || printf "%i\n" "$(random_ability_value)"
  done
  printf "%s %i\n" "hitpoints" "$hit_points"
}

main() {
  [[ -n $# && $# -lt 3 && $1 =~ ^generate|modifier$ ]] || usage_error "<operation> <number?>
  operation: generate|modifier
  number: score, required if operation is 'modifier'"

  "$@"
}

main "$@"
