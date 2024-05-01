#!/usr/bin/env bats
load bats-extra

# local version: 1.1.0.0

@test 'Zero is Armstrong numbers' {
  # [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash armstrong_numbers.bash 0
  assert_success
  assert_output "true"
}

@test 'Single digits are Armstrong numbers' {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash armstrong_numbers.bash 5
  assert_success
  assert_output "true"
}

@test 'There are no two digit Armstrong numbers' {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash armstrong_numbers.bash 10
  assert_success
  assert_output "false"
}

@test 'A three digit number that is an Armstrong number' {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash armstrong_numbers.bash 153
  assert_success
  assert_output "true"
}

@test 'A three digit number that is not an Armstrong number' {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash armstrong_numbers.bash 100
  assert_success
  assert_output "false"
}

@test 'A four digit number that is an Armstrong number' {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash armstrong_numbers.bash 9474
  assert_success
  assert_output "true"
}

@test 'A four digit number that is not an Armstrong number' {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash armstrong_numbers.bash 9475
  assert_success
  assert_output "false"
}

@test 'A seven digit number that is an Armstrong number' {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash armstrong_numbers.bash 9926315
  assert_success
  assert_output "true"
}

@test 'A seven digit number that is not an Armstrong number' {
  [[ $BATS_RUN_SKIPPED == "true" ]] || skip
  run bash armstrong_numbers.bash 9926314
  assert_success
  assert_output "false"
}
