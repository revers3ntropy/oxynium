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