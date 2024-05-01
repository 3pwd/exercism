#!/usr/bin/env bats
load bats-extra

# local version: 2.0.0.0

# Check if the given string is a pangram

@test "empty sentence" {
  #[[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash pangram.bash ""
  assert_success
  assert_output "false"
}

@test "perfect lower case" {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash pangram.bash "abcdefghijklmnopqrstuvwxyz"
  assert_success
  assert_output "true"
}

@test "only lower case" {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash pangram.bash "the quick brown fox jumps over the lazy dog"
  assert_success
  assert_output "true"
}

@test "missing the letter 'x'" {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash pangram.bash "a quick movement of the enemy will jeopardize five gunboats"
  assert_success
  assert_output "false"
}

@test "missing the letter 'h'" {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash pangram.bash "five boxing wizards jump quickly at it"
  assert_success
  assert_output "false"
}

@test "with underscores" {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash pangram.bash "the_quick_brown_fox_jumps_over_the_lazy_dog"
  assert_success
  assert_output "true"
}

@test "with numbers" {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash pangram.bash "the 1 quick brown fox jumps over the 2 lazy dogs"
  assert_success
  assert_output "true"
}

@test "missing letters replaced by numbers" {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash pangram.bash "7h3 qu1ck brown fox jumps ov3r 7h3 lazy dog"
  assert_success
  assert_output "false"
}

@test "mixed case and punctuation" {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash pangram.bash "\"Five quacking Zephyrs jolt my wax bed.\""
  assert_success
  assert_output "true"
}

@test "a-m and A-M are 26 different characters but not a pangram" {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash pangram.bash "abcdefghijklm ABCDEFGHIJKLM"
  assert_success
  assert_output "false"
}
