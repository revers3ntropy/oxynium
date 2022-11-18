describe "Arithmetic"

expect "1" "1"
expect "1+1" "2"
expect "1+3" "4"
expect "1+1+1+1+1" "5"
expect "58600+1" "58601"

expect "1-1" "0"
expect "1-3" "-2"
expect "1-1-1-1-1" "-3"
expect "58600-1" "58599"

expect "1*1" "1"
expect "1*3" "3"
expect "1*1*1*1*1" "1"
expect "58600*1" "58600"
expect "3*8" "24"

expect "1/1" "1"
expect "1/3" "0"
expect "1/1/1/1/1" "1"
expect "58600/1" "58600"
expect "24/8" "3"