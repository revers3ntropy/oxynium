describe "Arithmetic"

expect "1" "1"
expect "1+1" "2"
expect "1+3" "4"
expect "1+1+1+1+1" "5"
expect "   1 + 1 + 1 + 1 + 3+ 2  " "9"
#expect "58600+1" "58601"

expect "2-1" "1"
expect "1-1" "0"
#expect "1-3" "-2"
#expect "1-1-1-1-1" "-3"
#expect "58600-1" "58599"

#expect "1*1" "1"
#expect "1*3" "3"
#expect "1*1*1*1*1" "1"
#expect "58600*1" "58600"
#expect "3*8" "24"

#expect "1/1" "1"
#expect "1/3" "0"
#expect "1/1/1/1/1" "1"
#expect "58600/1" "58600"
#expect "24/8" "3"

#expect "1%1" "0"
#expect "1%3" "1"
#expect "1%1%1%1%1" "0"
#expect "58600%1" "0"
#expect "24%8" "0"

#expect "1^1" "1"
#expect "1^3" "1"
#expect "1^1^1^1^1" "1"
#expect "58600^1" "58600"
#expect "24^8" "16777216"

#expect "+0" "0"
#expect "+1" "1"
#expect "+3" "3"
#expect "+1+1+1+1+1" "5"

#expect "-0" "0"
#expect "-1" "-1"
#expect "-3" "-3"
#expect "-1-1-1-1-1" "-5"

#expect "1+2*3" "7"
#expect "1*2+3" "5"
#expect "1+2*3+4" "11"
#expect "1*2+3*4" "14"
#expect "1+2*3+4*5" "31"
#expect "1*2-3*4/5" "-1"
#expect "1+2*3-4*5/6" "0"
#expect "-1*2+3*4/5-100" "-100"

#expect "(1+2)" "3"
#expect "(1+2)*3" "9"
#expect "1+(2*3)" "7"
#expect "1+(2*3)+4" "11"
#expect "1+(2*3)+(4*5)" "31"