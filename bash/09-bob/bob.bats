#!/usr/bin/env bats
load bats-extra

# local version: 1.6.0.0

@test "stating something" {
  #[[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash bob.bash 'Tom-ay-to, tom-aaaah-to.'
  assert_success
  assert_output "Whatever."
}

@test "shouting" {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash bob.bash 'WATCH OUT!'
  assert_success
  assert_output "Whoa, chill out!"
}

@test "shouting gibberish" {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash bob.bash 'FCECDFCAAB'
  assert_success
  assert_output "Whoa, chill out!"
}

@test "asking a question" {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash bob.bash 'Does this cryogenic chamber make me look fat?'
  assert_success
  assert_output "Sure."
}

@test "asking a numeric question" {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash bob.bash 'You are, what, like 15?'
  assert_success
  assert_output "Sure."
}

@test "asking gibberish" {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash bob.bash 'fffbbcbeab?'
  assert_success
  assert_output "Sure."
}

@test "talking forcefully" {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash bob.bash "Hi there!"
  assert_success
  assert_output "Whatever."
}

@test "using acronyms in regular speech" {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash bob.bash "It's OK if you don't want to go work for NASA."
  assert_success
  assert_output "Whatever."
}

@test "forceful question" {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash bob.bash "WHAT'S GOING ON?"
  assert_success
  assert_output "Calm down, I know what I'm doing!"
}

@test "shouting numbers" {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash bob.bash '1, 2, 3 GO!'
  assert_success
  assert_output "Whoa, chill out!"
}

@test "no letters" {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash bob.bash '1, 2, 3'
  assert_success
  assert_output "Whatever."
}

@test "question with no letters" {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash bob.bash '4?'
  assert_success
  assert_output "Sure."
}

@test "shouting with special characters" {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash bob.bash 'ZOMG THE %^*@#$(*^ ZOMBIES ARE COMING!!11!!1!'
  assert_success
  assert_output "Whoa, chill out!"
}

@test "shouting with no exclamation mark" {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash bob.bash 'I HATE THE DENTIST'
  assert_success
  assert_output "Whoa, chill out!"
}

@test "statement containing question mark" {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash bob.bash 'Ending with ? means a question.'
  assert_success
  assert_output "Whatever."
}

@test "non-letters with question" {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash bob.bash ':) ?'
  assert_success
  assert_output "Sure."
}

@test "prattling on" {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash bob.bash 'Wait! Hang on. Are you going to be OK?'
  assert_success
  assert_output "Sure."
}

@test "silence" {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash bob.bash ''
  assert_success
  assert_output "Fine. Be that way!"
}

@test "prolonged silence" {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash bob.bash '          '
  assert_success
  assert_output "Fine. Be that way!"
}

@test "alternate silence" {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash bob.bash $'\t\t\t\t\t\t\t\t\t\t'
  assert_success
  assert_output "Fine. Be that way!"
}

@test "multiple line question" {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash bob.bash $'\nDoes this cryogenic chamber make me look fat?\nNo'
  assert_success
  assert_output "Whatever."
}

@test "starting with whitespace" {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash bob.bash '         hmmmmmmm...'
  assert_success
  assert_output "Whatever."
}

@test "ending with whitespace" {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash bob.bash 'Okay if like my  spacebar  quite a bit?   '
  assert_success
  assert_output "Sure."
}
# This test might act differently depending on your platform
@test "other whitespace" {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash bob.bash $'\n\r \t'
  assert_success
  assert_output "Fine. Be that way!"
}

@test "non-question ending with whitespace" {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash bob.bash 'This is a statement ending with whitespace      '
  assert_success
  assert_output "Whatever."
}

@test "no input is silence" {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash bob.bash
  assert_success
  assert_output "Fine. Be that way!"
}

# bash-specific tests
@test "yelling a filename expansion" {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash bob.bash '*READ* !'
  assert_success
  assert_output "Whoa, chill out!"
}

@test "asking a filename expansion" {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash bob.bash 'bob???'
  assert_success
  assert_output "Sure."
}
