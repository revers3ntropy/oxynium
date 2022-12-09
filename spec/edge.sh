describe 'Edge Cases'

expect '' ''
expect '' ' '
expect '' '


 '

expect_err 'SyntaxError' ';'
expect_err 'SyntaxError' ';;'
expect '' ';1'
expect '' ';1;'
expect '' ';;;;1;;;;;'
expect '' '1;;;;;;'
expect '' '1;'
expect '' '1'

# weird EOF cases
expect_err 'UnknownSymbolError' 'mut'
expect_err 'SyntaxError' 'const'
expect_err 'SyntaxError' 'var'
expect_err 'SyntaxError' 'let'
expect_err 'SyntaxError' 'let mut'
expect_err 'SyntaxError' 'fn'
expect_err 'SyntaxError' 'extern'
expect_err 'SyntaxError' 'extern fn'
expect_err 'SyntaxError' 'extern var'
expect_err 'SyntaxError' 'extern const a'
expect_err 'SyntaxError' 'extern const a:'
expect_err 'SyntaxError' 'extern const a:'
