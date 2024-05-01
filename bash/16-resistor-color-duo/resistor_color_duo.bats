#!/usr/bin/env bats
load bats-extra

# local version: 2.1.0.1
# add test for invalid color

@test "brown black" {
    run bash resistor_color_duo.bash brown black
    assert_success
    assert_output "10"
}

@test "blue grey" {
    run bash resistor_color_duo.bash blue grey
    assert_success
    assert_output "68"
}

@test "yellow violet" {
    run bash resistor_color_duo.bash yellow violet
    assert_success
    assert_output "47"
}

@test "white red" {
    run bash resistor_color_duo.bash white red
    assert_success
    assert_output "92"
}

@test "orange orange" {
    run bash resistor_color_duo.bash orange orange
    assert_success
    assert_output "33"
}

@test "invalid color" {
    run bash resistor_color_duo.bash foo
    assert_failure
    assert_output --partial "invalid color"
}

@test "one valid color and one invalid color" {
    run bash resistor_color_duo.bash blue foo
    assert_failure
    assert_output --partial "invalid color"
}

@test "one invalid color and one valid color" {
    run bash resistor_color_duo.bash foo blue
    assert_failure
    assert_output --partial "invalid color"
}

@test "ignore too many colors" {
    run bash resistor_color_duo.bash green brown orange
    assert_success
    assert_output "51"
}

@test "black brown" {
    run bash resistor_color_duo.bash black brown
    assert_success
    assert_output "1"
}

@test "black grey" {
    run bash resistor_color_duo.bash black grey
    assert_success
    assert_output "8"
}
