#!/bin/sh

function how_old() {
    local x=$(($1))
    local y=$(($2))
    local result=$(wasmtime hello.wat --invoke how_old $x $y 2>/dev/null)
    echo "$result"
}

for num in "2021 2000" "2021 1980" "2021 1960"; do
    set -- $num
    echo "how_old($1, $2) = $(how_old "$1" "$2")"
done
