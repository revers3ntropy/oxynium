describe 'Edge Cases'

expect '' ''
expect ' ' ''
expect '


 ' ''

expect_err ';' 'SyntaxError'
expect_err ';;' 'SyntaxError'
expect ';1' ''
expect ';1;' ''
expect ';;;;1;;;;;' ''
expect '1;;;;;;' ''
expect '1;' ''
expect '1' ''

expect_expr_int '9223372036854775806' '9223372036854775806' # 2^(64-1)-2, max int size
expect_expr_int '9223372036854775807' '9223372036854775807' # 2^(64-1)-1, max int size
expect_err '9223372036854775808' 'NumericOverflow' # 2^63, too big for int