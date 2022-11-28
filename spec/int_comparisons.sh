describe 'Integer GT'

expect_expr_int '1 > 2' '0'
expect_expr_int '2 > 1' '1'
expect_expr_int '1 > 1' '0'
expect_expr_int '1 > 0' '1'
expect_expr_int '0 > 1' '0'
expect_expr_int '1 > -1' '1'
expect_expr_int '-1 > 1' '0'
expect_expr_int '1 + 4 > 2' '1'
expect_expr_int '1 + 4 > 2 * 3' '0'


describe 'Integer LT'

expect_expr_int '1 < 2' '1'
expect_expr_int '2 < 1' '0'
expect_expr_int '1 < 1' '0'
expect_expr_int '1 < 0' '0'
expect_expr_int '0 < 1' '1'
expect_expr_int '1 < -1' '0'
expect_expr_int '-1 < 1' '1'
expect_expr_int '1 + 4 < 2' '0'
expect_expr_int '1 + 4 < 2 * 3' '1'


describe 'Integer GE'

expect_expr_int '1 >= 2' '0'
expect_expr_int '2 >= 1' '1'
expect_expr_int '1 >= 1' '1'
expect_expr_int '1 >= 0' '1'
expect_expr_int '0 >= 1' '0'
expect_expr_int '1 >= -1' '1'
expect_expr_int '-1 >= 1' '0'
expect_expr_int '1 + 4 >= 2' '1'
expect_expr_int '1 + 4 >= 2 * 3' '0'


describe 'Integer LE'

expect_expr_int '1 <= 2' '1'
expect_expr_int '2 <= 1' '0'
expect_expr_int '1 <= 1' '1'
expect_expr_int '1 <= 0' '0'
expect_expr_int '0 <= 1' '1'
expect_expr_int '1 <= -1' '0'
expect_expr_int '-1 <= 1' '1'
expect_expr_int '1 + 4 <= 2' '0'
expect_expr_int '1 + 4 <= 2 * 3' '1'


describe 'Integer EQ'

expect_expr_int '1 == 2' '0'
expect_expr_int '2 == 1' '0'
expect_expr_int '1 == 1' '1'
expect_expr_int '1 == 0' '0'
expect_expr_int '0 == 1' '0'
expect_expr_int '1 == -1' '0'
expect_expr_int '-1 == 1' '0'
expect_expr_int '1 + 4 == 2' '0'
expect_expr_int '1 + 4 == 2 * 3' '0'


describe 'Integer NE'

expect_expr_int '1 != 2' '1'
expect_expr_int '2 != 1' '1'
expect_expr_int '1 != 1' '0'
expect_expr_int '1 != 0' '1'
expect_expr_int '0 != 1' '1'
expect_expr_int '1 != -1' '1'
expect_expr_int '-1 != 1' '1'
expect_expr_int '1 + 4 != 2' '1'
expect_expr_int '1 + 4 != 2 * 3' '1'
