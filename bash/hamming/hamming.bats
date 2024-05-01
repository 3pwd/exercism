#!/usr/bin/env bats
load bats-extra

# local version: 2.3.0.3
# 
# bash-specific test: Input validation, proper quoting

@test 'empty strands' {
  run bash hamming.bash '' ''
  assert_success
  assert_output "0"
}

@test 'single letter identical strands' {
  run bash hamming.bash 'A' 'A'
  assert_success
  assert_output "0"
}

@test 'single letter different strands' {
  run bash hamming.bash 'G' 'T'
  assert_success
  assert_output "1"
}

@test 'long identical strands' {
  run bash hamming.bash 'GGACTGAAATCTG' 'GGACTGAAATCTG'
  assert_success
  assert_output "0"
}

@test 'long different strands' {
  run bash hamming.bash 'GGACGGATTCTG' 'AGGACGGATTCT'
  assert_success
  assert_output "9"
}

@test 'disallow first strand longer' {
  run bash hamming.bash 'AATG' 'AAA'
  assert_failure
  assert_output --partial "strands must be of equal length"
}

@test 'disallow second strand longer' {
  run bash hamming.bash 'ATA' 'AGTG'
  assert_failure
  assert_output --partial "strands must be of equal length"
}

@test 'disallow left empty strand' {
  run bash hamming.bash '' 'G'
  assert_failure
  assert_output --partial "strands must be of equal length"
}

@test 'disallow right empty strand' {
  run bash hamming.bash 'G' ''
  assert_failure
  assert_output --partial "strands must be of equal length"
}

@test "no input" {
  run bash hamming.bash
  assert_failure
  assert_output "Usage: hamming.bash <string1> <string2>"
}

@test "invalid input" {
  run bash hamming.bash 'A'
  assert_failure
  assert_output "Usage: hamming.bash <string1> <string2>"
}

# Within [[...]] the == operator is a _pattern matching_ operator.
# To test for string equality, the right-hand side must be
# quoted to prevent interpretation as a glob-style pattern.

@test "expose subtle '[[ \$x == \$y ]]' bug" {
  run bash hamming.bash 'AAA' 'A?A'
  assert_success
  assert_output "1"
}
