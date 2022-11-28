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

expect_expr_int '9223372036854775807' '9223372036854775807' # 2^63-1, max int size
expect_err '9223372036854775808' 'NumericOverflow' # 2^63, too big for int