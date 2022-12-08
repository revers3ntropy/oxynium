describe 'Arithmetic'

expect_expr_int '1' '1'
expect_expr_int '2' '1+1'
expect_expr_int '4' '1+3'
expect_expr_int '5' '1+1+1+1+1'
expect_expr_int '9' '   1 + 1 + 1 + 1 + 3+ 2  '
expect_expr_int '58601' '58600+1'

expect_expr_int '1' '2-1'
expect_expr_int '0' '1-1'
expect_expr_int '-2' '1-3'
expect_expr_int '-3' '1-1-1-1-1'
expect_expr_int '58599' '58600-1'

expect_expr_int '1' '1*1'
expect_expr_int '3' '1*3'
expect_expr_int '1' '1*1*1*1*1'
expect_expr_int '58600' '58600*1'
expect_expr_int '24' '3*8'

expect_expr_int '1' '1/1'
expect_expr_int '0' '1/3'
expect_expr_int '1' '1/1/1/1/1'
expect_expr_int '58600' '58600/1'
expect_expr_int '3' '24/8'

expect_expr_int '0' '1%1'
expect_expr_int '1' '1%3'
expect_expr_int '1' '3%2'
expect_expr_int '0' '1%1%1%1%1'
expect_expr_int '0' '58600%1'
expect_expr_int '0' '24%8'
expect_expr_int '2' '3%2*2'
expect_expr_int '30033' '30033%876542'
expect_expr_int '3' '3%(2*2)'
expect_expr_int '7' '7/8+7%8'

expect_expr_int '0' '(-0)'
expect_expr_int '-1' '(-1)'
expect_expr_int '-3' '(-3)'
expect_expr_int '-5' '(-1-1-1-1-1)'

expect_expr_int '7' '1+2*3'
expect_expr_int '5' '1*2+3'
expect_expr_int '11' '1+2*3+4'
expect_expr_int '14' '1*2+3*4'
expect_expr_int '27' '1+2*3+4*5'
expect_expr_int '-3' '1+2*3-4*5/2'
expect_expr_int '-96' '0+-1*2+3*-8/-4-100'

expect_expr_int '3' '(1+2)'
expect_expr_int '9' '(1+2)*3'
expect_expr_int '7' '1+(2*3)'
expect_expr_int '11' '1+(2*3)+4'
expect_expr_int '27' '1+((2*3)+4*5)'

expect_err 'SyntaxError' '1+'
expect_err 'SyntaxError' '1 1'
expect_err 'SyntaxError' '1 1-'
expect_err 'SyntaxError' '1 -1-'
expect_err 'SyntaxError' '1 -1-'
expect_err 'SyntaxError' '1 */ 2'
expect_err 'SyntaxError' '1 * - / 2'
expect_err 'SyntaxError' '1 = 2'
expect_err 'SyntaxError' '1 + 2 = 3'
expect_err 'SyntaxError' '*'
expect_err 'SyntaxError' '/'
expect_err 'SyntaxError' '+'
expect_err 'SyntaxError' '=='
expect_err 'SyntaxError' '*1'
expect_err 'SyntaxError' '+1'
expect_err 'SyntaxError' '+-2'
expect_err 'SyntaxError' '('
expect_err 'SyntaxError' '()'
expect_err 'SyntaxError' '(1()'
expect_err 'SyntaxError' '(2+2'
expect_err 'SyntaxError' '((2+2)'
expect_err 'SyntaxError' '((2+(2*6))'
expect_err 'SyntaxError' '((2+(2*6)'


describe 'Int Overflow'

expect_expr_int '9223372036854775806' '9223372036854775806' # 2^(64-1)-2, max int size
expect_expr_int '9223372036854775807' '9223372036854775807' # 2^(64-1)-1, max int size
expect_err 'NumericOverflow' '9223372036854775808' # 2^63, too big for int
expect_expr_int '0' '9223372036854775807+1' # overflow to -0
expect_expr_int '-9223372036854775807' '9223372036854775807+2' # overflow to negative max
expect_expr_int '-9223372036854775806' '9223372036854775807+3' # overflow to negative max