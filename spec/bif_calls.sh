describe "'add_ints' BIF"

expect "add_ints(1, 2)" "3"
expect "add_ints(9*7%3, (1+1))" "2"


describe "'sub_ints' BIF"

expect "sub_ints(1, 2)" "-1"
expect "sub_ints(9*7%3, (1+1))" "-2"