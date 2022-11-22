describe "'add_ints' BIF"

expect_exec "add_ints(1, 2)" "3"
expect_exec "add_ints(9*7%3, (1+1))" "2"


describe "'sub_ints' BIF"

expect_exec "sub_ints(1, 2)" "-1"
expect_exec "sub_ints(9*7%3, (1+1))" "-2"