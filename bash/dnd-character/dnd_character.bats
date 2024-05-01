#!/usr/bin/env bats
load bats-extra

# local version: 1.1.0.0

# usage: dnd_character.bash modifier n
# -> output expected modifier

# usage: dnd_character.bash generate
# -> output each characteristic and ability value, one per line


# ability modifier

@test "ability modifier for score 3 is -4" {
    run bash dnd_character.bash modifier 3
    assert_success
    assert_output "-4"
}

@test "ability modifier for score 4 is -3" {
    run bash dnd_character.bash modifier 4
    assert_success
    assert_output "-3"
}

@test "ability modifier for score 5 is -3" {
    run bash dnd_character.bash modifier 5
    assert_success
    assert_output "-3"
}

@test "ability modifier for score 6 is -2" {
    run bash dnd_character.bash modifier 6
    assert_success
    assert_output "-2"
}

@test "ability modifier for score 7 is -2" {
    run bash dnd_character.bash modifier 7
    assert_success
    assert_output "-2"
}

@test "ability modifier for score 8 is -1" {
    run bash dnd_character.bash modifier 8
    assert_success
    assert_output "-1"
}

@test "ability modifier for score 9 is -1" {
    run bash dnd_character.bash modifier 9
    assert_success
    assert_output "-1"
}

@test "ability modifier for score 10 is 0" {
    run bash dnd_character.bash modifier 10
    assert_success
    assert_output "0"
}

@test "ability modifier for score 11 is 0" {
    run bash dnd_character.bash modifier 11
    assert_success
    assert_output "0"
}

@test "ability modifier for score 12 is +1" {
    run bash dnd_character.bash modifier 12
    assert_success
    assert_output "1"
}

@test "ability modifier for score 13 is +1" {
    run bash dnd_character.bash modifier 13
    assert_success
    assert_output "1"
}

@test "ability modifier for score 14 is +2" {
    run bash dnd_character.bash modifier 14
    assert_success
    assert_output "2"
}

@test "ability modifier for score 15 is +2" {
    run bash dnd_character.bash modifier 15
    assert_success
    assert_output "2"
}

@test "ability modifier for score 16 is +3" {
    run bash dnd_character.bash modifier 16
    assert_success
    assert_output "3"
}

@test "ability modifier for score 17 is +3" {
    run bash dnd_character.bash modifier 17
    assert_success
    assert_output "3"
}

@test "ability modifier for score 18 is +4" {
    run bash dnd_character.bash modifier 18
    assert_success
    assert_output "4"
}


# generate a character, validate expected output

@test "generate a character" {
    run bash dnd_character.bash generate
    assert_success
    # these don't have to appear in any particular order
    assert_line --regexp '^strength [[:digit:]]{1,2}$'
    assert_line --regexp '^dexterity [[:digit:]]{1,2}$'
    assert_line --regexp '^constitution [[:digit:]]{1,2}$'
    assert_line --regexp '^intelligence [[:digit:]]{1,2}$'
    assert_line --regexp '^wisdom [[:digit:]]{1,2}$'
    assert_line --regexp '^charisma [[:digit:]]{1,2}$'
    assert_line --regexp '^hitpoints [[:digit:]]{1,2}$'
    # no other output: `run` populates the `lines` array
    assert_equal ${#lines[@]} 7
}


# Usage: between $val $low $high
# Value is between low (inclusive) and high (inclusive).
between() { 
    (( $2 <= $1 && $1 <= $3 ))
}

# random ability is within range
@test "validate ability range and hitpoint value" {
    for i in {1..50}; do
        while read c v; do
            if [[ $c == "hitpoints" ]]; then
                hits=$v
            else
                assert between "$v" 3 18
                [[ $c == "constitution" ]] && const=$v
            fi
        done < <(bash dnd_character.bash generate)

        const_mod=$(bash dnd_character.bash modifier "$const")
        assert_equal $((10 + const_mod)) "$hits"
    done
}
